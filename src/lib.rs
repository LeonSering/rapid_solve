#![warn(missing_docs)]
//! This library provides a metaheuristic framework for solving combinatorial optimization
//! problems.
//!
//! # Overview
//! ### Metaheuristics
//! The following [metaheuristics][heuristics] are included:
//! - [local search][heuristics::local_search] (with recursion and several neighborhood
//!   exploration strategies, supports recursion for multiple modifactions in one step)
//! - [parallel local search][heuristics::parallel_local_search] (the neighborhood is explored in
//!   parallel using [`rayon`](https://docs.rs/rayon/), supports recursion)
//! - [threshold accepting][heuristics::threshold_accepting]
//! - [simulated annealing][heuristics::simulated_annealing]
//! - [tabu search][heuristics::tabu_search] (and a faster [parallel
//!   version][heuristics::parallel_tabu_search])
//!
//! ### Hierarchical Objective
//! The framework supports [hierarchical objective][objective], i.e., objectives
//! that consists of multiple levels of linear combinations.
//! The first level is first minimized and only for tie-breaks the next level is considered.
//! This is useful to model hard-constraints as high-priority soft-constraints (via a violation
//! measure), such that normally
//! infeasible solutions are considered feasible. The solver than minimizes these constraints first
//! until the violation is zero and then starts to optimize the remaining objective levels.
//!
//! ### Examples
//! As an example we provide a simple implementation of the [Traveling Salesman Problem
//! (TSP)][examples::tsp] with the 3-opt neighborhood.
//!
//! # How to use this library (step-by-step example)
//! Suppose you have given a combinatorial optimization problem and defined a solution type.
//! To run a local search solver you need to do the following four steps:
//! 1. Define the [`Objective`][objective::Objective] for your problem by defining
//!    [`Indicators`][objective::Indicator] and build a hierarchical objective of
//!    [`LinearCombinations`][objective::LinearCombination] of these indicators.
//! 2. Define modifications for your solution type. The solution type should not be mutable,
//!    instead a modified clone should be returned.
//! 3. Implement the [`Neighborhood`][heuristics::common::Neighborhood] for the local search.
//! 4. Initialize the [`LocalSearchSolver`][heuristics::local_search::LocalSearchSolver]
//! and run it.
//!
//! We demonstrate these steps on a simple (but totally artificial) example, where the solution type
//! consists of a fixed-size vector of integers.
//!
//! ```rust
//! struct Solution(Vec<i64>);
//! ```
//!
//! #### 1. Define the [`Objective`][objective::Objective] for your problem.
//!
//! For the example, we want to find permutation (i.e., 0 to 10 should appear exactly once) where
//! the sum of squared differences between consecutive elements (cyclic) is minimized.
//!
//! Hence, we define two [`Indicators`][objective::Indicator], namely `PermutationViolation` and
//! `SquaredDifference`, and build a hierarchical
//! objective where `PermutationViolation` is minimized first and only for tie-breaks
//! `SquaredDifference` is considered.
//! ```rust
//! # struct Solution(Vec<i64>);
//! use rapid_solve::objective::{BaseValue, Indicator, Objective};
//!
//! struct PermutationViolation;
//!
//! impl Indicator<Solution> for PermutationViolation {
//!     fn evaluate(&self, solution: &Solution) -> BaseValue {
//!         let violation: i64 = (0..solution.0.len())
//!             .map(|i| (solution.0.iter().filter(|&n| *n == i as i64).count() as i64 - 1).abs())
//!             .sum();
//!         BaseValue::Integer(violation)
//!     }
//!     fn name(&self) -> String {
//!         String::from("PermutationViolation")
//!     }
//! }
//!
//! struct SquaredDifference;
//!
//! impl Indicator<Solution> for SquaredDifference {
//!     fn evaluate(&self, solution: &Solution) -> BaseValue {
//!         let squared_diff: i64 = (0..solution.0.len())
//!             .map(|i| (solution.0[i] - solution.0[(i + 1) % solution.0.len()]).pow(2))
//!             .sum();
//!         BaseValue::Integer(squared_diff)
//!     }
//!     fn name(&self) -> String {
//!         String::from("SquaredDifference")
//!     }
//! }
//!
//! fn build_objective() -> Objective<Solution> {
//!     Objective::new_single_indicator_per_level(vec![
//!         Box::new(PermutationViolation),
//!         Box::new(SquaredDifference),
//!     ])
//! }
//! ```
//!
//! #### 2. Define modifications for your solution type.
//!
//! In our example we use two modifications:
//!
//! - Changing one entry to a number between 0 and 10.
//! - Swapping two entries.
//!
//! The solution type should not be mutable, instead a modified clone should be returned.
//! For larger solution types the immutable data structures [crate `im`](https://docs.rs/im/) might increase
//! performance.
//! ```rust
//! # struct Solution(Vec<i64>);
//! impl Solution {
//!     fn change_entry(&self, index: usize, new_value: i64) -> Self {
//!         let mut new_values = self.0.clone();
//!         new_values[index] = new_value;
//!         Solution(new_values)
//!     }
//!     fn swap(&self, index1: usize, index2: usize) -> Self {
//!         let mut new_values = self.0.clone();
//!         new_values.swap(index1, index2);
//!         Solution(new_values)
//!     }
//! }
//! ```
//!
//! #### 3. Implement the [`Neighborhood`][heuristics::common::Neighborhood].
//!
//! In our example we want to first try to change all entries and then try all swaps.
//!
//! ```rust
//! # struct Solution(Vec<i64>);
//! # impl Solution {
//! #     fn change_entry(&self, index: usize, new_value: i64) -> Self {
//! #         let mut new_values = self.0.clone();
//! #         new_values[index] = new_value;
//! #         Solution(new_values)
//! #     }
//! #     fn swap(&self, index1: usize, index2: usize) -> Self {
//! #         let mut new_values = self.0.clone();
//! #         new_values.swap(index1, index2);
//! #         Solution(new_values)
//! #     }
//! # }
//! use rapid_solve::heuristics::common::Neighborhood;
//!
//! struct ChangeEntryThenSwapNeighborhood;
//!
//! impl Neighborhood<Solution> for ChangeEntryThenSwapNeighborhood {
//!     fn neighbors_of<'a>(
//!         &'a self,
//!         solution: &'a Solution,
//!     ) -> Box<dyn Iterator<Item = Solution> + Send + Sync + 'a> {
//!         let change_entry = (0..solution.0.len()).flat_map(move |i| {
//!             (0..10).map(move |new_value| solution.change_entry(i, new_value))
//!         });
//!         let swap = (0..solution.0.len())
//!             .flat_map(move |i| (0..solution.0.len()).map(move |j| solution.swap(i, j)));
//!         Box::new(change_entry.chain(swap))
//!     }
//! }
//! ```
//!
//! #### 4. Initialize the [`LocalSearchSolver`][heuristics::local_search::LocalSearchSolver] and run it.
//!
//! In the example only a local optimum is found, which is worse than the global optimum.
//! ```rust
//! # struct Solution(Vec<i64>);
//! #
//! # use rapid_solve::objective::{BaseValue, Indicator, Objective};
//! #
//! # struct PermutationViolation;
//! #
//! # impl Indicator<Solution> for PermutationViolation {
//! #     fn evaluate(&self, solution: &Solution) -> BaseValue {
//! #         let violation: i64 = (0..solution.0.len())
//! #             .map(|i| (solution.0.iter().filter(|&n| *n == i as i64).count() as i64 - 1).abs())
//! #             .sum();
//! #         BaseValue::Integer(violation)
//! #     }
//! #     fn name(&self) -> String {
//! #         String::from("PermutationViolation")
//! #     }
//! # }
//! #
//! # struct SquaredDifference;
//! #
//! # impl Indicator<Solution> for SquaredDifference {
//! #     fn evaluate(&self, solution: &Solution) -> BaseValue {
//! #         let squared_diff: i64 = (0..solution.0.len())
//! #             .map(|i| (solution.0[i] - solution.0[(i + 1) % solution.0.len()]).pow(2))
//! #             .sum();
//! #         BaseValue::Integer(squared_diff)
//! #     }
//! #     fn name(&self) -> String {
//! #         String::from("SquaredDifference")
//! #     }
//! # }
//! #
//! # fn build_objective() -> Objective<Solution> {
//! #     Objective::new_single_indicator_per_level(vec![
//! #         Box::new(PermutationViolation),
//! #         Box::new(SquaredDifference),
//! #     ])
//! # }
//! #
//! # impl Solution {
//! #     fn change_entry(&self, index: usize, new_value: i64) -> Self {
//! #         let mut new_values = self.0.clone();
//! #         new_values[index] = new_value;
//! #         Solution(new_values)
//! #     }
//! #     fn swap(&self, index1: usize, index2: usize) -> Self {
//! #         let mut new_values = self.0.clone();
//! #         new_values.swap(index1, index2);
//! #         Solution(new_values)
//! #     }
//! # }
//! #
//! # use rapid_solve::heuristics::common::Neighborhood;
//! #
//! # struct ChangeEntryThenSwapNeighborhood;
//! #
//! # impl Neighborhood<Solution> for ChangeEntryThenSwapNeighborhood {
//! #     fn neighbors_of<'a>(
//! #         &'a self,
//! #         solution: &'a Solution,
//! #     ) -> Box<dyn Iterator<Item = Solution> + Send + Sync + 'a> {
//! #         let change_entry = (0..solution.0.len()).flat_map(move |i| {
//! #             (0..10).map(move |new_value| solution.change_entry(i, new_value))
//! #         });
//! #         let swap = (0..solution.0.len())
//! #             .flat_map(move |i| (0..solution.0.len()).map(move |j| solution.swap(i, j)));
//! #         Box::new(change_entry.chain(swap))
//! #     }
//! # }
//! use rapid_solve::heuristics::Solver;
//! use rapid_solve::heuristics::local_search::LocalSearchSolver;
//! use std::sync::Arc;
//!
//! let objective = Arc::new(build_objective());
//! let neighborhood = Arc::new(ChangeEntryThenSwapNeighborhood);
//! let solver = LocalSearchSolver::initialize(neighborhood, objective);
//!
//! let initial_solution = Solution(vec![0; 10]);
//!
//! let evaluated_local_minimum = solver.solve(initial_solution);
//! assert_eq!(
//!     *evaluated_local_minimum.objective_value().as_vec(),
//!     vec![BaseValue::Integer(0), BaseValue::Integer(36)]
//! );
//! assert_eq!(
//!     *evaluated_local_minimum.solution().0,
//!     vec![1, 0, 2, 4, 5, 7, 9, 8, 6, 3]
//! );
//! // one global optimum is [0, 2, 4, 6, 8, 9, 7, 5, 3, 1] with a squared differences of 34.
//! ```
//!
//! For a more less artificial demonstration, we refer to the [tsp-example][examples::tsp].
//!
pub mod examples;
pub mod heuristics;
pub mod objective;
