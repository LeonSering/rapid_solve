//! [`TabuMinimizer`] searches the whole [`TabuNeighborhood`] of a solution and returns the best
//! neighbor.
use std::{collections::VecDeque, sync::Arc};

use crate::{
    heuristics::tabu_search::TabuNeighborhood,
    objective::{EvaluatedSolution, Objective},
};

use super::TabuImprover;

/// [`TabuMinimizer`] searches the whole [`TabuNeighborhood`] of a solution (and a tabu list)
/// and returns the best non-tabu neighbor with new tabus.
/// * No parallelism is used.
/// * Works for every solution type `S` and tabu type `T`.
/// * Is fast if the computation and the evaluating of a neighbor is cheap.
/// * If all neighbors are tabu, `None` is returned.
pub struct TabuMinimizer<S, T> {
    neighborhood: Arc<dyn TabuNeighborhood<S, T>>,
    objective: Arc<Objective<S>>,
}

impl<S, T> TabuMinimizer<S, T> {
    /// Creates a new [`TabuMinimizer`] with the given [`TabuNeighborhood`] and [`Objective`].
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

impl<S, T> TabuImprover<S, T> for TabuMinimizer<S, T> {
    /// Searches the whole [`TabuNeighborhood`] of a solution (and a tabu list) and returns the best
    /// non-tabu neighbor with new tabus.
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
