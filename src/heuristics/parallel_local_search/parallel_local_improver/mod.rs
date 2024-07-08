//! This module contains several [`ParallelLocalImprover`] implementations, which define the strategy to
//! explore the neighborhood of a solution in each iteration of the
//! [`ParallelLocalSearchSolver`][super::ParallelLocalSearchSolver].
mod parallel_minimizer;
mod take_any_recursion;

use crate::objective::EvaluatedSolution;
pub use parallel_minimizer::ParallelMinimizer;
pub use take_any_recursion::TakeAnyRecursion;

/// Determines for a given solution (as [`EvaluatedSolution`]) the best neighbor that has an
/// smaller [`ObjectiveValue`][crate::objective::ObjectiveValue].
/// * A solver is equipped with only one [`ParallelLocalImprover`].
/// * The [`ParallelLocalImprover`] is invoked in each iteration of the solver.
/// * Depending on the problem and especially the
/// computation costs of computing and evaluating neighbors, different [`ParallelLocalImprover`] might be
/// better.
/// * Returns `None` if there is no better solution in the [`ParallelNeighborhood`][super::ParallelNeighborhood].
pub trait ParallelLocalImprover<S>: Send + Sync {
    /// Determines for a given [`EvaluatedSolution`] the best neighbor that has an smaller
    /// [`ObjectiveValue`][crate::objective::ObjectiveValue].
    /// Returns `None` if there is no better solution in the [`ParallelNeighborhood`][super::ParallelNeighborhood].
    /// This method is called in each iteration of the
    /// [`ParallelLocalSearchSolver`][super::ParallelLocalSearchSolver].
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>>;
}
