use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("./example_puzzle.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let pairs = contents.split("\n");

    let mut total_overlaps = 0;

    for single_pair in pairs {
        let mut characters = single_pair.chars();

        let a1 = characters.nth(0);
        let a2 = characters.nth(1);
        let b1 = characters.nth(1);
        let b2 = characters.nth(1);

        println!("{:?}, {:?}, {:?}, {:?}", a1, a2, b1, b2);

        if a1 <= b1 {
            if b2 <= a2 {
                println!("Yup");
                total_overlaps += 1;
            }
        } else {
            if a2 <= b2 {
                println!("Yup");
                total_overlaps += 1;
            }
        }
    }

    println!("{}", total_overlaps.to_string());

    let response = reqwest::blocking::get(
        "https://adventofcode.com/2022/day/4/input",
    )
    .unwrap()
    .text()
    .unwrap();
}


// pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
//     let mut parser = map_res(digit1, u32::from_str);
//     parser(input)
// }

// use nom::sequence::tuple;
// use nom::character::complete::{alpha1, digit1};
// let mut parser = tuple((alpha1, digit1, alpha1));