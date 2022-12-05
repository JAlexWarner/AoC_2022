pub fn first_solution(aoc_input: String) -> String {
    let split = aoc_input.split("\r\n");

    let str_vec: Vec<&str> = split.collect();

    // println!("\nVector of Strings: {:?}", str_vec);

    let mut splits: Vec<Vec<&str>> = vec![vec![]];

    for entry in str_vec.iter() {
        // Check if entry is a value or an empty string slice
        match entry.is_empty() {
            // This entry is not a value
            true => splits.push(vec![]),
            // This entry is a value
            false => {
                splits.last_mut().unwrap().push(entry);
            }
        }
    }

    let int_vec: Vec<Vec<i32>> = splits
        .iter()
        .map(|u| {
            u.iter()
                .map(|&v| v.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let sums_vec: Vec<i32> = int_vec.iter().map(|w| w.iter().sum()).collect();

    // let mut max_index: i32 = 0;
    let mut max_value: i32 = 0;

    for (_, value) in sums_vec.iter().enumerate() {
        if value > &max_value {
            // max_index = index as i32;
            max_value = *value;
        }
    }

    //max_index += 1;

    max_value.to_string()
}
