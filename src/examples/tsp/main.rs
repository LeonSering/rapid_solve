use std::env;
use std::sync::Arc;

use rapid_solve::examples::tsp::solvers;
use rapid_solve::examples::tsp::tsp_tour_with_info::TspTourWithInfo;
use rapid_solve::examples::tsp::{tsp_instance::TspInstance, tsp_tour::TspTour};
use rapid_solve::heuristics::Solver;

/// With this main function, you can run a TSP solver with a provided TSPLIB file.
fn main() {
    let args: Vec<String> = env::args().collect();
    let start_time = std::time::Instant::now();

    if args.len() != 3 {
        print_usage(args[0].as_str());
        std::process::exit(1);
    }

    let tsp_instance = Arc::new(TspInstance::from_tsplib_file(&args[2]).unwrap());
    let initial_tour = TspTour::from_instance_nearest_neighbor(tsp_instance.clone());

    let final_tour = match args[1].as_str() {
        "basic_local_search" => {
            let basic_local_search_solver =
                Box::new(solvers::basic_local_search::build(tsp_instance));
            basic_local_search_solver.solve(initial_tour).unwrap()
        }

        "take_first_local_search" => {
            let take_first_local_search_solver =
                Box::new(solvers::take_first_local_search::build(tsp_instance));
            take_first_local_search_solver.solve(initial_tour).unwrap()
        }
        "threshold_accepting" => {
            let threshold_accepting_solver =
                Box::new(solvers::threshold_accepting::build(tsp_instance));
            let tsp_tour_with_info =
                threshold_accepting_solver.solve(TspTourWithInfo::new(initial_tour, 0));
            tsp_tour_with_info.unwrap().unwrap()
        }
        "simulated_annealing" => {
            let simulated_annealing_solver =
                Box::new(solvers::simulated_annealing::build(tsp_instance));
            let tsp_tour_with_info =
                simulated_annealing_solver.solve(TspTourWithInfo::new(initial_tour, 0));
            tsp_tour_with_info.unwrap().unwrap()
        }
        "tabu_search" => {
            let tabu_search_solver = Box::new(solvers::tabu_search::build(tsp_instance));
            tabu_search_solver.solve(initial_tour).unwrap()
        }
        "parallel_tabu_search" => {
            let parallel_tabu_search_solver =
                Box::new(solvers::parallel_tabu_search::build(tsp_instance));
            parallel_tabu_search_solver.solve(initial_tour).unwrap()
        }
        _ => {
            eprintln!("Unknown solver: {}", args[1]);
            print_usage(args[0].as_str());
            std::process::exit(1);
        }
    };

    println!("\nFinal tour: {:?}", final_tour.get_nodes());
    println!("Total distance: {:0.2}", final_tour.get_total_distance());
    println!(
        "\nRunning time: {:0.2}sec",
        start_time.elapsed().as_secs_f64()
    );
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} <solver> <tsplib_file>", program_name);
    eprintln!("  <solver>: basic | take_first");
}
