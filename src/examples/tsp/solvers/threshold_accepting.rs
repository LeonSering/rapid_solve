//! This module contains the implementation of the [`ThresholdAcceptingSolver`] for the TSP
//! problem.
use std::sync::Arc;

use crate::{
    examples::tsp::{
        tsp_instance::TspInstance,
        tsp_tour_with_info::{
            neighborhood::RotatedThreeOptNeighborhood,
            objective::build_objective_for_tsp_tour_with_info, TspTourWithInfo,
        },
        Distance,
    },
    heuristics::threshold_accepting::ThresholdAcceptingSolver,
    objective::{BaseValue, Objective, ObjectiveValue},
};

/// Builds a [`ThresholdAcceptingSolver`] for the TSP problem.
/// The initial threshold is set to the average distance between two nodes.
/// The neighborhood is the 3-opt neighborhood, i.e., the neighborhood that consists of
/// all tours that can be obtained by applying the 3-opt operation.
/// Since starting each neighborhood with the index (0, 1, 2) leads to back and forth moves,
/// the [`TspTour`][`super::super::tsp_tour::TspTour`] is wrapped in a [`TspTourWithInfo`] to store the first index of the last move.
/// The next move then starts with the first index one after the first index of the previous move, which
/// means that the backwards move appears very late in the neighborhood iterator.
pub fn build(tsp_instance: Arc<TspInstance>) -> ThresholdAcceptingSolver<TspTourWithInfo> {
    let node_count = tsp_instance.get_number_of_nodes();
    let average_distance: Distance = (0..node_count)
        .flat_map(|i| (0..node_count).filter_map(move |j| if i != j { Some((i, j)) } else { None }))
        .map(|(i, j)| tsp_instance.get_distance(i, j))
        .sum::<Distance>()
        / (node_count * (node_count - 1)) as Distance;

    let objective: Arc<Objective<TspTourWithInfo>> =
        Arc::new(build_objective_for_tsp_tour_with_info());
    let neighborhood = Arc::new(RotatedThreeOptNeighborhood::new(tsp_instance));
    let initial_threshold = ObjectiveValue::new(vec![BaseValue::Float(average_distance)]);
    ThresholdAcceptingSolver::initialize(neighborhood, objective, initial_threshold, 0.9)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        examples::tsp::{
            tsp_instance::TspInstance, tsp_tour::TspTour, tsp_tour_with_info::TspTourWithInfo,
        },
        heuristics::Solver,
    };

    use super::build;

    #[test]
    fn test_threshold_accepting() {
        let tsp_instance = Arc::new(TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]));
        let tsp_tour_with_infos =
            TspTourWithInfo::new(TspTour::new(vec![0, 1, 2, 3], tsp_instance.clone()), 0);

        let solver = build(tsp_instance.clone());

        let final_tour = solver.solve(tsp_tour_with_infos);
        assert_eq!(final_tour.unwrap().unwrap().get_nodes(), &vec![0, 1, 3, 2]);
    }

    #[test]
    fn test_basic_local_search_large_instance() {
        let tsp_instance = Arc::new(
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap(),
        );
        let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());
        let tour_with_infos = TspTourWithInfo::new(tour, 0);
        let solver = build(tsp_instance.clone());

        let local_opt_tour = solver.solve(tour_with_infos);

        assert_eq!(
            local_opt_tour.unwrap().unwrap().get_nodes(),
            &vec![
                0, 21, 17, 16, 2, 44, 18, 40, 7, 8, 9, 32, 42, 14, 4, 5, 3, 27, 26, 25, 46, 12, 13,
                51, 10, 50, 11, 24, 45, 47, 23, 37, 36, 39, 38, 35, 34, 33, 43, 15, 28, 49, 19, 22,
                20, 41, 6, 1, 29, 30, 31, 48
            ]
        );
    }
}
