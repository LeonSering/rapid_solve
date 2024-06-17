//! This module contains the threshold accpeting solver and its components.
//! The [threshold accpeting heuristic](https://doi.org/10.1016%2F0021-9991%2890%2990201-B)
//! starts with an initial solution and iteratively considers neighbors.
//! An improvement is always accepted, but a worsening is accepted if it is below a threshold.
//! After each step the threshold is reduced by a factor.
//! The search stops after a certain number of iterations or after a certain time limit.
//! The best solution seen so far is returned.
//!
//! The threshold accepting heuristic is similar to the [simulated annealing
//! heuristic][super::simulated_annealing], but deterministic and without
//! computing the acceptance probability (which often contains an exponential function).

use super::common::{function_between_steps, FunctionBetweenSteps, Neighborhood};
use super::Solver;
use crate::objective::{EvaluatedSolution, Objective, ObjectiveValue};
use std::sync::Arc;
use std::time as stdtime;

/// If max_iterations and max_time is None, the solver runs until a whole neighborhood is explored
/// without any accpetance.
pub struct ThresholdAcceptingSolver<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
    threshold: ObjectiveValue,
    threshold_factor: f32,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S> ThresholdAcceptingSolver<S> {
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        threshold: ObjectiveValue,
        threshold_factor: f32,
    ) -> Self {
        Self::with_options(
            neighborhood,
            objective,
            threshold,
            threshold_factor,
            None,
            None,
            None,
        )
    }

    pub fn with_options(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        threshold: ObjectiveValue,
        threshold_factor: f32,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            threshold,
            threshold_factor,
            function_between_steps: function_between_steps
                .unwrap_or_else(|| function_between_steps::get_default()),
            time_limit,
            iteration_limit,
        }
    }
}

impl<S: Clone> Solver<S> for ThresholdAcceptingSolver<S> {
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();
        let mut current_solution = self.objective.evaluate(initial_solution);
        let mut best_solution_seen = current_solution.clone();
        let mut current_threshold: ObjectiveValue = self.threshold.clone();

        let mut iteration_counter = 1;

        while let Some(new_solution) =
            self.explore_neihborhood(&current_solution, &current_threshold)
        {
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
            current_threshold = current_threshold * self.threshold_factor;
            println!("New threshold:");
            self.objective.print_objective_value(&current_threshold);
            iteration_counter += 1;
        }

        best_solution_seen
    }
}

impl<S> ThresholdAcceptingSolver<S> {
    fn explore_neihborhood(
        &self,
        current_solution: &EvaluatedSolution<S>,
        current_threshold: &ObjectiveValue,
    ) -> Option<EvaluatedSolution<S>> {
        self.neighborhood
            .neighbors_of(current_solution.solution())
            .find_map(|neighbor| {
                let neighbor_solution = self.objective.evaluate(neighbor);
                if neighbor_solution.objective_value().clone()
                    < current_solution.objective_value().clone() + current_threshold.clone()
                {
                    Some(neighbor_solution)
                } else {
                    None
                }
            })
    }
}
