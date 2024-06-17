use std::{sync::Arc, time as stdtime};

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::objective::ObjectiveValue;
use crate::objective::{EvaluatedSolution, Objective};

use super::{
    common::{function_between_steps, FunctionBetweenSteps, Neighborhood},
    Solver,
};

pub type Temperature = f64;
pub type Probability = f64;
pub type ScalingFactor = f64;

type AcceptanceProbabilityFunction =
    Box<dyn Fn(&ObjectiveValue, &ObjectiveValue, Temperature) -> Probability>;

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
                .unwrap_or(function_between_steps::default()),
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
