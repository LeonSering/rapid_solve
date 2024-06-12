// use std::env;
// use std::sync::Arc;
//
// use rapid_solve::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};
//
// use rapid_solve::examples::tsp::solvers;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() != 2 {
//         eprintln!("Usage: {} <tsplib_file>", args[0]);
//         std::process::exit(1);
//     }
//
//     let tsp_instance = Arc::new(TspInstance::from_tsplib_file(&args[1]).unwrap());
//     let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());
//
//     // let solver = solvers::basic_three_opt_local_search::build(tsp_instance.clone());
//     let solver = solvers::take_first_three_opt_local_search::build(tsp_instance.clone());
//     solver.solve(tour.clone());
// }
fn main() {
    // struct Solution {
    //     value: Vec<i64>,
    // }
    // impl Solution {
    //     fn new(value: Vec<i64>) -> Self {
    //         Solution { value }
    //     }
    //
    //     fn get(&self, index: usize) -> i64 {
    //         self.value[index]
    //     }
    //
    //     fn len(&self) -> usize {
    //         self.value.len()
    //     }
    // }
    struct Solution(Vec<i64>);

    use rapid_solve::objective::{BaseValue, Indicator, Objective};

    struct PermutationViolation;
    impl Indicator<Solution> for PermutationViolation {
        fn evaluate(&self, solution: &Solution) -> BaseValue {
            let violation: i64 = (0..solution.0.len())
                .map(|i| (solution.0.iter().filter(|&n| *n == i as i64).count() as i64 - 1).abs())
                .sum();
            BaseValue::Integer(violation)
        }
        fn name(&self) -> String {
            String::from("PermutationViolation")
        }
    }

    struct SquaredDifference;
    impl Indicator<Solution> for SquaredDifference {
        fn evaluate(&self, solution: &Solution) -> BaseValue {
            let squared_diff: i64 = (0..solution.0.len())
                .map(|i| (solution.0[i] - solution.0[(i + 1) % solution.0.len()]).pow(2))
                .sum();
            BaseValue::Integer(squared_diff)
        }
        fn name(&self) -> String {
            String::from("SquaredDifference")
        }
    }

    fn build_objective() -> Objective<Solution> {
        Objective::new_single_indicator_per_level(vec![
            Box::new(PermutationViolation),
            Box::new(SquaredDifference),
        ])
    }

    impl Solution {
        fn change_entry(&self, index: usize, new_value: i64) -> Self {
            let mut new_values = self.0.clone();
            new_values[index] = new_value;
            Solution(new_values)
        }

        fn swap_entries(&self, index1: usize, index2: usize) -> Self {
            let mut new_values = self.0.clone();
            new_values.swap(index1, index2);
            Solution(new_values)
        }
    }

    use rapid_solve::heuristics::local_search::Neighborhood;
    struct ChangeEntryThenSwapNeighborhood;
    impl Neighborhood<Solution> for ChangeEntryThenSwapNeighborhood {
        fn neighbors_of<'a>(
            &'a self,
            solution: &'a Solution,
        ) -> Box<dyn Iterator<Item = Solution> + Send + Sync + 'a> {
            let change_entry = (0..solution.0.len()).flat_map(move |i| {
                (0..10).map(move |new_value| solution.change_entry(i, new_value))
            });
            let swap_entries = (0..solution.0.len())
                .flat_map(move |i| (0..solution.0.len()).map(move |j| solution.swap_entries(i, j)));
            Box::new(change_entry.chain(swap_entries))
        }
    }

    use rapid_solve::heuristics::local_search::LocalSearchSolver;
    use std::sync::Arc;

    let objective = Arc::new(build_objective());
    let neighborhood = Arc::new(ChangeEntryThenSwapNeighborhood);
    let solver = LocalSearchSolver::initialize(neighborhood, objective);

    let initial_solution = Solution(vec![0; 10]);

    let evaluated_local_minimum = solver.solve(initial_solution);
    assert_eq!(
        *evaluated_local_minimum.objective_value().as_vec(),
        vec![BaseValue::Integer(0), BaseValue::Integer(36)]
    );
    assert_eq!(
        *evaluated_local_minimum.solution().0,
        vec![1, 0, 2, 4, 5, 7, 9, 8, 6, 3]
    );
    // one global optimum is [0, 2, 4, 6, 8, 9, 7, 5, 3, 1] with a squared difference of 34.
}
