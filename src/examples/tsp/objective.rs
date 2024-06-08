use crate::objective::{BaseValue, Coefficient, Indicator, Level, Objective};

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

pub fn build() -> Objective<TspTour> {
    // An objective is hierarchically and consists of multiple levels given by a Vec<Level>. The first level is the most
    // important one, and the last level is the least important one.
    // Each level consists of a linear combination of indicators. The indicators are weighted by
    // coefficients. The value of the level is the sum of the weighted indicators.

    // In this case, we only have one level with one indicator, which is the total distance of the
    // tour.
    let level = Level::new(vec![(Coefficient::Integer(1), Box::new(DistanceIndicator))]);

    Objective::new(vec![level])
}
