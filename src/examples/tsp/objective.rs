//! This module contains the [`Objective`] for the TSP.
use crate::objective::{BaseValue, Indicator, Objective};

use super::tsp_tour::TspTour;

struct DistanceIndicator;

impl Indicator<TspTour> for DistanceIndicator {
    fn evaluate(&self, tsp_tour: &TspTour) -> BaseValue {
        BaseValue::Float(tsp_tour.get_total_distance())
    }

    fn name(&self) -> String {
        String::from("TotalDistance")
    }
}

/// Builds the [`Objective`] for the TSP, which consists of a single [`Indicator`] for the total
/// distance of the tour.
pub fn build_tsp_objective() -> Objective<TspTour> {
    Objective::new_single_indicator(Box::new(DistanceIndicator))
}
