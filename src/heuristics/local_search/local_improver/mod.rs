pub mod minimizer;
pub mod take_any_parallel_recursion;
pub mod take_first_recursion;

pub use minimizer::Minimizer;
pub use take_any_parallel_recursion::TakeAnyParallelRecursion;
pub use take_first_recursion::TakeFirstRecursion;

use crate::objective::EvaluatedSolution;
/// Determines for a given solution the best neighbor that has an improving objective function.
/// Returns None if there is no better solution in the neighborhood.
pub trait LocalImprover<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>>;
}
