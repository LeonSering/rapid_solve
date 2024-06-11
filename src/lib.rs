//! This library provides a metaheuristic framework for solving combinatorial optimization
//! problems.
//!
//! # Overview
//! ### Metaheuristics
//! The following metaheuristics are included:
//! - [local search][crate::heuristics::local_search] (with several neighborhood
//! exploration strategies)
//!
//! ### Hierarchical Objective
//! The framework supports [hierarchical objective][crate::objective], i.e., objectives
//! that consists of multiple levels of linear combinations.
//! The first level is first minimized and only for tie-breaks the next level is considered.
//! This is useful to model hard-constraints as high-priority soft-constraints (via a violation
//! measure), such that normally
//! infeasible solutions are considered feasible. The solver than minimizes these constraints first
//! until the violation is zero and then starts to optimize the remaining objective levels.
//!
//! ### Examples
//! As an example we provide a simple implementation of the [Traveling Salesman Problem
//! (TSP)][crate::examples::tsp] with the 3-Opt neighborhood.
//!
//! # How to use this library
//! Suppose you have given a combinatorial optimization problem and defined a solution type S. For
//! example take a fixed sized array of integers:
//! ```rust
//! type Solution = [i8; 10];
//! ```
//! S should be non-mutable.
//!
//! 1. Define the objective for your problem.
//! For example we want to find permutation where the positional length between successive elements
//! is minimized:
//!
pub mod examples;
pub mod heuristics;
pub mod objective;
pub mod time;
