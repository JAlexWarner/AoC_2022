use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("./answer_2022_2/aoc_22_2_input.txt");

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let rounds = contents.split("\n");
    let str_vec: Vec<&str> = rounds.collect();

    let mut final_score = 0;
    for r in str_vec {
        let pov_choice = r.chars().nth(2).unwrap();
        let elf_choice = r.chars().nth(0).unwrap();

        let final_points = match pov_choice {
            'X' => 1 + match elf_choice {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => panic!("Dear god!"),
            },
            'Y' => 2 + match elf_choice {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => panic!("Dear god!"),
            },
            'Z' => 3 + match elf_choice {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => panic!("Dear god!"),
            },
            _ => panic!("Dear god!")
            
        };
        
        println!("{}", final_points);
        final_score += final_points;
    }
    println!("{}", final_score);
}
