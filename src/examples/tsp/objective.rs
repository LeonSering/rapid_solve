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

pub fn build_tsp_objective() -> Objective<TspTour> {
    // In this case, we only have one level with one indicator, which is the total distance of the tour.
    Objective::new_single_indicator(Box::new(DistanceIndicator))
}
