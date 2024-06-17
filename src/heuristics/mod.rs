//! This module contains the implementation of the (meta)heuristics.

use crate::objective::EvaluatedSolution;
pub mod common;
pub mod local_search;
pub mod threshold_accepting;

/// All local-search-based solvers implement this trait.
pub trait Solver<S> {
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S>;
}
