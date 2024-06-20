pub mod parallel_tabu_minimizer;

use crate::objective::EvaluatedSolution;
pub use parallel_tabu_minimizer::ParallelTabuMinimizer;
use std::collections::VecDeque;

pub trait ParallelTabuImprover<S, T> {
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)>;
}
