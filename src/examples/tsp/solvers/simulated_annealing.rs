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

type Type = Distance;

pub fn build(tsp_instance: Arc<TspInstance>) -> SimulatedAnnealingSolver<TspTourWithInfo> {
    let node_count = tsp_instance.get_number_of_nodes();
    let average_distance: Distance = (0..node_count)
        .flat_map(|i| (0..node_count).filter_map(move |j| if i != j { Some((i, j)) } else { None }))
        .map(|(i, j)| tsp_instance.get_distance(i, j))
        .sum::<Type>()
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
