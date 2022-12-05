use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("./full_puzzle.txt");

    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let rucksacks = contents.split("\n");

    let mut total_priority = 0;

    for rs in rucksacks {
        let first_half: &str = &rs[0..rs.len() / 2];

        let mut duplicate = '-';

        for item in first_half.chars() {
            if rs[rs.len() / 2..].contains(item) {
                // println!("{}", item.to_string());
                duplicate = item;
                break;
            }
        }

        let alpha_position = ('a'..='z').into_iter().position(|x| x == duplicate);

        let priority = match alpha_position {
            Some(index) => index + 1,
            None => {
                ('A'..='Z')
                    .into_iter()
                    .position(|y| y == duplicate)
                    .unwrap()
                    + 27
            }
        };

        total_priority += priority;
    }

    println!("{}", total_priority.to_string());
}
