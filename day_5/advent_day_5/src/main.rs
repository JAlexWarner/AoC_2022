use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::error::Error;
use nom::sequence::tuple;
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("src/day_5_input.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let mut columns: Vec<Vec<&str>> = vec![
        vec!["H", "R", "B", "D", "Z", "F", "L", "S"],
        vec!["T", "B", "M", "Z", "R"],
        vec!["Z", "L", "C", "H", "N", "S"],
        vec!["S", "C", "F", "J"],
        vec!["P", "G", "H", "W", "R", "Z", "B"],
        vec!["V", "J", "Z", "G", "D", "N", "M", "T"],
        vec!["G", "L", "N", "W", "F", "S", "P", "Q"],
        vec!["M", "Z", "R"],
        vec!["M", "C", "L", "G", "V", "R", "T"],
    ];

    let mut parser = tuple::<&str, _, _, _>((
        tag("move "),
        digit1::<&str, Error<&str>>,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ));

    for instruction in contents.lines() {
        let parser_output = parser(instruction);

        match parser_output {
            Ok(output) => {
                let parsed_output = (
                    output.1 .1.parse::<usize>().unwrap(),
                    output.1 .3.parse::<usize>().unwrap(),
                    output.1 .5.parse::<usize>().unwrap(),
                );
                let original_length = columns[parsed_output.1 - 1].len();

                let temp_crates = &columns[parsed_output.1 - 1][original_length - parsed_output.0..original_length].to_vec();
                columns[parsed_output.2 - 1].extend(temp_crates[..].into_iter().rev());
                columns[parsed_output.1 - 1] = columns[parsed_output.1 - 1][..original_length - parsed_output.0].to_vec();
            }
            Err(_) => continue,
        };
    }

    println!("{:?}", columns)
}
