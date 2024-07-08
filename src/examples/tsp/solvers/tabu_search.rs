//! This module contains the implementation of the [`TabuSearchSolver`] for the TSP, see
//! the [build] function for details.
use std::{collections::VecDeque, sync::Arc};

use crate::{
    examples::tsp::{
        objective::build_tsp_objective, tsp_instance::TspInstance, tsp_tour::TspTour, NodeIdx,
    },
    heuristics::tabu_search::{TabuNeighborhood, TabuSearchSolver},
    objective::Objective,
};

/// A tabu consisits of a directed arc between two nodes. Neighbors that would insert this arc are
/// tabu.
#[derive(Debug)]
pub struct Tabu {
    start: NodeIdx,
    end: NodeIdx,
}

impl Tabu {
    /// Checks if the given 3-opt move is tabu.
    pub fn is_tabu(&self, i: usize, j: usize, k: usize, tour: &TspTour) -> bool {
        let n = tour.get_nodes().len();
        (self.start == *tour.get_nodes().get(i).unwrap()
            && self.end == *tour.get_nodes().get(j + 1).unwrap())
            || (self.start == *tour.get_nodes().get(j).unwrap()
                && self.end == *tour.get_nodes().get((k + 1) % n).unwrap())
            || (self.start == *tour.get_nodes().get(k).unwrap()
                && self.end == *tour.get_nodes().get(i + 1).unwrap())
    }

    /// Creates for a given 3-opt move the three tabus corresponding to the arcs that are removed
    /// by the move.
    pub fn create_tabus(i: usize, j: usize, k: usize, tour: &TspTour) -> Vec<Tabu> {
        let n = tour.get_nodes().len();
        vec![
            Tabu {
                start: *tour.get_nodes().get(i).unwrap(),
                end: *tour.get_nodes().get(i + 1).unwrap(),
            },
            Tabu {
                start: *tour.get_nodes().get(j).unwrap(),
                end: *tour.get_nodes().get(j + 1).unwrap(),
            },
            Tabu {
                start: *tour.get_nodes().get(k).unwrap(),
                end: *tour.get_nodes().get((k + 1) % n).unwrap(),
            },
        ]
    }
}

/// A 3-opt [`TabuNeighborhood`] for the TSP.
/// For a given tour and a tabu list, all 3-opt moves are generated, all moves that are tabu (i.e.,
/// that would insert a tabu arc) are filtered out.
/// Each 3-opt move is equipped with three tabus, one for each arc that is removed by the move.
pub struct ThreeOptTabuNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl ThreeOptTabuNeighborhood {
    /// Creates a new [`ThreeOptTabuNeighborhood`] for the given [`TspInstance`].
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl TabuNeighborhood<TspTour, Tabu> for ThreeOptTabuNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        tour: &'a TspTour,
        tabu_list: &'a VecDeque<Tabu>,
    ) -> Box<dyn Iterator<Item = (TspTour, Vec<Tabu>)> + Send + Sync + 'a> {
        let num_nodes = self.tsp_instance.get_number_of_nodes();
        Box::new(
            (0..num_nodes - 2)
                .flat_map(move |i| {
                    (i + 1..num_nodes - 1)
                        .flat_map(move |j| (j + 1..num_nodes).map(move |k| (i, j, k)))
                })
                .filter_map(move |(i, j, k)| {
                    if tabu_list.iter().any(|tabu| tabu.is_tabu(i, j, k, tour)) {
                        return None;
                    }
                    Some((
                        tour.three_opt_swap(i, j, k),
                        Tabu::create_tabus(i, j, k, tour),
                    ))
                }),
        )
    }
}

/// Builds a [`TabuSearchSolver`] for the TSP.
/// * The neighborhood is the 3-opt neighborhood, i.e., the neighborhood that consists of
/// all tours that can be obtained by applying the 3-opt operation.
/// * The tabu list size is set to 30.
/// * The iteration without global improvement limit is set to 100, i.e., the search stops if no
/// global improvement is found for 100 iterations.
/// * Takes the default ['TabuImprover`] [`TabuMinimizer`] which returns the best non-tabu neighbor
/// without using parallelism.
pub fn build(tsp_instance: Arc<TspInstance>) -> TabuSearchSolver<TspTour, Tabu> {
    let objective: Arc<Objective<TspTour>> = Arc::new(build_tsp_objective());
    let neighborhood = Arc::new(ThreeOptTabuNeighborhood::new(tsp_instance.clone()));
    let tabu_list_size = 30;
    let iteration_without_global_improvement_limit = 100;

    TabuSearchSolver::initialize(
        neighborhood,
        objective,
        tabu_list_size,
        iteration_without_global_improvement_limit,
    )
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::{
        examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour},
        heuristics::Solver,
    };
    use std::sync::Arc;

    #[test]
    fn test_tabu_search() {
        let tsp_instance = Arc::new(TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]));

        let tour = TspTour::new(vec![0, 1, 2, 3], tsp_instance.clone());

        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour);

        assert_eq!(local_opt_tour.solution().get_nodes(), &vec![0, 2, 3, 1]);
    }

    #[test]
    fn test_tabu_search_large_instance() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());
        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour);

        assert_eq!(
            local_opt_tour.solution().get_nodes(),
            &vec![
                0, 35, 38, 39, 36, 37, 23, 47, 4, 14, 5, 3, 24, 45, 43, 33, 34, 48, 31, 44, 18, 40,
                7, 8, 9, 42, 32, 50, 11, 27, 26, 12, 13, 51, 10, 25, 46, 28, 15, 49, 19, 22, 29, 1,
                6, 41, 20, 16, 2, 17, 30, 21
            ]
        );
    }
}
