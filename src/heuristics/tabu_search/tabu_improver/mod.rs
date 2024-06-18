pub mod tabu_minimizer;
use std::collections::VecDeque;

use crate::objective::EvaluatedSolution;
pub use tabu_minimizer::TabuMinimizer;

pub trait TabuImprover<S, T> {
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)>;
}
