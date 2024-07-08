//! This module contains the [`ParallelTabuSearchSolver`] implementing the
//! [tabu search metaheuristic](https://en.wikipedia.org/wiki/Tabu_search), where the neighborhood
//! exploration is done in parallel.
//! * This solver requires a [`ParallelTabuNeighborhood`], which, in comparison to a regular
//! [`Neighborhood`][crate::heuristics::common::Neighborhood],
//! requires a tabu list as an additional argument and returns
//! a [`ParallelIterator`] (from the [`rayon`] crate) over the neighbors of the solution together with a list of tabus that
//! should be added to the tabu list.
//! * Starts with an initial solution and explores the neighborhood of the current
//! solution in parallel, while ignoring tabu solutions.
//! * The best non-tabu neighbor, even if it is worse than the current solution, is chosen.
//! * Each neighbor is paired with a list of tabus that should be added to the tabu list.
//! * A good tabu should forbid to return to the previous solution.
//! * The list of tabus is limited in size, and the oldest tabus are removed when the list is full.
//! * The search stops after a certain number of iterations, after a certain time limit, or if no
//! global improvement is found after a certain number of iterations.
//! * The best solution  seen is returned.
//!
//! For examples, see the [tabu search solver][crate::examples::tsp::solvers::tabu_search] for the TSP.
pub mod parallel_tabu_improver;

use rayon::iter::ParallelIterator;

use self::parallel_tabu_improver::{ParallelTabuImprover, ParallelTabuMinimizer};

use super::common::{default_function_between_steps, FunctionBetweenSteps};
use super::Solver;
use crate::objective::{EvaluatedSolution, Objective};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time as stdtime;

/// Defines a neighborhood for a tabu search. Compared to a regular neighborhood, a tabu
/// neighborhood takes a tabu list as an additional argument and returns
/// a [`ParallelIterator`] (from the [`rayon`] crate) over the neighbors of the solution together with
/// a list of tabus that should be added to the tabu list.
pub trait ParallelTabuNeighborhood<S: Send, T: Send>: Send + Sync {
    /// TODO
    fn neighbors_of<'a>(
        &'a self,
        solution: &'a S,
        tabu_list: &'a VecDeque<T>,
    ) -> impl ParallelIterator<Item = (S, Vec<T>)> + 'a;
}

/// A tabu search solver that uses a [`ParallelTabuNeighborhood`], an [`Objective`], a tabu list size, as
/// well as a termination criterion to find a good solution.
/// * The `function_between_steps` is executed after each improvement step.
/// * The default `function_between_steps` (if `None`) is printing the iteration number, the
/// objective value (in comparison the the previous objective value) and the time elapsed since the
/// start.
/// * The termination criterion can be either the maximal number of iterations without global
/// improvement, a time limit, or a maximal number of iterations. (One of them must be set.)
///
/// For a high-level overview, see the [module documentation][super::parallel_tabu_search] and for examples,
/// see the [parallel tabu search solver][crate::examples::tsp::solvers::parallel_tabu_search] for the
/// TSP.
pub struct ParallelTabuSearchSolver<S, T> {
    objective: Arc<Objective<S>>,
    tabu_list_size: usize,
    local_improver: Box<dyn ParallelTabuImprover<S, T>>,
    function_between_steps: FunctionBetweenSteps<S>,
    iteration_without_global_improvement_limit: Option<u32>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S: 'static + Send + Sync, T: 'static + Send + Sync> ParallelTabuSearchSolver<S, T> {
    /// Creates a new [`ParallelTabuSearchSolver`] with the given [`ParallelTabuNeighborhood`], [`Objective`], tabu
    /// list size, and as a termination criterion the maximal number of iterations without global
    /// improvement.
    pub fn initialize(
        neighborhood: Arc<impl ParallelTabuNeighborhood<S, T> + 'static>,
        objective: Arc<Objective<S>>,
        tabu_list_size: usize,
        iteration_without_global_improvement_limit: u32,
    ) -> Self {
        Self::with_options(
            neighborhood,
            objective,
            tabu_list_size,
            None,
            None,
            Some(iteration_without_global_improvement_limit),
            None,
            None,
        )
    }

    /// Creates a new [`ParallelTabuSearchSolver`] with the given [`ParallelTabuNeighborhood`], [`Objective`], tabu
    /// list size.
    /// * `function_between_steps` is executed after each improvement step. If `None`, the default
    /// is printing the iteration number, the objective value (in comparison the the previous
    /// objective value) and the time elapsed since the start.
    /// * `iteration_without_global_improvement_limit` is the maximum number of iterations allowed
    /// without global improvement. If `None`, there is no limit.
    /// * `time_limit` is the maximum time allowed for the local search to start a new iteration.
    /// The last iteration is allowed to finish. If `None`, there is no time limit.
    /// * `iteration_limit` is the maximum number of iterations allowed for the local search. If
    /// `None`, there is no iteration limit.
    /// * At least one of `iteration_without_global_improvement_limit`, `time_limit` or
    /// `iteration_limit` must be set.
    /// * If multiple termination criteria are set, the search stops when any of them is reached.
    #[allow(clippy::too_many_arguments)]
    pub fn with_options(
        neighborhood: Arc<impl ParallelTabuNeighborhood<S, T> + 'static>,
        objective: Arc<Objective<S>>,
        tabu_list_size: usize,
        local_improver: Option<Box<dyn ParallelTabuImprover<S, T>>>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        iteration_without_global_improvement_limit: Option<u32>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        if iteration_without_global_improvement_limit.is_none()
            && time_limit.is_none()
            && iteration_limit.is_none()
        {
            panic!("At least one of `iteration_without_global_improvement_limit`, `time_limit` or `iteration_limit` must be set.");
        }

        let local_improver = match local_improver {
            Some(local_improver) => local_improver,
            None => Box::new(ParallelTabuMinimizer::new(neighborhood, objective.clone()))
                as Box<dyn ParallelTabuImprover<S, T>>,
        };
        Self {
            objective,
            tabu_list_size,
            local_improver,
            function_between_steps: function_between_steps
                .unwrap_or(default_function_between_steps()),
            iteration_without_global_improvement_limit,
            time_limit,
            iteration_limit,
        }
    }
}

impl<S: Clone, T: std::fmt::Debug> Solver<S> for ParallelTabuSearchSolver<S, T> {
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();

        let mut current_solution = self.objective.evaluate(initial_solution);
        let mut best_solution_seen = current_solution.clone();
        let mut tabu_list = VecDeque::with_capacity(self.tabu_list_size);
        let mut iteration_counter = 1;
        let mut iteration_without_global_improvement = 0;
        while let Some((new_solution, new_tabus)) =
            self.local_improver.improve(&current_solution, &tabu_list)
        {
            tabu_list.extend(new_tabus.into_iter());
            while tabu_list.len() > self.tabu_list_size {
                tabu_list.pop_front();
            }
            (self.function_between_steps)(
                iteration_counter,
                &new_solution,
                Some(&current_solution),
                self.objective.clone(),
                Some(start_time),
                self.time_limit,
                self.iteration_limit,
            );
            current_solution = new_solution;
            if current_solution.objective_value() < best_solution_seen.objective_value() {
                best_solution_seen = current_solution.clone();
                iteration_without_global_improvement = 0;
            } else {
                iteration_without_global_improvement += 1;
            }

            if let Some(iteration_without_global_improvement_limit) =
                self.iteration_without_global_improvement_limit
            {
                if iteration_without_global_improvement
                    >= iteration_without_global_improvement_limit
                {
                    println!("Iteration without global improvement limit reached.");
                    break;
                }
            }

            if let Some(time_limit) = self.time_limit {
                if stdtime::Instant::now().duration_since(start_time) > time_limit {
                    println!("Time limit reached.");
                    break;
                }
            }
            if let Some(iteration_limit) = self.iteration_limit {
                if iteration_counter >= iteration_limit {
                    println!("Iteration limit reached.");
                    break;
                }
            }
            iteration_counter += 1;
        }
        best_solution_seen
    }
}
