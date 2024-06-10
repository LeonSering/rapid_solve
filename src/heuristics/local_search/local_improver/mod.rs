pub mod minimizer;
pub mod parallel_minimizer;
pub mod take_any_parallel_recursion;
pub mod take_first;
pub mod take_first_recursion;

pub use minimizer::Minimizer;
pub use take_any_parallel_recursion::TakeAnyParallelRecursion;
pub use take_first_recursion::TakeFirstRecursion;

use crate::objective::EvaluatedSolution;
/// Determines for a given solution the best neighbor that has an improving objective function.
/// A solver is equipped with only one LocalImprover. Depending on the problem and especially the
/// computation costs of computing and evaluating neighbors, different LocalImprover might be
/// better.
/// Returns None if there is no better solution in the neighborhood.
pub trait LocalImprover<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>>;
}
