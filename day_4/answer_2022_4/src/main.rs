use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::tuple;
use nom::error::Error;
use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("src/day_4_input.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    let mut total_overlaps = 0;

    let mut parser = tuple::<&str, _, _, _>((
        digit1::<&str, Error<&str>>,
        tag("-"),
        digit1,
        tag(","),
        digit1,
        tag("-"),
        digit1,
    ));

    for assignment in contents.split("\n") {
        println!("{}", assignment);

        let parser_output = parser(assignment).unwrap().1;
        let first_lower = parser_output.0.parse::<i32>().unwrap();
        let first_higher = parser_output.2.parse::<i32>().unwrap();
        let second_lower = parser_output.4.parse::<i32>().unwrap();
        let second_higher = parser_output.6.parse::<i32>().unwrap();

        if (first_lower == second_lower) || (first_higher == second_higher) {
            total_overlaps +=1;
        } else if first_lower < second_lower {
            if first_higher > second_higher {
                total_overlaps += 1;
            }
        } else {
            if first_higher < second_higher {
                total_overlaps += 1;
            }
        }
        println!{"{}", total_overlaps};
    };

    println!("{}", total_overlaps);
}

#[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_parser() {
            let mut parser = tuple::<&str, _, _, _>((
                digit1::<&str, Error<&str>>,
                tag("-"),
                digit1,
                tag(","),
                digit1,
                tag("-"),
                digit1,
            ));

            assert_eq!(
                parser("2-4,6-8"),
                Ok(("", ("2", "-", "4", ",", "6", "-", "8")))
            );

            assert_eq!(
                parser("57-93,9-57"),
                Ok(("", ("57", "-", "93", ",", "9", "-", "57")))
            );
        }
    }