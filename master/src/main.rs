use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    // process::Command,
    str::Bytes,
    thread,
    time::Duration,
};
mod mods;
use bincode::serialize;
use memmap2::{Mmap, MmapMut};
use mods::*;

use tokio::{main, process::Command};

#[tokio::main]
async fn main() {
    //checking the command
    let args: Vec<String> = env::args().collect();

    let matrix_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/matrix.dat");
    let des_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/decision.dat");
    let exe_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/execute.dat");
    let fr_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/final_results.dat");
    let fs_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/final_statistics.dat");
    let pt_path: String = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace("/master", "/processes_track.dat");
    if args[1] == "start".to_string() {
        println!(
            "{}",
            env::current_dir().unwrap().to_str().unwrap().to_owned() + "/matrix.dat"
        );

        // initialize the project
        let mut matrix_file = File::create_new(matrix_path).unwrap();

        //  initializing the matrix
        let _ = matrix_file.write_all(&(serialize(&init_matrix(7, 7)).unwrap()));
        let mut des_file = File::create_new(des_path).unwrap();
        des_file.write_all("0".as_bytes()).unwrap();
        let mut exe_file = File::create_new(exe_path).unwrap();
        exe_file.write_all("0".as_bytes()).unwrap();
        let mut fr_file = File::create_new(fr_path).unwrap();
        fr_file.write_all("0".as_bytes()).unwrap();
        let mut fs_file = File::create_new(fs_path).unwrap();
        fs_file.write_all(&Vec::with_capacity("repairer id: 255, repairs: 255, moves: 255, all_players_repairs: (255, 255, 255, 255), goal: 255 \n ".as_bytes().len() * 4)).unwrap();
        let mut pt_file = File::create_new(pt_path).unwrap();
        pt_file.write_all(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        println!("project initialized successfully");
        // running the processes
        // for i in 1..5 {
        // println!("running {}", i);
        thread::spawn(|| process_runner);

        // }

        let m_results = unsafe { Mmap::map(&fr_file) }.unwrap();

        let m_matrix = unsafe { Mmap::map(&matrix_file) }.unwrap();

        let m_pt = unsafe { Mmap::map(&pt_file) }.unwrap();

        let m_fs = unsafe { Mmap::map(&fs_file) }.unwrap();

        while m_results[0] != 4 {
            // printing the process
            clear_terminal();

            print_matrix(decode_matrix(&m_matrix).data, decode_pt(&m_pt));
            thread::sleep(Duration::from_millis(50));
        }

        // printing the results when the processes are done
        println!("{}", decode_fs(&m_fs));
    } else if args[1] == "re-init".to_string() {
        // reinitialize the project
        let mut matrix = File::create(matrix_path).unwrap();
        let _ = matrix.write_all(&(serialize(&init_matrix(7, 7)).unwrap()));
        let mut des = File::create(des_path).unwrap();
        let mut exe = File::create(exe_path).unwrap();
        let mut fr = File::create(fr_path).unwrap();
        let mut fs = File::create(fs_path).unwrap();
        let mut pt = File::create(pt_path).unwrap();
        des.write_all("0".as_bytes()).unwrap();
        exe.write_all("0".as_bytes()).unwrap();
        fr.write_all("0".as_bytes()).unwrap();
        fs.write_all(&Vec::with_capacity("repairer id: 255, repairs: 255, moves: 255, all_players_repairs: (255, 255, 255, 255), goal: 255 \n ".as_bytes().len() * 4)).unwrap();
        pt.write_all(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        println!("project re-initialized successfully");
    } else if args[1] == "clear".to_string() {
        fs::remove_file(matrix_path).unwrap();
        fs::remove_file(des_path).unwrap();
        fs::remove_file(exe_path).unwrap();
        fs::remove_file(fr_path).unwrap();
        fs::remove_file(fs_path).unwrap();
        fs::remove_file(pt_path).unwrap();
    } else {
        println!("no flag specified");
        return;
    }
}

pub async fn process_runner() {
    let _ = Command::new("sh")
        .arg("/home/javadyakuza/Desktop/arvan_test_final/master/master_runner.sh")
        .output()
        .await;
}
