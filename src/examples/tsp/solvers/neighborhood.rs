use std::sync::Arc;

use crate::{
    examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour},
    heuristics::local_search::Neighborhood,
};

pub struct ThreeOptNeighborhood {
    tsp_instance: Arc<TspInstance>,
}

impl ThreeOptNeighborhood {
    pub fn new(tsp_instance: Arc<TspInstance>) -> Self {
        Self { tsp_instance }
    }
}

impl Neighborhood<TspTour> for ThreeOptNeighborhood {
    fn neighbors_of<'a>(
        &'a self,
        tour: &'a TspTour,
    ) -> Box<dyn Iterator<Item = TspTour> + Send + Sync + 'a> {
        Box::new(
            (0..self.tsp_instance.get_number_of_nodes() - 2).flat_map(move |i| {
                (i + 1..self.tsp_instance.get_number_of_nodes() - 1).flat_map(move |j| {
                    (j + 1..self.tsp_instance.get_number_of_nodes())
                        .map(move |k| tour.three_opt_swap(i, j, k))
                })
            }),
        )
    }
}
