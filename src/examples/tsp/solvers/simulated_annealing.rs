//! This module contains the implementation of the [`SimulatedAnnealingSolver`] for the TSP, see
//! the [build] function for details.
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
    heuristics::simulated_annealing::{SimulatedAnnealingSolver, Temperature},
    objective::{Objective, ObjectiveValue},
};

/// Builds a [`SimulatedAnnealingSolver`] for the TSP.
/// * The neighborhood is the 3-opt neighborhood, i.e., the neighborhood that consists of
/// all tours that can be obtained by applying the 3-opt operation.
/// * Since starting each neighborhood with the index (0, 1, 2) leads to back and forth moves,
/// the [`TspTour`][`super::super::tsp_tour::TspTour`] is wrapped in a [`TspTourWithInfo`] to store
/// the first index of the last move. The next move then starts with the first index one after the
/// first index of the previous move, which means that the backwards move appears very late in the
/// neighborhood iterator.
/// * The initial temperature is set to the average distance between two nodes.
/// * The acceptance probability function is an exponential function that accepts worse solutions
/// with a probability given by the formula e<sup>-∆f/T</sup>, where ∆f is the difference in
/// objective value and T is the current temperature.
/// * The cooling factor is set to 0.9.
/// * We set a random seed to have reproducible results.
pub fn build(tsp_instance: Arc<TspInstance>) -> SimulatedAnnealingSolver<TspTourWithInfo> {
    let node_count = tsp_instance.get_number_of_nodes();
    let average_distance: Distance = (0..node_count)
        .flat_map(|i| (0..node_count).filter_map(move |j| if i != j { Some((i, j)) } else { None }))
        .map(|(i, j)| tsp_instance.get_distance(i, j))
        .sum::<Distance>()
        / (node_count * (node_count - 1)) as Distance;

    let acceptance_probability_function = Box::new(
        |current_objective_value: &ObjectiveValue,
         new_objective_value: &ObjectiveValue,
         temperature: Temperature| {
            if new_objective_value < current_objective_value {
                1.0
            } else {
                let current_total_distance = current_objective_value
                    .iter()
                    .next()
                    .unwrap()
                    .unwrap_float();
                let new_total_distance = new_objective_value.iter().next().unwrap().unwrap_float();

                ((current_total_distance - new_total_distance) / temperature).exp()
            }
        },
    );

    let initial_temperature = average_distance;

    let neighborhood = Arc::new(RotatedThreeOptNeighborhood::new(tsp_instance));

    let objective: Arc<Objective<TspTourWithInfo>> =
        Arc::new(build_objective_for_tsp_tour_with_info());

    SimulatedAnnealingSolver::initialize(
        neighborhood,
        objective,
        initial_temperature,
        0.9,
        acceptance_probability_function,
        Some(13), // random_seed
    )
}

#[cfg(test)]
mod tests {
    use super::build;
    use crate::{
        examples::tsp::{
            tsp_instance::TspInstance, tsp_tour::TspTour, tsp_tour_with_info::TspTourWithInfo,
        },
        heuristics::Solver,
    };
    use std::sync::Arc;

    #[test]
    fn test_simulated_annealing() {
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
    fn test_simulated_annealing_large_instance() {
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
                0, 35, 38, 39, 37, 36, 33, 34, 48, 31, 44, 18, 40, 7, 8, 9, 42, 32, 11, 27, 26, 25,
                46, 12, 13, 51, 10, 50, 24, 3, 5, 14, 4, 23, 47, 45, 43, 15, 28, 49, 19, 22, 29, 1,
                6, 41, 20, 16, 2, 17, 30, 21
            ]
        );
    }
}
