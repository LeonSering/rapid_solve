use super::super::Neighborhood;
use super::LocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::{Objective, ObjectiveValue};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
/// This improver uses parallel computation at two steps. In the recursion when multiple solutions
/// are given, each solution get its own thread. Within each thread the neighborhood iterator is tranformed
/// to a ParallelIterator (messes up the ordering) and search for ANY improving solution in
/// parallel.
/// As soon as an improving soltion is found a terminus-signal is broadcast to all other solutions.
/// If no improving solution is found the width-many solutions of each thread are take to recursion
/// (dublicates are removed)
/// Due to the parallel computation and find_any() this improver is the fastest but not
/// deterministic.
pub struct TakeAnyParallelRecursion<S> {
    recursion_depth: u8,
    recursion_width: Option<usize>, // number of schedule that are considered per schedule for the next recursion (the one with best objectivevalue are taken for each schedule, dublicates are removed)
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
}

impl<S: Send + Sync + Clone + Ord> LocalImprover<S> for TakeAnyParallelRecursion<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        let old_objective = solution.objective_value();
        self.improve_recursion(vec![solution.clone()], old_objective, self.recursion_depth)
    }
}

impl<S: Send + Sync + Clone + Ord> TakeAnyParallelRecursion<S> {
    pub fn new(
        recursion_depth: u8,
        recursion_width: Option<usize>,
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> TakeAnyParallelRecursion<S> {
        TakeAnyParallelRecursion {
            recursion_depth,
            recursion_width,
            neighborhood,
            objective,
        }
    }

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
                        .par_bridge()
                        .map(|neighbor| self.objective.evaluate(neighbor))
                        .find_any(|evaluated_neighbor| {
                            if remaining_recursion > 0 {
                                let mut schedules_mutex = new_solutions_mutex.lock().unwrap();

                                schedules_mutex.push(evaluated_neighbor.clone());

                                // if there is a recursion_width truncate schedules to the best width many
                                if let Some(width) = self.recursion_width {
                                    schedules_mutex.sort();
                                    // schedules_mutex.dedup(); //remove dublicates
                                    schedules_mutex.dedup_by(|s1, s2| {
                                        s1.objective_value().cmp(s2.objective_value()).is_eq()
                                    }); //remove dublicates according to objective_value
                                    let width = width.min(schedules_mutex.len());
                                    schedules_mutex.truncate(width);
                                }
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

                schedules_for_recursion.sort();
                schedules_for_recursion.dedup_by(|s1, s2| s1.cmp(&s2).is_eq());

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
