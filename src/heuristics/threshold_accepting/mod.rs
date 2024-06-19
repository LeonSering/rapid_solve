//! This module contains the [`ThresholdAcceptingSolver`] implementing the
//! [threshold accpeting metaheuristic](https://doi.org/10.1016%2F0021-9991%2890%2990201-B).
//! * Starts with an initial solution and iteratively considers neighbors.
//! * An improvement is always accepted, but a worse neighbor is also accepted if the difference in objective value
//! is below a given threshold.
//! * After every step, in which a worse neighbor is accepted, the threshold is reduced by a factor.
//! * The search stops after a certain number of iterations, after a certain time limit, or if the
//! whole neighborhood is explored without any acceptance.
//! * The best solution seen during this process is returned.
//! * The threshold accepting heuristic is similar to the [simulated annealing
//! heuristic][super::simulated_annealing], but deterministic and without
//! computing the acceptance probability (which often contains costly computations of exponential functions).
//!
//! For an example, see the [threshold accepting solver for the
//! TSP][crate::examples::tsp::solvers::threshold_accepting].

use super::common::{default_function_between_steps, FunctionBetweenSteps, Neighborhood};
use super::Solver;
use crate::objective::{EvaluatedSolution, Objective, ObjectiveValue};
use std::sync::Arc;
use std::time as stdtime;

/// Type for the `threshold_factor`.
pub type ScalingFactor = f32;

/// The threshold accepting solver uses a [`Neighborhood`], an [`Objective`], an
/// `initial_threshold` ([`ObjectiveValue`]) and a `threshold_factor`
/// (`f32` between 0 and 1, e.g., 0.9) to find a good solution,
/// while occasionally accepting worse solutions with the hope to not get trapped within a bad local minimum.
/// * Whenever a worse neighbor is accepted, the `current_threshold` is reduced by the `threshold_factor`.
/// * The `function_between_steps` is executed after each improvement step.
/// * The default `function_between_steps` (if `None`) is printing the iteration number, the objective value
/// (in comparison the the previous objective value) and the time elapsed since the start.
/// * The solver stops after a certain number of iterations or after a certain time limit.
/// * If `max_iterations` and `max_time` is `None`, the solver runs until a whole neighborhood is explored
/// without any accpetance.
///
/// For a high-level overview, see the [module documentation][super::threshold_accepting] and for an example, see the
/// [threshold accepting solver for the TSP][crate::examples::tsp::solvers::threshold_accepting].
pub struct ThresholdAcceptingSolver<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
    initial_threshold: ObjectiveValue,
    threshold_factor: ScalingFactor,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S> ThresholdAcceptingSolver<S> {
    /// Creates a new [`ThresholdAcceptingSolver`] with the given [`Neighborhood`], [`Objective`],
    /// `initial_threshold` and `threshold_factor` (value between 0 and 1, e.g., 0.9).
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        initial_threshold: ObjectiveValue,
        threshold_factor: ScalingFactor,
    ) -> Self {
        Self::with_options(
            neighborhood,
            objective,
            initial_threshold,
            threshold_factor,
            None,
            None,
            None,
        )
    }

    /// Creates a new [`ThresholdAcceptingSolver`] with the given [`Neighborhood`], [`Objective`],
    /// `initial_threshold` and `threshold_factor` (value between 0 and 1, e.g., 0.9).
    /// * `function_between_steps` is executed after each improvement step. If `None`, the default
    /// is printing the iteration number, the objective value (in comparison the the previous
    /// objective value) and the time elapsed since the start.
    /// * `time_limit` is the maximum time allowed for the local search to start a new iteration.
    /// The last iteration is allowed to finish. If `None`, there is no time limit.
    /// * `iteration_limit` is the maximum number of iterations allowed for the local search. If
    /// `None`, there is no iteration limit.
    /// * If `max_iterations` and `max_time` is `None`, the solver runs until a whole neighborhood
    /// is explored without any accpetance.
    /// * If `max_iterations` and `max_time` are both set, the search stops when either limit is
    /// reached first.
    pub fn with_options(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        initial_threshold: ObjectiveValue,
        threshold_factor: ScalingFactor,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            initial_threshold,
            threshold_factor,
            function_between_steps: function_between_steps
                .unwrap_or(default_function_between_steps()),
            time_limit,
            iteration_limit,
        }
    }
}

impl<S: Clone> Solver<S> for ThresholdAcceptingSolver<S> {
    /// Solves the problem using the threshold accepting heuristic.
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();
        let mut current_solution = self.objective.evaluate(initial_solution);
        let mut best_solution_seen = current_solution.clone();
        let mut current_threshold: ObjectiveValue = self.initial_threshold.clone();

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

            if new_solution.objective_value() >= current_solution.objective_value() {
                current_threshold = current_threshold * self.threshold_factor;
                println!("New threshold:");
                self.objective.print_objective_value(&current_threshold);
            }

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
