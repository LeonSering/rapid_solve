# RapidSolve [![RapidSolve crate](https://img.shields.io/crates/v/rapid_solve.svg)](https://crates.io/crates/rapid_solve) [![RapidSolve documentation](https://docs.rs/rapid_solve/latest/rapid_solve/badge.svg)](https://docs.rs/rapid_solve)

This library provides a metaheuristic framework for solving combinatorial optimization
problems.

## Overview

### Metaheuristics

The following metaheuristics are included:

- [local search](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/local_search/index.html) (with several neighborhood
  exploration strategies)
- [parallel local search](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/parallel_local_search/index.html)
  (the neighborhood is explored in parallel using [`rayon`](https://docs.rs/rayon/))
- [threshold accepting](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/threshold_accepting/index.html)
- [simulated annealing](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/simulated_annealing/index.html)
- [tabu search](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/tabu_search/index.html) (and a faster [parallel
  version](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/parallel_tabu_search/index.html))

### Hierarchical Objective

The framework supports [hierarchical objective](https://docs.rs/rapid_solve/latest/rapid_solve/objective/index.html), i.e., objectives
that consists of multiple levels of linear combinations.
The first level is first minimized and only for tie-breaks the next level is considered.
This is useful to model hard-constraints as high-priority soft-constraints (via a violation
measure), such that normally
infeasible solutions are considered feasible. The solver than minimizes these constraints first
until the violation is zero and then starts to optimize the remaining objective levels.

### Examples

As an example we provide a simple implementation of the [Traveling Salesman Problem
(TSP)](https://docs.rs/rapid_solve/latest/rapid_solve/examples/tsp/index.html) with the 3-opt neighborhood.

## How to use this library (step-by-step example)

Suppose you have given a combinatorial optimization problem and defined a solution type.
To run a local search solver you need to do the following four steps:

1. Define the [`Objective`](https://docs.rs/rapid_solve/latest/rapid_solve/objective/struct.Objective.html) for your problem by defining
   [`Indicators`](https://docs.rs/rapid_solve/latest/rapid_solve/objective/indicator/trait.Indicator.html) and build a hierarchical objective of
   [`LinearCombinations`](https://docs.rs/rapid_solve/latest/rapid_solve/objective/linear_combination/struct.LinearCombination.html) of these indicators.
2. Define modifications for your solution type. The solution type should not be mutable,
   instead a modified clone should be returned.
3. Implement the [`Neighborhood`](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/common/neighborhood/trait.Neighborhood.html) for the local search.
4. Initialize the [`LocalSearchSolver`](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/local_search/struct.LocalSearchSolver.html)
   and run it.

We demonstrate these steps on a simple (but totally artificial) example, where the solution type
consists of a fixed-size vector of integers.

```rust
struct Solution(Vec<i64>);
```

#### 1. Define the [`Objective`](https://docs.rs/rapid_solve/latest/rapid_solve/objective/index.html) for your problem.

For the example, we want to find permutation (i.e., 0 to 10 should appear exactly once) where
the sum of squared differences between consecutive elements (cyclic) is minimized.

Hence, we define two [`Indicators`](https://docs.rs/rapid_solve/latest/rapid_solve/objective/indicator/trait.Indicator.html), namely `PermutationViolation` and
`SquaredDifference`, and build a hierarchical
objective where `PermutationViolation` is minimized first and only for tie-breaks
`SquaredDifference` is considered.

```rust
# struct Solution(Vec<i64>);
use rapid_solve::objective::{BaseValue, Indicator, Objective};

struct PermutationViolation;

impl Indicator<Solution> for PermutationViolation {
    fn evaluate(&self, solution: &Solution) -> BaseValue {
        let violation: i64 = (0..solution.0.len())
            .map(|i| (solution.0.iter().filter(|&n| *n == i as i64).count() as i64 - 1).abs())
            .sum();
        BaseValue::Integer(violation)
    }
    fn name(&self) -> String {
        String::from("PermutationViolation")
    }
}

struct SquaredDifference;

impl Indicator<Solution> for SquaredDifference {
    fn evaluate(&self, solution: &Solution) -> BaseValue {
        let squared_diff: i64 = (0..solution.0.len())
            .map(|i| (solution.0[i] - solution.0[(i + 1) % solution.0.len()]).pow(2))
            .sum();
        BaseValue::Integer(squared_diff)
    }
    fn name(&self) -> String {
        String::from("SquaredDifference")
    }
}

fn build_objective() -> Objective<Solution> {
    Objective::new_single_indicator_per_level(vec![
        Box::new(PermutationViolation),
        Box::new(SquaredDifference),
    ])
}
```

#### 2. Define modifications for your solution type.

In our example we use two modifications:

- Changing one entry to a number between 0 and 10.
- Swapping two entries.

The solution type should not be mutable, instead a modified clone should be returned.
For larger solution types the immutable data structures [crate `im`](https://docs.rs/im/) might increase
performance.

```rust
impl Solution {
    fn change_entry(&self, index: usize, new_value: i64) -> Self {
        let mut new_values = self.0.clone();
        new_values[index] = new_value;
        Solution(new_values)
    }
    fn swap(&self, index1: usize, index2: usize) -> Self {
        let mut new_values = self.0.clone();
        new_values.swap(index1, index2);
        Solution(new_values)
    }
}
```

#### 3. Implement the [`Neighborhood`](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/common/neighborhood/trait.Neighborhood.html).

In our example we want to first try to change all entries and then try all swaps.

```rust
use rapid_solve::heuristics::local_search::Neighborhood;

struct ChangeEntryThenSwapNeighborhood;

impl Neighborhood<Solution> for ChangeEntryThenSwapNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        solution: &'a Solution,
    ) -> Box<dyn Iterator<Item = Solution> + Send + Sync + 'a> {
        let change_entry = (0..solution.0.len()).flat_map(move |i| {
            (0..10).map(move |new_value| solution.change_entry(i, new_value))
        });
        let swap = (0..solution.0.len())
            .flat_map(move |i| (0..solution.0.len()).map(move |j| solution.swap(i, j)));
        Box::new(change_entry.chain(swap))
    }
}
```

#### 4. Initialize the [`LocalSearchSolver`](https://docs.rs/rapid_solve/latest/rapid_solve/heuristics/local_search/struct.LocalSearchSolver.html) and run it.

In the example only a local optimum is found, which is worse than the global optimum.

```rust
use rapid_solve::heuristics::local_search::LocalSearchSolver;
use std::sync::Arc;

let objective = Arc::new(build_objective());
let neighborhood = Arc::new(ChangeEntryThenSwapNeighborhood);
let solver = LocalSearchSolver::initialize(neighborhood, objective);

let initial_solution = Solution(vec![0; 10]);

let evaluated_local_minimum = solver.solve(initial_solution);
assert_eq!(
    *evaluated_local_minimum.objective_value().as_vec(),
    vec![BaseValue::Integer(0), BaseValue::Integer(36)]
);
assert_eq!(
    *evaluated_local_minimum.solution().0,
    vec![1, 0, 2, 4, 5, 7, 9, 8, 6, 3]
);
// one global optimum is [0, 2, 4, 6, 8, 9, 7, 5, 3, 1] with a squared differences of 34.
```

For a more less artificial demonstration, we refer to the [tsp-example](https://docs.rs/rapid_solve/latest/rapid_solve/examples/tsp/index.html).
