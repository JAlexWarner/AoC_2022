use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("src/input.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let mut directory_vec: Vec<String> = vec!["".to_string()];

    let mut directory_sizes: HashMap<String, i32> = Default::default();
    let mut file_no_repeat: Vec<&str> = vec![];

    let digit_parser = digit1::<&str, Error<&str>>;
    let cd_parser = tag::<&str, &str, Error<&str>>("$ cd ");

    for cmd_line in &contents.lines().collect::<Vec<_>>()[1..] {
        println!("{:?}", cmd_line);
        if cmd_line == &"$ ls" {
            continue;
        }
        if &cmd_line[0..=2] == "dir" {
            continue;
        }
        // println!("{:?}", cd_parser(cmd_line));
        match cd_parser(cmd_line) {
            Ok(x) if x.0 == ".." => {
                directory_vec.pop();
                // println!("We're popping");
                ()
            }
            Ok(x) => {
                let mut new_directory = directory_vec.last().unwrap().to_string().to_owned();
                new_directory.push_str("/");
                new_directory.push_str(x.0);
                directory_vec.push(new_directory);
                // println!("We're pushing");
            }
            Err(_) => {
                let d_parser_out = digit_parser(cmd_line).unwrap();
                for ind_directory in &directory_vec {
                    if !file_no_repeat.contains(&&d_parser_out.0[1..]) {
                        *directory_sizes
                            .entry(ind_directory.to_string())
                            .or_insert(0) += d_parser_out.1.parse::<i32>().unwrap();
                        file_no_repeat.push(&d_parser_out.1[1..]);
                    }
                    // println!("{:?}", directory_sizes);
                }
            }
        }

        println!("{:?}", directory_vec);
    }

    let sums_over_1000 = directory_sizes
        .iter()
        .filter_map(|(_key, &val)| if val <= 100_000 { Some(val) } else { None })
        .collect::<Vec<i32>>();

    let final_sum: i32 = sums_over_1000.iter().sum();

    // println! {"{:?}", digit_parser("290515 cvrd.hcf")}
    // println! {"{:?}", cd_parser("290515 cvrd.hcf")}
    // println! {"{:?}", cd_parser("$ cd mlm")}
    println! {"{}", final_sum};
}
