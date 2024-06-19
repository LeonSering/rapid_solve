//! This module contains the [`SimulatedAnnealingSolver`] implementing the
//! [simulated annealing metaheuristic](https://en.wikipedia.org/wiki/Simulated_annealing).
//! * Starts with an initial solution and iteratively considers neighbors.
//! * An improvement is always accepted, but a worse neighbor is also accepted with a certain
//! probability.
//! * This probability is based on the difference in objective value and the current
//! temperature.
//! * The temperature is reduced whenever a worse neighbor is accepted.
//! * The search stops after a certain number of iterations, or after a certain time limit, or if the
//! whole neighborhood is explored without any acceptance.
//! * The best solution seen during this process is returned.
//! * The acceptance probability usualy depends exponentially on the difference in objective value
//! and the current temperature, i.e., e<sup>-∆f/T</sup>, where ∆f is the difference in
//! objective value and T is the current temperature.
//! * The simulated annealing heuristic is similar to the deterministic [threshold accepting
//! heuristic][super::threshold_accepting], which performs similar, but does not require
//! computing the acceptance probability.
//!
//! For an example, see the [simulated annealing solver for the
//! TSP][crate::examples::tsp::solvers::simulated_annealing].
use std::{sync::Arc, time as stdtime};

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::objective::ObjectiveValue;
use crate::objective::{EvaluatedSolution, Objective};

use super::common::default_function_between_steps;
use super::{
    common::{FunctionBetweenSteps, Neighborhood},
    Solver,
};

/// Type for the temperature, which should be in the magnitude of the objective values in the
/// beginning.
pub type Temperature = f64;
/// Type for the acceptance probability.
pub type Probability = f64;
/// Type for the `cooling_factor`, which is a value between 0 and 1 (e.g., 0.9).
pub type ScalingFactor = f64;

/// Type for the `acceptance_probability_function`.
pub type AcceptanceProbabilityFunction =
    Box<dyn Fn(&ObjectiveValue, &ObjectiveValue, Temperature) -> Probability>;

/// A simulated annealing solver that uses a [`Neighborhood`] and an [`Objective`], an
/// `initial_temperature` (`f32` in the magnitute of the objective values),
/// a `cooling_factor` (`f32`between 0 and 1, e.g., 0.9), and an
/// [`AcceptanceProbabilityFunction`] to find a good solution.
/// * The [`AcceptanceProbabilityFunction`] is a function that takes the current objective value,
/// the objective value of a neighbor, and the current temperature and returns the accpetance probability
/// (which should be 1 if the neighbor is and improvement and it should decrease with
/// increasing difference in objective value and decreasing temperature). Typical it is an
/// exponential function, e.g., e<sup>-∆f/T</sup>, where ∆f is the difference in objective value
/// and T is the current temperature.
/// * Whenever a worse neighbor is accepted, the `current_temperature` is reduced by the `cooling_factor`.
/// * The `function_between_steps` is executed after each improvement step.
/// * The default `function_between_steps` (if `None`) is printing the iteration number, the
/// objective value (in comparison the the previous objective value) and the time elapsed since the start.
/// * The solver stops after a certain number of iterations or after a certain time limit.
/// * If `iteration_limit` and `time_limit` is `None`, the solver runs until a whole neighborhood is explored
/// without any acceptance.
/// For a high-level overview, see the [module documentation][super::simulated_annealing] and for an example, see the
/// [simulated annealing solver for the TSP][crate::examples::tsp::solvers::simulated_annealing].
pub struct SimulatedAnnealingSolver<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
    initial_temperature: Temperature,
    cooling_factor: ScalingFactor,
    acceptance_probability_function: AcceptanceProbabilityFunction,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
    random_seed: Option<u64>,
}

impl<S> SimulatedAnnealingSolver<S> {
    /// Creates a new [`SimulatedAnnealingSolver`] with the given [`Neighborhood`], [`Objective`],
    /// `initial_temperature`, `cooling_factor`, and [`AcceptanceProbabilityFunction`].
    /// * A `random_seed` can be provided to make the search reproducible.
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        initial_temperature: Temperature,
        cooling_factor: ScalingFactor,
        acceptance_probability_function: AcceptanceProbabilityFunction,
        random_seed: Option<u64>,
    ) -> Self {
        Self::with_options(
            neighborhood,
            objective,
            initial_temperature,
            cooling_factor,
            acceptance_probability_function,
            random_seed,
            None,
            None,
            None,
        )
    }

    /// Creates a new [`SimulatedAnnealingSolver`] with the given [`Neighborhood`], [`Objective`],
    /// `initial_temperature`, `cooling_factor`, and [`AcceptanceProbabilityFunction`].
    /// * `random_seed` can be provided to make the search reproducible.
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
    #[allow(clippy::too_many_arguments)]
    pub fn with_options(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        initial_temperature: Temperature,
        cooling_factor: ScalingFactor,
        acceptance_probability_function: AcceptanceProbabilityFunction,
        random_seed: Option<u64>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            initial_temperature,
            cooling_factor,
            acceptance_probability_function,
            function_between_steps: function_between_steps
                .unwrap_or(default_function_between_steps()),
            time_limit,
            iteration_limit,
            random_seed,
        }
    }
}

impl<S: Clone> Solver<S> for SimulatedAnnealingSolver<S> {
    /// Solves the problem using the simulated annealing heuristic.
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();
        let mut current_solution = self.objective.evaluate(initial_solution);
        let mut best_solution_seen = current_solution.clone();
        let mut current_temperature = self.initial_temperature;

        let mut rng = match self.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        let mut iteration_counter = 1;

        while let Some(new_solution) =
            self.explore_neihborhood(&current_solution, current_temperature, &mut rng)
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
                current_temperature *= self.cooling_factor;
                println!("New temperature: {:0.2}", current_temperature);
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

impl<S> SimulatedAnnealingSolver<S> {
    fn explore_neihborhood(
        &self,
        current_solution: &EvaluatedSolution<S>,
        current_temperature: Temperature,
        rng: &mut StdRng,
    ) -> Option<EvaluatedSolution<S>> {
        self.neighborhood
            .neighbors_of(current_solution.solution())
            .find_map(|neighbor| {
                let neighbor_solution = self.objective.evaluate(neighbor);
                let acceptance_probability = (self.acceptance_probability_function)(
                    current_solution.objective_value(),
                    neighbor_solution.objective_value(),
                    current_temperature,
                );
                let random_number = rng.gen::<Probability>();
                if acceptance_probability > random_number {
                    Some(neighbor_solution)
                } else {
                    None
                }
            })
    }
}
