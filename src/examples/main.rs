use std::env;
use std::sync::Arc;

use rapid_solve::examples::tsp::solvers;
use rapid_solve::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(args[0].as_str());
        std::process::exit(1);
    }

    let tsp_instance = Arc::new(TspInstance::from_tsplib_file(&args[2]).unwrap());
    let tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());

    let solver = match args[1].as_str() {
        "basic" => solvers::basic_three_opt_local_search::build(tsp_instance.clone()),
        "take_first" => solvers::take_first_three_opt_local_search::build(tsp_instance.clone()),
        _ => {
            eprintln!("Unknown solver: {}", args[1]);
            print_usage(args[0].as_str());
            std::process::exit(1);
        }
    };
    solver.solve(tour.clone());
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <solver> <tsplib_file>", program_name);
    eprintln!("  <solver>: basic | take_first");
    std::process::exit(1);
}
