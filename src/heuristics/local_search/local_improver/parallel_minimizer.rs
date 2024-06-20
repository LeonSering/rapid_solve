//! [`ParallelMinimizer`] searches the whole [`Neighborhood`] of a solution in parallel and returns the best
//! neighbor.
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

use super::super::Neighborhood;
use super::LocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use std::sync::Arc;

// TODO: Check when this Improver performs better than the normal Minimizer
/// [`ParallelMinimizer`] searches the whole [`Neighborhood`] of a solution in parallel and returns the best neighbor.
/// * This is done in parallel using [`par_bridge()`][rayon::iter::ParallelBridge] of [`rayon`].
/// * If the computation or the evaluation of a neighbor is CPU-heavy this might be a good choice.
/// * Solution type `S` must implement [`Send`] and [`Sync`].
pub struct ParallelMinimizer<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
}

impl<S> ParallelMinimizer<S> {
    /// Creates a new [`ParallelMinimizer`] with the given [`Neighborhood`] and [`Objective`].
    pub fn new(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> ParallelMinimizer<S> {
        ParallelMinimizer {
            neighborhood,
            objective,
        }
    }
}

impl<S: Send + Sync> LocalImprover<S> for ParallelMinimizer<S> {
    fn improve(&self, solution: &EvaluatedSolution<S>) -> Option<EvaluatedSolution<S>> {
        let best_neighbor_opt = self
            .neighborhood
            .neighbors_of(solution.solution())
            .par_bridge()
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
