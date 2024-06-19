//! [`Minimizer`] searches the whole [`Neighborhood`] of a solution and returns the best
//! improving neighbor.
use super::super::Neighborhood;
use super::LocalImprover;
use crate::objective::EvaluatedSolution;
use crate::objective::Objective;
use std::sync::Arc;

/// [`Minimizer`] searches the whole [`Neighborhood`] of a solution and returns the best neighbor
/// if it is better than the given solution.
/// * No parallelism is used.
/// * Works for every solution type `S`.
/// * Is fast if the computation and the evaluating of a neighbor is cheap.
/// * If no neighbor is better than the given solution (or no neighbor exists), `None` is returned.
pub struct Minimizer<S> {
    neighborhood: Arc<dyn Neighborhood<S>>,
    objective: Arc<Objective<S>>,
}

impl<S> Minimizer<S> {
    /// Creates a new [`Minimizer`] with the given [`Neighborhood`] and [`Objective`].
    pub fn new(
        neighborhood: Arc<dyn Neighborhood<S>>,
        objective: Arc<Objective<S>>,
    ) -> Minimizer<S> {
        Minimizer {
            neighborhood,
            objective,
        }
    }
}

impl<S> LocalImprover<S> for Minimizer<S> {
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
