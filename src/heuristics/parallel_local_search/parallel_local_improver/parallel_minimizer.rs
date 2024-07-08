//! [`ParallelMinimizer`] searches the whole [`ParallelNeighborhood`] of a solution in parallel and returns the best
//! improving neighbor. As the neighborhood returns a [`ParallelIterator`] over all neighbors, the
//! [`ParallelMinimizer`] can evaluate the neighbors in parallel.
use super::super::ParallelNeighborhood;
use super::ParallelLocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use rayon::iter::ParallelIterator;
use std::sync::Arc;

/// [`ParallelMinimizer`] searches the whole [`ParallelNeighborhood`] of a solution in parallel and returns the best neighbor
/// if it is better than the given solution.
/// * This is done in parallel using the [`ParallelIterator`] of [`rayon`].
/// * If the computation or the evaluation of a neighbor is CPU-heavy this might be a good choice.
/// * Solution type `S` must implement [`Send`] and [`Sync`].
pub struct ParallelMinimizer<S, N> {
    neighborhood: Arc<N>,
    objective: Arc<Objective<S>>,
}

impl<S, N> ParallelMinimizer<S, N> {
    /// Creates a new [`ParallelMinimizer`] with the given [`ParallelNeighborhood`] and [`Objective`].
    pub fn new(neighborhood: Arc<N>, objective: Arc<Objective<S>>) -> ParallelMinimizer<S, N> {
        ParallelMinimizer {
            neighborhood,
            objective,
        }
    }
}

impl<S: Send + Sync, N: ParallelNeighborhood<S>> ParallelLocalImprover<S>
    for ParallelMinimizer<S, N>
{
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        let best_neighbor_opt = self
            .neighborhood
            .neighbors_of(solution.solution())
            .map(|neighbor| self.objective.evaluate(neighbor))
            .min_by(|s1, s2| {
                s1.objective_value()
                    .partial_cmp(s2.objective_value())
                    .unwrap()
            });
        match best_neighbor_opt {
            Some(best_neighbor) => {
                if best_neighbor.objective_value() < solution.objective_value() {
                    Some(best_neighbor)
                } else {
                    None // no improvement found
                }
            }
            None => {
                println!("\x1b[31mwarning:\x1b[0m no swap possible.");
                None
            }
        }
    }
}
