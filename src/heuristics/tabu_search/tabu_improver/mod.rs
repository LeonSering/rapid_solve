//! This module contains several [`TabuImprover`] implementation, which define the strategy to
//! explore the neighborhood of a solution in each iteration of the
//! [`TabuSearchSolver`][super::TabuSearchSolver].
pub mod tabu_minimizer;

use crate::objective::EvaluatedSolution;
use std::collections::VecDeque;
pub use tabu_minimizer::TabuMinimizer;

/// Determines for a given solution (as [`EvaluatedSolution`]) and a tabu list the best neighbor,
/// that are not tabu, together with new tabus to add to the tabu list.
/// * A solver is equipped with only one [`TabuImprover`].
/// * The [`TabuImprover`] is invoked in each iteration of the tabu search.
/// * Only returns `None` if there are no neighbors.
pub trait TabuImprover<S, T>: Send + Sync {
    /// Determines for a given [`EvaluatedSolution`] and a tabu list the best neighbor, that are
    /// not tabu, together with new tabus to add to the tabu list.
    /// Returns `None` if there are no neighbors.
    /// This method is called in each iteration of the
    /// [`TabuSearchSolver`][super::TabuSearchSolver].
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)>;
}
