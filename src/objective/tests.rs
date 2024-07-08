use rapid_time::Duration;

use super::*;

#[derive(Clone, PartialEq, PartialOrd)]
struct TestSolution {
    field1: i32,
    field2: i32,
    field3: f32,
    field4: f32,
    field5: Duration,
    field6: Duration,
}

struct FirstIndicator;

impl Indicator<TestSolution> for FirstIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Integer(solution.field1 as i64)
    }

    fn name(&self) -> String {
        "Field1".to_string()
    }
}

struct SecondIndicator;

impl Indicator<TestSolution> for SecondIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Integer(solution.field2 as i64)
    }

    fn name(&self) -> String {
        "Field2".to_string()
    }
}

struct ThirdIndicator;

impl Indicator<TestSolution> for ThirdIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Float(solution.field3 as f64)
    }

    fn name(&self) -> String {
        "Field3".to_string()
    }
}

struct FourthIndicator;

impl Indicator<TestSolution> for FourthIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Float(solution.field4 as f64)
    }

    fn name(&self) -> String {
        "Field4".to_string()
    }
}

struct FifthIndicator;

impl Indicator<TestSolution> for FifthIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Duration(solution.field5)
    }

    fn name(&self) -> String {
        "Field5".to_string()
    }
}

struct SixthIndicator;

impl Indicator<TestSolution> for SixthIndicator {
    fn evaluate(&self, solution: &TestSolution) -> BaseValue {
        BaseValue::Duration(solution.field6)
    }

    fn name(&self) -> String {
        "Field6".to_string()
    }
}

#[test]
fn test() {
    // ARRANGE
    let level1 = LinearCombination::new(vec![
        (Coefficient::Integer(1), Box::new(FirstIndicator)),
        (Coefficient::Float(10.5), Box::new(SecondIndicator)),
    ]);

    let level2 = LinearCombination::new(vec![
        (Coefficient::Integer(1), Box::new(ThirdIndicator)),
        (Coefficient::Float(-1.5), Box::new(FourthIndicator)),
    ]);

    let level3 = LinearCombination::new(vec![
        (Coefficient::Float(0.5), Box::new(FifthIndicator)),
        (Coefficient::Integer(10), Box::new(SixthIndicator)),
    ]);

    let objective = Objective::new(vec![level1, level2, level3]);

    let solution1 = TestSolution {
        field1: 1,
        field2: 2,
        field3: 6.0,
        field4: 4.0,
        field5: Duration::from_seconds(10),
        field6: Duration::from_seconds(6),
    };

    let solution2 = TestSolution {
        field1: 2122,
        field2: -200,
        field3: 150.0015,
        field4: 100.001,
        field5: Duration::from_iso("PT20H2M10S"),
        field6: Duration::from_iso("P1DT5H1M6S"),
    };

    let objective_value1 = ObjectiveValue::new(vec![
        BaseValue::Integer(22),
        BaseValue::Float(0.0),
        BaseValue::Duration(Duration::from_seconds(65)),
    ]);

    let objective_value2 = ObjectiveValue::new(vec![
        BaseValue::Integer(22),
        BaseValue::Zero,
        BaseValue::Duration(Duration::from_iso("P12DT12H12M5S")),
    ]);

    let objective_value3 =
        ObjectiveValue::new(vec![BaseValue::Maximum, BaseValue::Zero, BaseValue::Zero]);

    let objective_value4 = ObjectiveValue::new(vec![
        BaseValue::Integer(22),
        BaseValue::Float(0.0),
        BaseValue::Zero,
    ]);

    let objective_value_diff = ObjectiveValue::new(vec![
        BaseValue::Zero,
        BaseValue::Zero,
        BaseValue::Duration(Duration::from_iso("P12DT12H11M0S")),
    ]);

    let objective_value_sum = ObjectiveValue::new(vec![
        BaseValue::Integer(44),
        BaseValue::Float(0.0),
        BaseValue::Duration(Duration::from_iso("P12DT12H13M10S")),
    ]);

    let zero = ObjectiveValue::new(vec![BaseValue::Zero, BaseValue::Zero, BaseValue::Zero]);

    // ACT
    let evaluated_solution1 = objective.evaluate(solution1);
    let evaluated_solution2 = objective.evaluate(solution2);

    // ASSERT
    assert_eq!(evaluated_solution1.objective_value(), &objective_value1);

    assert_eq!(evaluated_solution2.objective_value(), &objective_value2);

    assert!(evaluated_solution1 < evaluated_solution2);

    assert!(evaluated_solution1.objective_value() < &objective_value3);

    assert!(evaluated_solution2.objective_value() > &objective_value4);

    assert_eq!(
        evaluated_solution1.objective_value().clone() - objective_value1,
        zero
    );

    assert_eq!(
        evaluated_solution2.objective_value().clone() - objective_value2,
        zero
    );

    assert_eq!(
        evaluated_solution2.objective_value().clone()
            - evaluated_solution1.objective_value().clone(),
        objective_value_diff
    );

    assert_eq!(
        evaluated_solution2.objective_value().clone()
            + evaluated_solution1.objective_value().clone(),
        objective_value_sum
    );
}
