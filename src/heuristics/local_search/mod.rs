//! This module contains the local search solver and its components.
//! The [local search heuristic](https://en.wikipedia.org/wiki/Local_search_(optimization))
//! starts with an initial solution and iteratively improves it by
//! exploring the neighborhood of the current solution.

pub mod local_improver;
pub mod neighborhood;
mod search_result;

pub use neighborhood::Neighborhood;

use std::sync::Arc;
use std::time as stdtime;
use std::time::Instant;

use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use local_improver::LocalImprover;

#[allow(unused_imports)]
use local_improver::Minimizer;

#[allow(unused_imports)]
use local_improver::TakeFirstRecursion;

#[allow(unused_imports)]
use local_improver::TakeAnyParallelRecursion;

use search_result::SearchResult;
use search_result::SearchResult::{Improvement, NoImprovement};

// function between steps with the following signature:
// * iteration counter
// * current solution
// * previous solution (Option)
// * objective
// * time that local search started (Option)
type FunctionBetweenSteps<S> = Box<
    dyn Fn(
        u32,
        &EvaluatedSolution<S>,
        Option<&EvaluatedSolution<S>>,
        Arc<Objective<S>>,
        Option<Instant>,
    ),
>;

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
    function_between_steps: Option<FunctionBetweenSteps<S>>,
}

impl<S> LocalSearchSolver<S> {
    /// Creates a new [`LocalSearchSolver`] with the given [`Neighborhood`] and [`Objective`].
    /// Uses the default [`LocalImprover`] ([`Minimizer`]) and the default `function_between_steps` (print
    /// iteration number, objective value, time elapsed).
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            local_improver: None,
            function_between_steps: None,
        }
    }

    /// Creates a new [`LocalSearchSolver`] with the given [`Neighborhood`] and [`Objective`].
    /// Uses the given [`LocalImprover`] (`None` = [`Minimizer`]) and the default `function_between_steps` (`None` = print
    /// iteration number, objective value, time elapsed).
    pub fn with_local_improver_and_function(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        local_improver: Option<Box<dyn LocalImprover<S>>>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            local_improver,
            function_between_steps,
        }
    }
}

impl<S> LocalSearchSolver<S> {
    /// Finds a local minimum by iteratively improving the given initial solution.
    pub fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
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
            Some(start_time),
        )
        .unwrap()
    }

    fn find_local_optimum(
        &self,
        start_solution: EvaluatedSolution<S>,
        local_improver: &dyn LocalImprover<S>,
        start_time: Option<stdtime::Instant>,
    ) -> SearchResult<S> {
        let mut result = NoImprovement(start_solution);
        let mut iteration_counter = 1;
        while let Some(new_solution) = local_improver.improve(result.as_ref()) {
            match self.function_between_steps.as_ref() {
                None => {
                    println!("Iteration {}:", iteration_counter);
                    self.objective.print_objective_value_with_comparison(
                        new_solution.objective_value(),
                        result.as_ref().objective_value(),
                    );
                    if let Some(start_time) = start_time {
                        println!(
                            "elapsed time for local search: {:0.2}sec",
                            stdtime::Instant::now()
                                .duration_since(start_time)
                                .as_secs_f32()
                        );
                    }
                    println!();
                }
                Some(f) => f(
                    iteration_counter,
                    &new_solution,
                    Some(result.as_ref()),
                    self.objective.clone(),
                    start_time,
                ),
            }
            result = Improvement(new_solution);
            iteration_counter += 1;
        }
        result
    }
}
