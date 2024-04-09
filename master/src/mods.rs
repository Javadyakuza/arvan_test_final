use std::{io::BufReader, process::Command};

use memmap2::{Mmap, MmapMut};
use rand::{prelude::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::io::Read;
#[derive(Deserialize, Serialize, Debug)]
pub struct Matrix {
    pub data: Vec<Vec<(Vec<String>, u8)>>,
}
pub fn gen_rand_index(amount: i32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (min..max).collect();
    numbers.shuffle(&mut rng);
    numbers.iter().take(amount as usize).cloned().collect()
}

pub fn init_matrix(rows: u8, cols: u8) -> Matrix {
    let total_broken: i32 = rand::thread_rng().gen_range(3..7);

    // <total_broken> rows for broken houses
    let broken_rows = gen_rand_index(total_broken, 0, rows as i32);

    // <total_broken> column for broken houses
    let broken_columns = gen_rand_index(total_broken, 0, cols as i32);

    // Creating the matrix

    let mut matrix: Matrix = Matrix {
        data: Vec::with_capacity(rows as usize),
    };

    for _ in 0..rows {
        let mut row: Vec<(Vec<String>, u8)> = Vec::with_capacity(cols as usize);

        for _ in 0..cols {
            let tmp_notes = vec![
                "0 repaired 0 times".to_string(),
                "1 repaired 0 times".to_string(),
                "2 repaired 0 times".to_string(),
                "3 repaired 0 times".to_string(),
            ];
            row.push((tmp_notes, 0)); // Initialize all elements to false
        }
        matrix.data.push(row);
    }

    // adding the broken houses
    for i in 0..total_broken {
        let row_idx = broken_rows[i as usize] as usize;
        let col_idx = broken_columns[i as usize] as usize;

        matrix.data[row_idx][col_idx].1 = 244; // only increment is possible
    }

    matrix
}

pub fn decode_matrix(m_matrix: &Mmap) -> Matrix {
    let mut contents: Vec<u8> = Vec::new();
    let _ = BufReader::new(&m_matrix[..])
        .read_to_end(&mut contents)
        .unwrap();

    bincode::deserialize(&contents).unwrap()
}

pub fn decode_pt(m_pt: &Mmap) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::new();
    let _ = BufReader::new(&m_pt[..])
        .read_to_end(&mut contents)
        .unwrap();
    contents
}

pub fn decode_fs(m_fs: &Mmap) -> String {
    let mut contents: Vec<u8> = Vec::new();
    let _ = BufReader::new(&m_fs[..])
        .read_to_end(&mut contents)
        .unwrap();
    String::from_utf8_lossy(&contents).to_string()
}

pub fn print_matrix(matrix: Vec<Vec<(Vec<String>, u8)>>, repairers: Vec<u8>) {
    println!("   repairer 1    |    repairer 2    |    repairer 3    |    repairer 4    ");
    println!(
        "     {:?}             {:?}            {:?}            {:?}        ",
        repairers[0..2].to_vec(),
        repairers[2..4].to_vec(),
        repairers[4..6].to_vec(),
        repairers[6..8].to_vec()
    );
    println!();
    for row in matrix.iter() {
        for element in row.iter() {
            print!("{:?} | ", element.1);
        }
        println!();
        for _ in row.iter() {
            print!("-   ");
        }
        println!();
    }
}

pub fn clear_terminal() {
    let _ = Command::new("clear")
        .status()
        .expect("failed to execute ls");
}
