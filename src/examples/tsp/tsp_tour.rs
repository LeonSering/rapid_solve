//! This module contains the [`TspTour`], i.e., a permutation of all nodes of the [`TspInstance`].
use std::sync::Arc;

use super::{tsp_instance::TspInstance, Distance, NodeIdx};

/// Represents a tour of a [`TspInstance`]. Contain all indices between 0 and n-1.
#[derive(Clone)]
pub struct TspTour {
    nodes: Vec<NodeIdx>,
    total_distance: Distance,
    tsp_instance: Arc<TspInstance>,
}

impl TspTour {
    /// Creates a new [`TspTour`] with the given nodes and computes the total distance.
    pub fn new(nodes: Vec<NodeIdx>, tsp_instance: Arc<TspInstance>) -> TspTour {
        let total_distance = nodes
            .iter()
            .zip(nodes.iter().cycle().skip(1))
            .map(|(&i, &j)| tsp_instance.get_distance(i, j))
            .sum();

        TspTour::new_pre_computed(nodes, total_distance, tsp_instance)
    }

    fn new_pre_computed(
        nodes: Vec<NodeIdx>,
        total_distance: Distance,
        tsp_instance: Arc<TspInstance>,
    ) -> TspTour {
        TspTour {
            nodes,
            total_distance,
            tsp_instance,
        }
    }

    /// Creates a new [`TspTour`] using the [nearest neighbor heuristic](https://en.wikipedia.org/wiki/Nearest_neighbour_algorithm).
    pub fn from_instance_nearest_neighbor(tsp_instance: Arc<TspInstance>) -> TspTour {
        let mut nodes = Vec::with_capacity(tsp_instance.get_number_of_nodes());
        let mut visited = vec![false; tsp_instance.get_number_of_nodes()];
        let mut current_node = 0;
        let mut total_distance = 0.0;

        visited[current_node] = true;
        nodes.push(current_node);

        for _ in 1..tsp_instance.get_number_of_nodes() {
            let mut nearest_node = None;
            let mut nearest_distance = Distance::INFINITY;

            for (next_node, visited) in visited.iter().enumerate() {
                if !visited {
                    let distance = tsp_instance.get_distance(current_node, next_node);
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_node = Some(next_node);
                    }
                }
            }

            if let Some(next_node) = nearest_node {
                nodes.push(next_node);
                visited[next_node] = true;
                total_distance += nearest_distance;
                current_node = next_node;
            }
        }

        // Return to node 0
        total_distance += tsp_instance.get_distance(current_node, 0);

        TspTour::new_pre_computed(nodes, total_distance, tsp_instance)
    }

    /// Returns the `nodes` of the tour.
    pub fn get_nodes(&self) -> &Vec<NodeIdx> {
        &self.nodes
    }

    /// Returns the `total_distance` of the tour.
    pub fn get_total_distance(&self) -> Distance {
        self.total_distance
    }

    /// Performs a single [3-opt swap](https://en.wikipedia.org/wiki/3-opt) on the tour.
    /// * Assumes that 0 <= i < j < k < n.
    /// * New [`TspTour`] consists of the nodes with the following index in the current tour
    ///     - first nodes with index 0 to i
    ///     - then the nodes with index from j+1 to k
    ///     - then the nodes with index from i+1 to j
    ///     - finally the nodes with index from k+1 to n-1.
    pub fn three_opt_swap(&self, i: usize, j: usize, k: usize) -> TspTour {
        let mut new_distance = self.total_distance;
        let n = self.nodes.len();

        // Remove distance of arcs (i, i+1), (j, j+1), and (k, k+1)
        new_distance -= self
            .tsp_instance
            .get_distance(self.nodes[i], self.nodes[(i + 1) % n]);
        new_distance -= self
            .tsp_instance
            .get_distance(self.nodes[j], self.nodes[(j + 1) % n]);
        new_distance -= self
            .tsp_instance
            .get_distance(self.nodes[k], self.nodes[(k + 1) % n]);

        // Add distance of arcs (i, j+1), (j, k+1), and (k, i+1)
        new_distance += self
            .tsp_instance
            .get_distance(self.nodes[i], self.nodes[(j + 1) % n]);
        new_distance += self
            .tsp_instance
            .get_distance(self.nodes[j], self.nodes[(k + 1) % n]);
        new_distance += self
            .tsp_instance
            .get_distance(self.nodes[k], self.nodes[(i + 1) % n]);

        // Perform the swap
        let mut new_nodes = Vec::with_capacity(self.nodes.len());
        new_nodes.extend_from_slice(&self.nodes[0..i + 1]);
        new_nodes.extend_from_slice(&self.nodes[j + 1..k + 1]);
        new_nodes.extend_from_slice(&self.nodes[i + 1..j + 1]);
        new_nodes.extend_from_slice(&self.nodes[k + 1..]);

        /* for i in 0..1000000 { // TEMP
            let _ = i;
        } */

        TspTour::new_pre_computed(new_nodes, new_distance, self.tsp_instance.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tsp_tour() {
        let tsp_instance = TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]);

        let tour = TspTour::new(vec![0, 1, 2, 3], Arc::new(tsp_instance));
        assert_eq!(tour.get_nodes(), &vec![0, 1, 2, 3]);
        assert_eq!(tour.get_total_distance(), 10.0 + 35.0 + 30.0 + 20.0);
    }

    #[test]
    fn test_new_tsp_tour_nearest_neighbor() {
        let tsp_instance = TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]);

        let tour = TspTour::from_instance_nearest_neighbor(Arc::new(tsp_instance));
        assert_eq!(tour.get_nodes(), &vec![0, 1, 3, 2]);
        assert_eq!(tour.get_total_distance(), 10.0 + 25.0 + 30.0 + 15.0);
    }

    #[test]
    fn test_three_opt_swap() {
        let tsp_instance = TspInstance::new(vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ]);

        let tour = TspTour::from_instance_nearest_neighbor(Arc::new(tsp_instance));
        let new_tour = tour.three_opt_swap(1, 2, 3);
        assert_eq!(new_tour.get_nodes(), &vec![0, 1, 2, 3]);
        assert_eq!(new_tour.get_total_distance(), 10.0 + 35.0 + 30.0 + 20.0);
    }
}
