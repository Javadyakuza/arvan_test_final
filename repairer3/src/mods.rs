use crate::models::{Matrix, Move, Note, Repairer};
use memmap2::{MmapMut};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::{ OpenOptions};
use std::io::{BufReader, Read, Write};
use std::{
    ops::Add,
    process::Command,
};
use bincode::serialize;

pub fn gen_rand_index(amount: i32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (min..max).collect();
    numbers.shuffle(&mut rng);
    numbers.iter().take(amount as usize).cloned().collect()
}

pub fn make_decision(state: &mut Repairer) -> bool {
    // reading the matrix
    // Create a file and map it in read/write mode for both programs
    let matrix_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("/home/javadyakuza/Desktop/arvan_test_final/matrix.dat")
        .unwrap();

    let m_matrix: MmapMut = unsafe { MmapMut::map_mut(&matrix_file) }.unwrap();

    println!("{:?}",decode_matrix(&m_matrix));

    let decision_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("/home/javadyakuza/Desktop/arvan_test_final/decision.dat")
        .unwrap();

    let mut m_decision: MmapMut = unsafe { MmapMut::map_mut(&decision_file) }.unwrap();

    let mut rng = thread_rng();
    let mut n_move: Move = state.current_algorithm.get_move(rng.gen_bool(1.0 / 3.0));
    if state.last_move_rotated {
        n_move = state.last_move.clone();                                           
        state.last_move_rotated = false;
    }
    // checking the conditions that may change the algo

    // checking the current index status -> might change to Move::Fix
    let matrix = decode_matrix(&m_matrix);
    if matrix.data[state.current_location.0 as usize][state.current_location.1 as usize].1 == 11 {
        n_move = Move::Fix;

        state.decision = n_move.clone();

        if state.last_move_rotated {
            state.last_move_rotated = false;
        }
        println!("modifying");
        let tmp_des = String::from_utf8_lossy(&m_decision[..]).parse::<u8>().unwrap() + 1;
        m_decision[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
        m_decision.flush().unwrap();
        return true; // return true because the first priority is the fixing
    }

    // reading the notes // might change to Move::None
    for (note_idx, note) in matrix.data[state.current_location.0 as usize]
        [state.current_location.1 as usize]
        .0
        .iter()
        .enumerate()
    {
        let num_repairs = Note::parse(&note)
        .num_repairs;
        // checking with the previous value of the states value
        if **(state
            .other_repairers_repairs    
            .get(&((note_idx + 1) as u32))
            .as_ref()
            .unwrap())
            < num_repairs // the number of the fixes can not be reduced so != will do the job and there is no need for greater and smaller than sign.
            &&
            // the current threads state was updated in the last round of the execute function
            (note_idx + 1) as u32 != state.id
        {
            // updating the specific state total repairs
            state
                .other_repairers_repairs
                .insert(note_idx as u32, num_repairs)
                .unwrap();
        }
    }
    if state.get_total_fixes_from_notes() == state.total_broken {
        n_move = Move::None;
        state.decision = n_move.clone();
        println!("modifying");
        let tmp_des = String::from_utf8_lossy(&m_decision[..]).parse::<u8>().unwrap() + 1;
        m_decision[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
        m_decision.flush().unwrap();
        return true;
    }

    // checking the index // might rotate tha algo
    // case 1 => corners
    let corners = [
        (0, 0),
        (0, state.matrix_size - 1),
        (state.matrix_size - 1, state.matrix_size - 1),
        (state.matrix_size - 1, 0),
    ];
    if state.current_location == corners[0] && (n_move == Move::Left || n_move == Move::Up) {
        // updating the threads state
        state.current_algorithm.rotate_algo(&n_move);
        state.last_move_rotated = true;
        n_move.rotate_dir();
        state.last_move = n_move.clone();
        state.decision = n_move.clone();
    } else if state.current_location == corners[1] && (n_move == Move::Right || n_move == Move::Up)
    {
        // updating the threads state
        state.current_algorithm.rotate_algo(&n_move);
        state.last_move_rotated = true;
        n_move.rotate_dir();
        state.last_move = n_move.clone();
        state.decision = n_move.clone();
    } else if state.current_location == corners[2]
        && (n_move == Move::Right || n_move == Move::Down)
    {
        // updating the threads state
        state.current_algorithm.rotate_algo(&n_move);
        state.last_move_rotated = true;
        n_move.rotate_dir();
        state.last_move = n_move.clone();
        state.decision = n_move.clone();
    } else if state.current_location == corners[3] && (n_move == Move::Left || n_move == Move::Down)
    {
        // updating the threads state
        state.current_algorithm.rotate_algo(&n_move);
        state.last_move_rotated = true;
        n_move.rotate_dir();
        state.last_move = n_move.clone();
        state.decision = n_move.clone();
    } else {
        if !state.last_move_rotated {
            // case 2 => edges
            if n_move.is_horizontal() {
                // checking the right and the left edges
                // checking if the col value is 0 or <matrix_size - 1>
                if state.current_location.1 == 0 {
                    // on the left edge, changing if next move is left
                    if n_move == Move::Left {
                        state.current_algorithm.rotate_algo(&n_move);
                        n_move.rotate_dir();
                        state.decision = n_move.clone();
                    }
                } else if state.current_location.1 == state.matrix_size - 1 {
                    // on the right edge, changing if next move is right
                    if n_move == Move::Right {
                        state.current_algorithm.rotate_algo(&n_move);
                        n_move.rotate_dir();
                        state.decision = n_move.clone();
                    }
                }
            } else {
                // checking the bottom and the top edges
                // checking if the row value is 0 or <matrix_size - 1>
                if state.current_location.0 == 0 {
                    // on the upper edge, changing if next move is up
                    if n_move == Move::Up {
                        state.current_algorithm.rotate_algo(&n_move);
                        n_move.rotate_dir();
                        state.decision = n_move.clone();
                    }
                } else if state.current_location.0 == state.matrix_size - 1 {
                    // on the bottom edge, changing if next move is down
                    if n_move == Move::Down {
                        state.current_algorithm.rotate_algo(&n_move);
                        n_move.rotate_dir();
                        state.decision = n_move.clone();
                    }
                }
            }
        }
    }
    // sending the confirmation

    if state.decision == Move::Empty {
        state.decision = n_move.clone();
    }
    println!("modifying 3");
    println!(" m_decision raw {:?}, - {:?}", String::from_utf8_lossy(&m_decision[..]),&m_decision[..]);
    let tmp_des = String::from_utf8_lossy(&m_decision[..]).parse::<u8>().unwrap() + 1;
    m_decision[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
    m_decision.flush().unwrap();
    println!(" m_decision {:?} tmp {}", &m_decision[..], tmp_des);

    true // incase none of the if clauses returned the true we do the least move detected above by the <Move::get_move()> fn.
}

pub fn execute(
    state: &mut Repairer,
) -> bool {
    let matrix_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("/home/javadyakuza/Desktop/arvan_test_final/matrix.dat")
        .unwrap();

    let mut m_matrix: MmapMut = unsafe { MmapMut::map_mut(&matrix_file) }.unwrap();

    let execute_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(false)
    .open("/home/javadyakuza/Desktop/arvan_test_final/execute.dat")
    .unwrap();

    let mut m_execute: MmapMut = unsafe { MmapMut::map_mut(&execute_file) }.unwrap();
    
    let results_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(false)
    .open("/home/javadyakuza/Desktop/arvan_test_final/final_results.dat")
    .unwrap();

    let mut m_results: MmapMut = unsafe { MmapMut::map_mut(&results_file) }.unwrap();

    let processes_track_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(false)
    .open("/home/javadyakuza/Desktop/arvan_test_final/processes_track.dat")
    .unwrap();

    // its an 8 byte lengthen array, each two index storing the each process's current location
    let mut m_ptf: MmapMut = unsafe { MmapMut::map_mut(&processes_track_file) }.unwrap();

    let final_statistics_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(false)
    .open("/home/javadyakuza/Desktop/arvan_test_final/final_statistics.dat")
    .unwrap();           


    // its an 8 byte lengthen array, each two index storing the each process's current location
    let mut m_fsf: MmapMut = unsafe { MmapMut::map_mut(&final_statistics_file) }.unwrap();

    // let mut matrix = decode_matrix(&m_file);
    // applying the move
    match state.decision {
        Move::Empty => panic!("decision making round didn't make any decisions"),
        Move::None => {
            let tmp_des = String::from_utf8_lossy(&m_results[..]).parse::<u8>().unwrap() + 1;
            m_results[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
            m_results.flush().unwrap();
            state.total_moves += 1;
            state.result = format!("repairer id: {}, repairs: {}, moves: {}, all_players_repairs: {:?}, goal: {} \n", state.id, state.total_fixed, state.total_moves, state.other_repairers_repairs, state.total_broken);
            m_fsf[((state.id - 1) as usize  * state.result.as_bytes().len() as usize)..state.result.as_bytes().len() as usize].copy_from_slice(state.result.as_bytes());
            m_fsf.flush().unwrap();
            false
        }
        Move::Fix => {
            // move is fix
            // fixing
            let status = get_status_index(state.current_location, 18, state.matrix_size);
            match m_matrix[status].checked_add(1) {
                Some(_) => {
                    m_matrix[status..(status+1)].copy_from_slice(&[255]);
                    m_matrix.flush().unwrap();
                    state.total_fixed = state.total_fixed.add(1);
                    state.other_repairers_repairs.insert(state.id, state.total_fixed).unwrap();
                }, 
                None => {
                    // couldn't fix not updating the state, some other repairer updated.
                }
            } 



            // updating the decision
            state.decision = Move::Empty.clone();
            // adding the total moves
            state.total_moves = state.total_moves.add(1);

            // leaving the note

            let (start, end) = get_note_index(state.current_location,state.id as u8 - 1, 18, state.matrix_size);
            m_matrix[start..end].copy_from_slice(format!("{} repaired {} times", state.id, state.total_fixed).as_bytes());
            m_matrix.flush().unwrap();

            let tmp_des = String::from_utf8_lossy(&m_execute[..]).parse::<u8>().unwrap() + 1;
            m_execute[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
            m_execute.flush().unwrap();
            // updating the move turn
            state.move_turn = !state.move_turn;

            true
        }
        _ => {
            // move is actual move, changing the thread state

            // updating the current location
            state.current_location = state
                .decision
                .apply_on_index(state.current_location);

            m_ptf[(state.id - 1) as usize..(state.id + 1) as usize].copy_from_slice(&[state.current_location.0, state.current_location.1]);
            m_ptf.flush().unwrap();

            // updating the decision
            state.decision = Move::Empty.clone();

            //updating the move turn
            state.move_turn = !state.move_turn;

            // adding the total moves
            state.total_moves = state.total_moves.add(1);
            let (start, end) = get_note_index(state.current_location,state.id as u8 - 1, 18, state.matrix_size);
            m_matrix[start..end].copy_from_slice(format!("{} repaired {} times", state.id, state.total_fixed).as_bytes());
            m_matrix.flush().unwrap();

            let tmp_des = String::from_utf8_lossy(&m_execute[..]).parse::<u8>().unwrap() + 1;
            m_execute[..].copy_from_slice(format!("{}",tmp_des).as_bytes());
            m_execute.flush().unwrap();
            true
        }
    }
}

// pub fn clear_terminal() {
//     let _ = Command::new("clear")
//         .status()
//         .expect("failed to execute ls");
// }

pub fn decode_matrix(m_matrix: &MmapMut) -> Matrix {
    let mut contents: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(&m_matrix[..])
        .read_to_end(&mut contents)
        .unwrap();

        bincode::deserialize(&contents).unwrap()
    }


pub fn encode_matrix(matrix: Matrix) -> Vec<u8> {

    serialize(&matrix).unwrap()
    
}


pub fn get_note_index(
    element_idx: (u8, u8),
    repairer_id: u8, // 1, 2, 3, 4
    note_len: u8,
    size: u8,
) -> (usize, usize) {
    let notes_num = 4;
    let buffers_starting_nulls = 3 * 8;
    let mut passing_rows_indent = 0;
    if element_idx.0 != 0 {
        passing_rows_indent = element_idx.0;
    }
    let mut passing_cols_indent = 0;
    if element_idx.1 != 0 {
        passing_cols_indent = element_idx.1;
    }
    let passing_rows = passing_rows_indent as u32
        * (size as u32 * ((notes_num as u32 * (note_len as u32 + 8_u32)) + 9_u32)); // problem on this line
    let passing_cols =
        passing_cols_indent as u32 * ((notes_num as u32 * (note_len as u32 + 8_u32)) + 9_u32); // problem on this line
    println!("{}-{}", passing_rows, passing_cols);
    let num_rows_dis = element_idx.0 as u32 * 8_u32;
    let start = buffers_starting_nulls as u32
        + passing_rows as u32
        + passing_cols as u32
        + num_rows_dis as u32
        + ((repairer_id as u32 * (note_len as u32 + 8_u32)) + 8_u32);
    let end = start + note_len as u32;
    (start as usize, end as usize)
}

pub fn get_status_index(element_idx: (u8, u8), note_len: u8, size: u8) -> usize {
    let notes_num = 4;
    let buffers_starting_nulls = 3 * 8;
    let mut passing_rows_indent = 0;
    if element_idx.0 != 0 {
        passing_rows_indent = element_idx.0;
    }
    let mut passing_cols_indent = 0;
    if element_idx.1 != 0 {
        passing_cols_indent = element_idx.1;
    }
    let passing_rows = passing_rows_indent as u32
        * (size as u32 * ((notes_num as u32 * (note_len as u32 + 8_u32)) + 9_u32)); // problem on this line
    let passing_cols =
        passing_cols_indent as u32 * ((notes_num as u32 * (note_len as u32 + 8_u32)) + 9_u32); // problem on this line
    println!("{}-{}", passing_rows, passing_cols);
    let num_rows_dis = element_idx.0 as u32 * 8_u32;
    let status = buffers_starting_nulls as u32
        + passing_rows as u32
        + passing_cols as u32
        + num_rows_dis as u32
        + (notes_num as u32 * (note_len as u32 + 8_u32));
    status as usize
}

pub fn check_dead_repairers() -> bool {
        // Create a file and map it in read/write mode for both programs
        let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("/home/javadyakuza/Desktop/arvan_test_final/final_results.dat")
        .unwrap();    

    let m_file = unsafe { MmapMut::map_mut(&file) }.unwrap();

    String::from_utf8_lossy(&m_file[..]).parse::<u8>().unwrap() == 4
}

pub fn decision_barrier() -> bool {
    // Create a file and map it in read/write mode for both programs
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(false)
    .open("/home/javadyakuza/Desktop/arvan_test_final/decision.dat")
    .unwrap();

    let mut m_file = unsafe { MmapMut::map_mut(&file) }.unwrap();
    // Create a file and map it in read/write mode for both programs
let execute_file = OpenOptions::new()
.read(true)
.write(true)
.create(false)
.open("/home/javadyakuza/Desktop/arvan_test_final/execute.dat")
.unwrap();

let mut m_execute = unsafe { MmapMut::map_mut(&execute_file) }.unwrap();
if m_execute[0] != 0 {
    m_execute[0..1].copy_from_slice("0".as_bytes());
    m_execute.flush().unwrap();
}
    let mut all_made_des = false;

    while !all_made_des {
        if String::from_utf8_lossy(&m_file[..]).parse::<u8>().unwrap() == 4 {
            all_made_des = true;
        }
    }

    all_made_des
    // todo!("must set the exe"); and flush it 

}
pub fn execute_barrier() -> bool {
    println!("in the execute barrier");
// Create a file and map it in read/write mode for both programs
let file = OpenOptions::new()
.read(true)
.write(true)
.create(false)
.open("/home/javadyakuza/Desktop/arvan_test_final/execute.dat")
.unwrap();

let m_file = unsafe { MmapMut::map_mut(&file) }.unwrap();

// Create a file and map it in read/write mode for both programs
let decision_file = OpenOptions::new()
.read(true)
.write(true)
.create(false)
.open("/home/javadyakuza/Desktop/arvan_test_final/decision.dat")
.unwrap();

let mut m_decision = unsafe { MmapMut::map_mut(&decision_file) }.unwrap();
if String::from_utf8_lossy(&m_decision[..]).parse::<u8>().unwrap() != 0 {
    m_decision[..].copy_from_slice("0".as_bytes());
        m_decision.flush().unwrap();
}
let mut all_executed = false;

while !all_executed {
    if String::from_utf8_lossy(&m_file[..]).parse::<u8>().unwrap() == 4 {
        all_executed = true;
    }
}
    all_executed
    // todo!("must set the des"); and flush it

}