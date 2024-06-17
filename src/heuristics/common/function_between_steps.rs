//! This module contains the FunctionBetweenSteps type which is used to define a function that is
//! called between steps of a solver.

use std::sync::Arc;
use std::time as stdtime;
use std::time::Instant;

use crate::objective::{EvaluatedSolution, Objective};
/// function between steps with the following signature:
/// * iteration counter
/// * current solution
/// * previous solution (Option)
/// * objective
/// * time that local search started (Option)
/// * time limit (Option)
/// * iteration limit (Option)
pub type FunctionBetweenSteps<S> = Box<
    dyn Fn(
        u32,
        &EvaluatedSolution<S>,
        Option<&EvaluatedSolution<S>>,
        Arc<Objective<S>>,
        Option<Instant>,
        Option<stdtime::Duration>,
        Option<u32>,
    ),
>;

pub fn default<S>() -> FunctionBetweenSteps<S> {
    Box::new(
        |iteration,
         current_solution,
         previous_solution,
         objective,
         start_time,
         time_limit,
         iteration_limit| {
            println!("\nIteration {}:", iteration,);
            match previous_solution {
                Some(prev_solution) => {
                    objective.print_objective_value_with_comparison(
                        current_solution.objective_value(),
                        prev_solution.objective_value(),
                    );
                }
                None => {
                    objective.print_objective_value(current_solution.objective_value());
                }
            }
            if let Some(start_time) = start_time {
                println!(
                    "elapsed time for local search: {:0.2}sec",
                    stdtime::Instant::now()
                        .duration_since(start_time)
                        .as_secs_f32(),
                );
            }
            if time_limit.is_some() || iteration_limit.is_some() {
                println!(
                    "({}{}{})",
                    match iteration_limit {
                        Some(iteration_limit) => format!("iteration limit: {}", iteration_limit),
                        None => "".to_string(),
                    },
                    if time_limit.is_some() && iteration_limit.is_some() {
                        ", "
                    } else {
                        ""
                    },
                    match time_limit {
                        Some(time_limit) =>
                            format!("time limit: {:0.2}sec", time_limit.as_secs_f32()),
                        None => "".to_string(),
                    },
                );
            }
        },
    )
}
