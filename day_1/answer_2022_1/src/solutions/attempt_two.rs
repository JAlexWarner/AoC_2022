pub fn second_solution(aoc_input: String) -> String {
    let split = aoc_input.split("\r\n");

    let str_vec: Vec<&str> = split.collect();

    // println!("\nVector of Strings: {:?}", str_vec);

    //let mut curr_elf: i32 = 1;
    let mut curr_elf_total: i32 = 0;
    let mut max_value: i32 = 0;
    //let mut max_index: i32 = 1;

    for (_, value) in str_vec.iter().enumerate() {
        match value.is_empty() {
            // The value is a delimiter
            true => {
                //curr_elf += 1;
                curr_elf_total = 0;
            }
            // The value is a calorie count
            false => {
                curr_elf_total += value.parse::<i32>().unwrap();

                if curr_elf_total > max_value {
                    // max_index = curr_elf;
                    max_value = curr_elf_total;
                }
                // println!("index: {}, current elf: {}, value: {}, max so far: {}, max elf so far: {}",
                // index.to_string(),
                // curr_elf.to_string(),
                // value.to_string(),
                // max_value.to_string(),
                // max_index.to_string()
                //);
            }
        }
    }

    max_value.to_string()
}
