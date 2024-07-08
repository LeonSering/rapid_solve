//! [`TakeAnyRecursion`] searches in parallel for an improving neighbor and takes the first
//! one that is found. If no improving neighbor is found, it takes the best solutions found to
//! recursion.
use super::super::ParallelNeighborhood;
use super::ParallelLocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::{Objective, ObjectiveValue};
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

/// Searches in parallel for an improving neighbor. The first one that is found by
/// any thread is taken. If no improving neighbor is found, the best solutions found are taken to
/// recursion.
/// * uses parallel computation at two steps:
///   - In the recursion when multiple solutions are given, each solution get its own thread.
///   - Within each thread the neighborhood is given as [`ParallelIterator`] from the
///   [`ParallelNeighborhood`].
/// * As soon as an improving solution is found a terminus-signal is broadcast to all other threads.
/// * If no improving solution is found the best `recursion_width`-many solutions per thread (!) are
/// taken to recursion (dublicates according to the objective value are removed).
/// * Is can be fast if the computation or evaluation of a neighbor is CPU-heavy and the [`ParallelNeighborhood`]
/// is large.
/// * Produces quite a bit of overhead.
/// * Is not deterministic.
/// * The diversification for recursion is probably low.
pub struct TakeAnyRecursion<S, N> {
    recursion_depth: u8,
    recursion_width: u8,
    neighborhood: Arc<N>,
    objective: Arc<Objective<S>>,
}

impl<S, N> TakeAnyRecursion<S, N> {
    /// Creates a new instance of [`TakeAnyRecursion`]. In addition to the [`ParallelNeighborhood`]
    /// and the [`Objective`] the following parameters are needed:
    /// * `recursion_depth` is the number of recursions to be done.
    /// * `recursion_width` is the number of solutions to be taken to recursion.
    pub fn new(
        recursion_depth: u8,
        recursion_width: u8,
        neighborhood: Arc<N>,
        objective: Arc<Objective<S>>,
    ) -> TakeAnyRecursion<S, N> {
        TakeAnyRecursion {
            recursion_depth,
            recursion_width,
            neighborhood,
            objective,
        }
    }
}

impl<S: Send + Sync + Clone, N: ParallelNeighborhood<S>> ParallelLocalImprover<S>
    for TakeAnyRecursion<S, N>
{
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        let old_objective = solution.objective_value();
        self.improve_recursion(vec![solution.clone()], old_objective, self.recursion_depth)
    }
}

impl<S: Send + Sync + Clone, N: ParallelNeighborhood<S>> TakeAnyRecursion<S, N> {
    fn improve_recursion(
        &self,
        solutions: Vec<EvaluatedSolution<S>>,
        objective_to_beat: &ObjectiveValue,
        remaining_recursion: u8,
    ) -> Option<EvaluatedSolution<S>> {
        let mut solution_collection: Vec<Vec<EvaluatedSolution<S>>> = Vec::new();
        let mut result: Option<EvaluatedSolution<S>> = None;
        rayon::scope(|s| {
            let mut found_senders = Vec::new();
            let (success_sender, success_receiver) = channel();
            let (failure_sender, failure_receiver) = channel();

            for sol in solutions.iter() {
                let (found_sender, found_receiver) = channel();
                found_senders.push(found_sender);

                let succ_sender = success_sender.clone();
                let fail_sender = failure_sender.clone();
                s.spawn(move |_| {
                    let found_receiver_mutex = Arc::new(Mutex::new(found_receiver));

                    let mut new_solutions: Vec<EvaluatedSolution<S>> = Vec::new();
                    let new_solutions_mutex: Arc<Mutex<&mut Vec<EvaluatedSolution<S>>>> =
                        Arc::new(Mutex::new(&mut new_solutions));

                    let result = self
                        .neighborhood
                        .neighbors_of(sol.solution())
                        .map(|neighbor| self.objective.evaluate(neighbor))
                        .find_any(|evaluated_neighbor| {
                            if remaining_recursion > 0 {
                                let mut schedules_mutex = new_solutions_mutex.lock().unwrap();

                                schedules_mutex.push(evaluated_neighbor.clone());

                                schedules_mutex.sort_unstable_by(|a, b| {
                                    a.objective_value().cmp(b.objective_value())
                                });
                                schedules_mutex.dedup_by(|s1, s2| {
                                    s1.objective_value().cmp(s2.objective_value()).is_eq()
                                }); //remove dublicates according to objective_value
                                let width =
                                    (self.recursion_width as usize).min(schedules_mutex.len());
                                schedules_mutex.truncate(width);
                            }

                            let found_receiver_mutex = found_receiver_mutex.lock().unwrap();
                            let found = found_receiver_mutex.try_recv();
                            evaluated_neighbor
                                .objective_value()
                                .cmp(objective_to_beat)
                                .is_lt()
                                || found.is_ok()
                        });

                    match result {
                        Some(sol) => {
                            if sol.objective_value() < objective_to_beat {
                                succ_sender.send(sol).unwrap();
                            }
                            // if there is a Some result but the objective is not better, that means
                            // another thread was successful first. So there is nothing
                            // left to do for this thread.
                        }
                        None => {
                            fail_sender.send(new_solutions).unwrap();
                        }
                    }
                });
            }

            drop(success_sender);
            drop(failure_sender);

            while let Ok(new_sol_pair) = success_receiver.recv() {
                for s in found_senders.iter() {
                    s.send(true).ok();
                }
                if result.is_none()
                    || new_sol_pair.objective_value() < result.as_ref().unwrap().objective_value()
                {
                    result = Some(new_sol_pair);
                }
            }
            if result.is_none() {
                for v in failure_receiver.into_iter() {
                    solution_collection.push(v);
                }
            }
        });

        if result.is_none() {
            if remaining_recursion > 0 {
                let mut schedules_for_recursion: Vec<EvaluatedSolution<S>> =
                    solution_collection.into_iter().flatten().collect();

                schedules_for_recursion
                    .sort_unstable_by(|a, b| a.objective_value().cmp(b.objective_value()));
                schedules_for_recursion.dedup_by(|a, b| a.objective_value() == b.objective_value());

                self.improve_recursion(
                    schedules_for_recursion,
                    objective_to_beat,
                    remaining_recursion - 1,
                )
            } else {
                None
            }
        } else {
            result
        }
    }
}
