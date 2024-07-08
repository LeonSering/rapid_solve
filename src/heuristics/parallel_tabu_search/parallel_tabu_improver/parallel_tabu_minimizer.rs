//! [`ParallelTabuMinimizer`] searches the whole [`ParallelTabuNeighborhood`] of a solution in parallel
//! and returns the best non-tabu neighbor.

use crate::{
    heuristics::parallel_tabu_search::ParallelTabuNeighborhood,
    objective::{EvaluatedSolution, Objective},
};
use rayon::iter::ParallelIterator;
use std::{collections::VecDeque, sync::Arc};

use super::ParallelTabuImprover;

/// [`ParallelTabuMinimizer`] searches the whole [`ParallelTabuNeighborhood`] of a solution (and a tabu list)
/// and returns the best non-tabu neighbor with new tabus.
/// * This is done in parallel using the [`ParallelIterator`] of [`rayon`].
/// * Solution type `S` and the tabu type `T` must implement [`Send`] and [`Sync`].
/// * If the computation or the evaluation of a neighbor is CPU-heavy this might be a good choice.
/// * If all neighbors are tabu, `None` is returned.
pub struct ParallelTabuMinimizer<S, N> {
    neighborhood: Arc<N>,
    objective: Arc<Objective<S>>,
}

impl<S, N> ParallelTabuMinimizer<S, N> {
    /// Creates a new [`ParallelTabuMinimizer`] with the given [`ParallelTabuNeighborhood`] and [`Objective`].
    pub fn new(neighborhood: Arc<N>, objective: Arc<Objective<S>>) -> Self {
        Self {
            neighborhood,
            objective,
        }
    }
}

impl<S: Send + Sync, T: Send + Sync, N: ParallelTabuNeighborhood<S, T>> ParallelTabuImprover<S, T>
    for ParallelTabuMinimizer<S, N>
{
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)> {
        let best_neighbor_with_new_tabus = self
            .neighborhood
            .neighbors_of(solution.solution(), tabu_list)
            .map(|(neighbor, new_tabus)| (self.objective.evaluate(neighbor), new_tabus))
            .min_by(|(s1, _), (s2, _)| {
                s1.objective_value()
                    .partial_cmp(s2.objective_value())
                    .unwrap()
            });
        if best_neighbor_with_new_tabus.is_none() {
            println!("\x1b[31mwarning:\x1b[0m no swap possible.");
        }

        best_neighbor_with_new_tabus
    }
}
