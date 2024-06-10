use std::env;
use std::sync::Arc;

use rapid_solve::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};

use rapid_solve::examples::tsp::solvers;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <tsplib_file>", args[0]);
        std::process::exit(1);
    }

    let tsp_instance = Arc::new(TspInstance::from_tsplib_file(&args[1]).unwrap());
    let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());

    // let solver = solvers::basic_three_opt_local_search::build(tsp_instance.clone());
    let solver = solvers::take_first_three_opt_local_search::build(tsp_instance.clone());
    solver.solve(tour.clone());
}
