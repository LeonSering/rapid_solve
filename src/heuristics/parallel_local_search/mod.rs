//! This module contains the [`ParallelLocalSearchSolver`] implementing the
//! [local search heuristic](https://en.wikipedia.org/wiki/Local_search_(optimization)).
//! * Starts with an initial solution and improves it by exploring the neighborhood
//! of the current solution in parallel.
//! * The search stops after a certain number of iterations, after a certain time limit, or if no
//! improvement is found in the neighborhood (local minimum is reached).
//! * The last solution (which is the best found) is returned.
//!
//! For examples, see the [parallel local search solver][crate::examples::tsp::solvers::parallel_local_search] for the TSP.
pub mod parallel_local_improver;

use std::sync::Arc;
use std::time as stdtime;

use crate::objective::EvaluatedSolution;
use crate::objective::Objective;

use self::parallel_local_improver::ParallelLocalImprover;
use self::parallel_local_improver::ParallelMinimizer;

use super::common::default_function_between_steps;
use super::common::FunctionBetweenSteps;
use super::common::ParallelNeighborhood;
use super::Solver;

/// A local search solver that uses a [`ParallelNeighborhood`] and an [`Objective`] to find a local minimum.
/// * The `function_between_steps` is executed after each improvement step.
/// * The default `function_between_steps` (if `None`) is printing the iteration number, the objective value
/// (in comparison the the previous objective value) and the time elapsed since the start.
///
/// For a high-level overview, see the [module documentation][super::parallel_local_search] and for examples, see the
/// [parallel local search solver][crate::examples::tsp::solvers::parallel_local_search] for the
/// TSP.
pub struct ParallelLocalSearchSolver<S> {
    objective: Arc<Objective<S>>,
    local_improver: Box<dyn ParallelLocalImprover<S>>,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S: Send + Sync + 'static> ParallelLocalSearchSolver<S> {
    /// Creates a new [`ParallelLocalSearchSolver`] with the given [`ParallelNeighborhood`] and [`Objective`].
    /// Uses the default [`ParallelLocalImprover`] ([`ParallelMinimizer`]) and the default `function_between_steps` (print
    /// iteration number, objective value, time elapsed).
    pub fn initialize(
        neighborhood: Arc<impl ParallelNeighborhood<S> + 'static>,
        objective: Arc<Objective<S>>,
    ) -> Self {
        Self::with_options(neighborhood, objective, None, None, None, None)
    }

    /// Creates a new [`ParallelLocalSearchSolver`] with the given [`ParallelNeighborhood`] and [`Objective`].
    /// * `local_improver` (implementing [`ParallelLocalImprover`]) specifies the how the neighborhood is
    /// explored. If `None`, the default is [`ParallelMinimizer`].
    /// * `function_between_steps` is executed after each improvement step. If `None`, the default
    /// is printing the iteration number, the objective value (in comparison the the previous
    /// objective value) and the time elapsed since the start.
    /// * `time_limit` is the maximum time allowed for the local search to start a new iteration.
    /// The last iteration is allowed to finish. If `None`, there is no time limit.
    /// * `iteration_limit` is the maximum number of iterations allowed for the local search. If
    /// `None`, there is no iteration limit.
    /// * If both `time_limit` and `iteration_limit` are set, the search stops when either limit is
    /// reached.
    pub fn with_options(
        neighborhood: Arc<impl ParallelNeighborhood<S> + 'static>,
        objective: Arc<Objective<S>>,
        local_improver: Option<Box<dyn ParallelLocalImprover<S>>>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        let local_improver = match local_improver {
            Some(local_improver) => local_improver,
            None => Box::new(ParallelMinimizer::new(neighborhood, objective.clone()))
                as Box<dyn ParallelLocalImprover<S>>,
        };
        Self {
            objective,
            local_improver,
            function_between_steps: function_between_steps
                .unwrap_or(default_function_between_steps()),
            time_limit,
            iteration_limit,
        }
    }
}

impl<S> Solver<S> for ParallelLocalSearchSolver<S> {
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();

        let mut current_solution = self.objective.evaluate(initial_solution);
        let mut iteration_counter = 1;
        while let Some(new_solution) = self.local_improver.improve(&current_solution) {
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
        current_solution
    }
}
