//! TODO
use super::common::{default_function_between_steps, FunctionBetweenSteps, Neighborhood};
use super::local_search::local_improver::{LocalImprover, Minimizer};
use super::Solver;
use crate::objective::{EvaluatedSolution, Objective};
use std::sync::Arc;
use std::time as stdtime;

/// TODO
pub struct TabuSearchSolver<S, T> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
    tabu_list: Vec<T>,
    tabu_list_size: usize,
    local_improver: Option<Box<dyn LocalImprover<S>>>,
    function_between_steps: FunctionBetweenSteps<S>,
    time_limit: Option<stdtime::Duration>,
    iteration_limit: Option<u32>,
}

impl<S, T> TabuSearchSolver<S, T> {
    /// TODO
    pub fn initialize(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        tabu_list_size: usize,
    ) -> Self {
        Self::with_options(
            neighborhood,
            objective,
            tabu_list_size,
            None,
            None,
            None,
            None,
        )
    }

    /// TODO
    pub fn with_options(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
        tabu_list_size: usize,
        local_improver: Option<Box<dyn LocalImprover<S>>>,
        function_between_steps: Option<FunctionBetweenSteps<S>>,
        time_limit: Option<stdtime::Duration>,
        iteration_limit: Option<u32>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
            tabu_list: Vec::new(),
            tabu_list_size,
            local_improver,
            function_between_steps: function_between_steps
                .unwrap_or(default_function_between_steps()),
            time_limit,
            iteration_limit,
        }
    }
}

impl<S, T> Solver<S> for TabuSearchSolver<S, T> {
    /// TODO
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S> {
        let start_time = stdtime::Instant::now();

        let minimizer: Box<dyn LocalImprover<S>> = Box::new(Minimizer::new(
            self.neighborhood.clone(),
            self.objective.clone(),
        ));
        let local_improver = self.local_improver.as_ref().unwrap_or(&minimizer);

        let mut current_solution = self.objective.evaluate(initial_solution);
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
