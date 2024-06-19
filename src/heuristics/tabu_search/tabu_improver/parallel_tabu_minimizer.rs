//! [`ParallelTabuMinimizer`] searches the whole [`TabuNeighborhood`] of a solution in parallel
//! and returns the best non-tabu neighbor.
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

use super::TabuImprover;
use crate::{
    heuristics::tabu_search::TabuNeighborhood,
    objective::{EvaluatedSolution, Objective},
};
use std::{collections::VecDeque, sync::Arc};

// TODO: Check when this Improver performs better than the normal TabuMinimizer
/// [`ParallelTabuMinimizer`] searches the whole [`TabuNeighborhood`] of a solution (and a tabu list)
/// and returns the best non-tabu neighbor with new tabus.
/// * This is done in parallel using [`par_bridge()`][rayon::iter::ParallelBridge] of [`rayon`].
/// * Solution type `S` and the tabu type `T` must implement [`Send`] and [`Sync`].
/// * If the computation or the evaluation of a neighbor is CPU-heavy this might be a good choice.
/// * If all neighbors are tabu, `None` is returned.
pub struct ParallelTabuMinimizer<S, T> {
    neighborhood: Arc<dyn TabuNeighborhood<S, T>>,
    objective: Arc<Objective<S>>,
}

impl<S, T> ParallelTabuMinimizer<S, T> {
    /// Creates a new [`ParallelTabuMinimizer`] with the given [`TabuNeighborhood`] and [`Objective`].
    pub fn new(
        neighborhood: Arc<dyn TabuNeighborhood<S, T>>,
        objective: Arc<Objective<S>>,
    ) -> Self {
        Self {
            neighborhood,
            objective,
        }
    }
}

impl<S: Send + Sync, T: Send + Sync> TabuImprover<S, T> for ParallelTabuMinimizer<S, T> {
    fn improve(
        &self,
        solution: &EvaluatedSolution<S>,
        tabu_list: &VecDeque<T>,
    ) -> Option<(EvaluatedSolution<S>, Vec<T>)> {
        let best_neighbor_with_new_tabus = self
            .neighborhood
            .neighbors_of(solution.solution(), tabu_list)
            .par_bridge()
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
