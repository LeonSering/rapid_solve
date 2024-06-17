//! This module contains the [`Objective`] for the TSP problem when considered the
//! [`TspTourWithInfo`] wrapper.

use crate::objective::{BaseValue, Indicator, Objective};

use super::TspTourWithInfo;
struct DistanceIndicator;

impl Indicator<TspTourWithInfo> for DistanceIndicator {
    fn evaluate(&self, tsp_tour: &TspTourWithInfo) -> BaseValue {
        BaseValue::Float(tsp_tour.tour.get_total_distance())
    }

    fn name(&self) -> String {
        String::from("TotalDistance")
    }
}

/// Builds the [`Objective`] for [`TspTourWithInfo`], which consists of a single [`Indicator`] for the total
/// distance of the tour.
pub fn build_objective_for_tsp_tour_with_info() -> Objective<TspTourWithInfo> {
    Objective::new_single_indicator(Box::new(DistanceIndicator))
}
