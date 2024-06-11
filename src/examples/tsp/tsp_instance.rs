//! This module contains the [`TspInstance`] which is given by a distance matrix.
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{Distance, NodeIdx};

type Coordinate = f64;
type NodeCount = usize;

/// A [`TspInstance`] consists of a (potentially asymmetric) distance matrix and can be loading from a
/// [TSPLIB file](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/).
#[derive(PartialOrd, PartialEq)]
pub struct TspInstance {
    number_of_nodes: NodeCount,
    distance_matrix: Vec<Vec<Distance>>,
}

// methods
impl TspInstance {
    /// Returns the distance between two nodes.
    pub fn get_distance(&self, from: NodeIdx, to: NodeIdx) -> Distance {
        self.distance_matrix[from][to]
    }

    /// Returns the number of nodes in the instance.
    pub fn get_number_of_nodes(&self) -> NodeCount {
        self.number_of_nodes
    }
}

// static
impl TspInstance {
    /// Creates a new [`TspInstance`] with the given `distance_matrix`.
    pub fn new(distance_matrix: Vec<Vec<Distance>>) -> TspInstance {
        let number_of_nodes = distance_matrix.len();
        for row in distance_matrix.iter() {
            assert_eq!(row.len(), number_of_nodes);
        }
        TspInstance {
            number_of_nodes,
            distance_matrix,
        }
    }

    /// Loads a [`TspInstance`] from a [TSPLIB
    /// file](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/). Support symmetric and
    /// asymmetric instances.
    pub fn from_tsplib_file(file_path: &str) -> Result<TspInstance, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut line_iter = reader.lines().map(|l| l.unwrap().trim().to_string());

        let mut number_of_nodes = 0;

        let mut tsp_type: String = "".to_string();

        for line in line_iter.by_ref() {
            if line.starts_with("TYPE") {
                tsp_type = line.split(':').collect::<Vec<&str>>()[1].trim().to_string();
            }

            if line.starts_with("DIMENSION") {
                number_of_nodes = line.split(':').collect::<Vec<&str>>()[1].trim().parse()?;
                break;
            }
        }

        match tsp_type.as_str() {
            "TSP" => TspInstance::read_tsp_lines(line_iter, number_of_nodes),
            "ATSP" => TspInstance::read_atsp_lines(line_iter, number_of_nodes),
            _ => panic!("Unsupported TSP type: {}", tsp_type),
        }
    }

    fn read_tsp_lines(
        mut line_iter: impl Iterator<Item = String>,
        number_of_nodes: NodeCount,
    ) -> Result<TspInstance, Box<dyn Error>> {
        let mut nodes: Vec<(Coordinate, Coordinate)> = Vec::new();
        for line in line_iter.by_ref() {
            if line.starts_with("NODE_COORD_SECTION") {
                break;
            }
        }
        for _ in 0..number_of_nodes {
            let line = line_iter.next().ok_or("Error reading node coordinates")?;
            let values = line.split_whitespace().collect::<Vec<&str>>();

            let x = values[1].parse()?;
            let y = values[2].parse()?;

            nodes.push((x, y));
        }

        let mut distances = vec![vec![0.0; number_of_nodes]; number_of_nodes];
        for i in 0..number_of_nodes {
            for j in 0..number_of_nodes {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];
                distances[i][j] = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
            }
        }
        Ok(TspInstance::new(distances))
    }

    fn read_atsp_lines(
        mut line_iter: impl Iterator<Item = String>,
        number_of_nodes: NodeCount,
    ) -> Result<TspInstance, Box<dyn Error>> {
        let mut distances = vec![vec![0.0; number_of_nodes]; number_of_nodes];
        for line in line_iter.by_ref() {
            if line.starts_with("EDGE_WEIGHT_SECTION") {
                break;
            }
        }

        for distance_row in distances.iter_mut() {
            let mut values: Vec<Distance> = Vec::with_capacity(number_of_nodes);

            while values.len() < number_of_nodes {
                let line = line_iter.next().ok_or("Error reading edge weights")?;
                let parsed_values: Result<Vec<Distance>, _> =
                    line.split_whitespace().map(|s| s.parse()).collect();

                values.extend(parsed_values.map_err(|_| "Error parsing distance values")?);
            }

            if values.len() != number_of_nodes {
                return Err("Mismatch in number of distance values".into());
            }

            distance_row.copy_from_slice(&values);
        }

        Ok(TspInstance::new(distances))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn tsplib_tsp_file_test() {
        let tsp_instance =
            TspInstance::from_tsplib_file("resources/tsp_test_instances/berlin52.tsp").unwrap();
        assert_eq!(tsp_instance.get_number_of_nodes(), 52);

        let distance_between_0_and_1 = (540.0 * 540.0 + 390.0 * 390.0 as Distance).sqrt();
        assert_eq!(tsp_instance.get_distance(0, 1), distance_between_0_and_1);
        assert_eq!(tsp_instance.get_distance(1, 0), distance_between_0_and_1);
    }

    #[test]
    fn tsplib_atsp_file_test() {
        let tsp_instance =
            TspInstance::from_tsplib_file("resources/tsp_test_instances/br17.atsp").unwrap();
        assert_eq!(tsp_instance.get_number_of_nodes(), 17);

        assert_eq!(tsp_instance.get_distance(2, 3), 72.0);
        assert_eq!(tsp_instance.get_distance(3, 2), 74.0);
    }
}
