//! This module contains the local search solver and its components.
//! The [local search heuristic](https://en.wikipedia.org/wiki/Local_search_(optimization))
//! starts with an initial solution and iteratively improves it by
//! exploring the neighborhood of the current solution.

pub mod local_improver;

use std::sync::Arc;
use std::time as stdtime;

use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use local_improver::LocalImprover;

#[allow(unused_imports)]
use local_improver::Minimizer;

#[allow(unused_imports)]
use local_improver::TakeFirstRecursion;

#[allow(unused_imports)]
use local_improver::TakeAnyParallelRecursion;

use super::common::function_between_steps;
use super::common::FunctionBetweenSteps;
use super::common::Neighborhood;
use super::Solver;

/// A local search solver that uses a [`Neighborhood`] and an [`Objective`] to find a local minimum.
/// * There are a variety of [`LocalImprovers`][`LocalImprover`] that can be used with this solver.
/// * The `function_between_steps` is executed after each improvement step.
/// * The deafult [`LocalImprover`] (if `None`) is [`Minimizer`].
/// * The default `function_between_steps` (if `None`) is printing the iteration number, the objective value
/// (in comparison the the previous objective value) and the time elapsed since the start.
pub struct LocalSearchSolver<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
    local_improver: Option<Box<dyn LocalImprover<S>>>,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S> LocalSearchSolver<S> {
    /// Creates a new [`LocalSearchSolver`] with the given [`Neighborhood`] and [`Objective`].
    /// Uses the default [`LocalImprover`] ([`Minimizer`]) and the default `function_between_steps` (print
    /// iteration number, objective value, time elapsed).
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> Self {
        Self::with_options(neighborhood, objective, None, None, None, None)
    }

    /// Creates a new [`LocalSearchSolver`] with the given [`Neighborhood`] and [`Objective`].
    /// * `local_improver` (implementing [`LocalImprover`]) specifies the how the neighborhood is
    /// explored. If `None`, the default is [`Minimizer`].
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
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        local_improver: Option<Box<dyn LocalImprover<S>>>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            local_improver,
            function_between_steps: function_between_steps
                .unwrap_or_else(function_between_steps::get_default),
            time_limit,
            iteration_limit,
        }
    }
}

impl<S> Solver<S> for LocalSearchSolver<S> {
    /// Finds a local minimum by iteratively improving the given initial solution.
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();
        let init_solution = self.objective.evaluate(initial_solution);

        // default local improver is Minimizer
        let minimizer: Box<dyn LocalImprover<S>> = Box::new(Minimizer::new(
            self.neighborhood.clone(),
            self.objective.clone(),
        ));

        self.find_local_optimum(
            init_solution,
            self.local_improver.as_ref().unwrap_or(&minimizer).as_ref(),
            start_time,
        )
    }
}

impl<S> LocalSearchSolver<S> {
    fn find_local_optimum(
        &self,
        start_solution: EvaluatedSolution<S>,
        local_improver: &dyn LocalImprover<S>,
        start_time: stdtime::Instant,
    ) -> EvaluatedSolution<S> {
        let mut current_solution = start_solution;
        let mut iteration_counter = 1;
        while let Some(new_solution) = local_improver.improve(&current_solution) {
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
