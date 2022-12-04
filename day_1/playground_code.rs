fn main() {
    let dummy_text: String = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        .to_string();

    let split = dummy_text.split("\n");

    let str_vec: Vec<&str> = split.collect();

    println!("\nVector of Strings: {:?}", str_vec);

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

    println!("\nSplits after adding the entries: {:?}", splits);

    let int_vec: Vec<Vec<i32>> = splits
        .iter()
        .map(|u| {
            u.iter()
                .map(|&v| v.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    println!("\nint_vec: {:?}", int_vec);

    let sums_vec: Vec<i32> = int_vec.iter().map(|w| w.iter().sum()).collect();

    println!("\nsums_vec: {:?}", sums_vec);
    
    let mut max_index: i32 = 0;
    let mut max_value: i32 = 0;
    
    for (index, value) in sums_vec.iter().enumerate(){
        if value > &max_value{
            max_index = index as i32;
            max_value = *value;
        }
    }
    
    max_index += 1;
    
    println!("\n{}", max_index.to_string());
}
