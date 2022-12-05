mod solutions;

use std::fs;
use std::path::Path;
use std::time::Instant;

use solutions::attempt_one::first_solution;
use solutions::attempt_two::second_solution;

fn main() {
    let file_path = Path::new("./answer_2022_1/puzzle_copy.txt");

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    println!("Begin First Solution.");
    let now = Instant::now();

    let slow_sol = first_solution(contents.clone());
    
    let elapsed = now.elapsed();
    println!("\nSlow Solution Elapsed Time: {:.2?}", elapsed);

    println!("Begin Second Solution.");
    let now = Instant::now();

    let fast_sol = second_solution(contents);

    let elapsed = now.elapsed();
    println!("\nFast Solution Elapsed Time: {:.2?}", elapsed);

    println!("{}", slow_sol);
    println!("{}", fast_sol);
}
