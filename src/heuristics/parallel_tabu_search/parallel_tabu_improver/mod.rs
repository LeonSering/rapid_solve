//! This module contains the [`ParallelTabuImprover`] implementations, which define the strategy to
//! explore the neighborhood of a solution in each iteration of the
//! [`ParallelTabuSearchSolver`][super::ParallelTabuSearchSolver].
pub mod parallel_tabu_minimizer;

use crate::objective::EvaluatedSolution;
pub use parallel_tabu_minimizer::ParallelTabuMinimizer;
use std::collections::VecDeque;

/// Determines for a given solution (as [`EvaluatedSolution`]) and a tabu list the best neighbor,
/// that are not tabu, together with new tabus to add to the tabu list.
/// * A solver is equipped with only one [`ParallelTabuImprover`].
/// * The [`ParallelTabuImprover`] is invoked in each iteration of the tabu search.
/// * Only returns `None` if there are no neighbors.
/// * The Improver should use parallelization to speed up the search.
pub trait ParallelTabuImprover<S, T> {
    /// Determines for a given [`EvaluatedSolution`] and a tabu list the best neighbor, that are
    /// not tabu, together with new tabus to add to the tabu list.
    /// Returns `None` if there are no neighbors.
    /// This method is called in each iteration of the
    /// [`ParallelTabuSearchSolver`][super::ParallelTabuSearchSolver].
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)>;
}
