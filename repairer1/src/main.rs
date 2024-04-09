pub use rand::Rng;

pub mod models;
pub mod mods;
use mods::*;
use std::{collections::HashMap, env, fs::File};

use crate::models::{Move, MovementAlgorithm, Repairer};
fn main() {
    // clear_terminal();
    // creating a square matrix of the atomic bool's
    // rows and columns are pre-known for this specific example
    let rows: u8 = 7;
    let columns: u8 = 7;
    // let total: u8 = rows * columns;
    // generating some random indexes to be chosen as the broken elements, the value of the broken elements are 11 while the normal ones are 0
    let total_broken: Vec<String> = env::args().collect();
    // generating random rows for repairers
    let repairer_rows = gen_rand_index(1, 0, rows as i32);
    // generating random columns for repairers
    let repairer_columns = gen_rand_index(1, 0, columns as i32);

    // creating the initializing algorithms
    let init_algos: Vec<MovementAlgorithm> = vec![
        MovementAlgorithm::BRD,
        MovementAlgorithm::DDL,
        MovementAlgorithm::BLU,
        MovementAlgorithm::DUR,
    ];

    let mut repairs_track = HashMap::new();
    for i in 1..5 {
        repairs_track.insert(i, 0);
    }

    // creating the repairers state
    let mut state: Repairer = Repairer {
        id: 1,
        thread: None,
        total_broken: total_broken[1].parse().unwrap(),
        total_fixed: 0,
        other_repairers_repairs: repairs_track,
        total_moves: 0,
        current_algorithm: init_algos[0 as usize].clone(),
        current_location: (
            repairer_rows[0 as usize] as u8,
            repairer_columns[0 as usize] as u8,
        ),
        matrix_size: rows as u8,
        decision: Move::Empty,
        move_turn: true, // means the first move
        last_move_rotated: false,
        last_move: Move::Empty,
        result: "".to_string(),
    };

    // start
    // @param dead_repairers will be used to check the end of the repairing progress.

    // there is two while loops
    // the first one is the main one is the check for the end of the progress and the inner nested one is for confirming the decision making.

    while !check_dead_repairers() {
        if state.decision == Move::None {
            // this process is done but other are not, storing the result and ending this one
            break;
        }

        // thread::sleep(Duration::from_millis(50));

        // making decision
        make_decision(&mut state);

        // decision barrier
        let _ = decision_barrier();

        // executing
        execute(&mut state);

        // executing barrier
        let _ = execute_barrier();
    }
}
