//! This module contains the implementations of the (meta)heuristics.

use crate::objective::EvaluatedSolution;
pub mod common;
pub mod local_search;
pub mod parallel_local_search;
pub mod parallel_tabu_search;
pub mod simulated_annealing;
pub mod tabu_search;
pub mod threshold_accepting;

/// All local-search-based solvers implement this trait.
pub trait Solver<S> {
    /// Solves the problem starting from the given initial solution.
    fn solve(&self, initial_solution: S) -> EvaluatedSolution<S>;
}
