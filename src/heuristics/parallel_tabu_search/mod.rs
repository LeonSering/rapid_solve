pub mod parallel_tabu_improver;

use rayon::iter::ParallelIterator;

use self::parallel_tabu_improver::{ParallelTabuImprover, ParallelTabuMinimizer};

use super::common::{default_function_between_steps, FunctionBetweenSteps};
use super::Solver;
use crate::objective::{EvaluatedSolution, Objective};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time as stdtime;

/// TODO
pub trait ParallelTabuNeighborhood<S: Send, T: Send>: Send + Sync {
    /// TODO
    fn neighbors_of<'a>(
        &'a self,
        solution: &'a S,
        tabu_list: &'a VecDeque<T>,
    ) -> impl ParallelIterator<Item = (S, Vec<T>)> + 'a;
}

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
