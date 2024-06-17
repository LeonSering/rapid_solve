//! A simple implementation of the [Travelling Salesman Problem (TSP)](https://en.wikipedia.org/wiki/Travelling_salesman_problem) and [several metaheuristic solvers][solvers].

pub mod neighborhood;
pub mod objective;
pub mod solvers;
pub mod tsp_instance;
pub mod tsp_tour;
pub mod tsp_tour_with_info;

/// A node index.
pub type NodeIdx = usize;

/// Distance between nodes.
pub type Distance = f64;
