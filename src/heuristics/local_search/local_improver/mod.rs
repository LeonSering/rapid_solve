//! This module contains several [`LocalImprover`] implementations, which define the strategy to
//! explore the neighborhood of a solution in each iteration of the
//! [`LocalSearchSolver`][super::LocalSearchSolver].
mod minimizer;
mod parallel_minimizer;
mod take_any_parallel_recursion;
mod take_first;
mod take_first_recursion;

pub use minimizer::Minimizer;
pub use parallel_minimizer::ParallelMinimizer;
pub use take_any_parallel_recursion::TakeAnyParallelRecursion;
pub use take_first::TakeFirst;
pub use take_first_recursion::TakeFirstRecursion;

use crate::objective::EvaluatedSolution;
/// Determines for a given solution (as [`EvaluatedSolution`]) the best neighbor that has an
/// smaller [`ObjectiveValue`][crate::objective::ObjectiveValue].
/// A solver is equipped with only one [`LocalImprover`].
/// The [`LocalImprover`] is invoked in each iteration of the solver.
/// Depending on the problem and especially the
/// computation costs of computing and evaluating neighbors, different [`LocalImprover`] might be
/// better.
/// Returns `None` if there is no better solution in the [`Neighborhood`][super::Neighborhood].
pub trait LocalImprover<S> {
    /// Determines for a given [`EvaluatedSolution`] the best neighbor that has an smaller
    /// [`ObjectiveValue`][crate::objective::ObjectiveValue].
    /// Returns `None` if there is no better solution in the [`Neighborhood`][super::Neighborhood].
    /// This method is called in each iteration of the
    /// [`LocalSearchSolver`][super::LocalSearchSolver].
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>>;
}
