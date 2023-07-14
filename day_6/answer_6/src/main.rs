use itertools::Itertools;
use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("src/input.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    for start in 5..4096 {
        println!("{}", &contents[start - 5..=start - 2]);

        let current_string = &contents[start - 5..=start - 2];

        if current_string
            .as_bytes()
            .into_iter()
            .unique()
            .collect::<Vec<_>>()
            .len()
            == 4
        {
            println! {"{}", start-1};
            break;
        }
    }
}