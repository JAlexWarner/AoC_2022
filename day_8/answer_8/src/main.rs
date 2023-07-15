use std::fs;
use std::path::Path;
use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    let file_path = Path::new("src/input.txt");
    let contents = fs::read_to_string(file_path).expect("Could not read file");

    // Need to know row_length at compile time in order to use 2d array.
    // Also need to know col_length at compile time.
    const ROW_LENGTH: usize = 99;
    const COL_LENGTH: usize = 99;

    // If we wanted to use the a Vector of Vectors approach, we would use the lines below:
    // let row_length = contents.lines().next().unwrap().len();
    // let mut count = 0;
    // for _ in contents.lines() {
    //     count += 1;
    // }
    // let col_length: usize = count;
    // let mut visibility_grid = vec![ vec![0; row_length]; col_length];

    // let mut visibility_grid = [[0; ROW_LENGTH]; COL_LENGTH];
    let mut visibility_grid = Array::<i32, _>::zeros((ROW_LENGTH,COL_LENGTH).f());

    // let mut tree_grid = [[0; ROW_LENGTH]; COL_LENGTH];
    let mut tree_grid = Array::<i32, _>::zeros((ROW_LENGTH,COL_LENGTH).f());

    for (row_index, ind_line) in contents.lines().enumerate() {
        for (col_index, tree_height) in ind_line.chars().enumerate() {
            tree_grid[[row_index, col_index]] = tree_height as i32 - 0x30;
        }
    }
    
    // Assigning outer square a visibility of 1
    let mut row_0 = visibility_grid.row_mut(0);
    row_0.fill(1);
    drop(row_0);

    let mut col_0 = visibility_grid.column_mut(0);
    col_0.fill(1);
    drop(col_0);

    let mut row_98 = visibility_grid.row_mut(98);
    row_98.fill(1);
    drop(row_98);

    let mut col_98 = visibility_grid.column_mut(98);
    col_98.fill(1);
    drop(col_98);

    let mut elem_add_1 = |row: usize, col: usize| {
        let curr_elem = visibility_grid.get_mut([row,col]).unwrap();
        *curr_elem += 1;
    };

    // Assigning row visibility left to right
    for curr_row in 1..=97 {
        let mut highest = tree_grid[[curr_row,0]];

        for index in 1..=97 {
            let current_height = tree_grid[[curr_row,index]];
            if current_height > highest {
                highest = current_height;
                elem_add_1(curr_row, index);
            }
        }
    }

    // Assigning row visibility right to left
    for curr_row in (1..=97).rev() {
        let mut highest = tree_grid[[curr_row, 98]];

        for index in (1..=97).rev() {
            let current_height = tree_grid[[curr_row, index]];
            if current_height > highest {
                highest = current_height;
                elem_add_1(curr_row, index);
            }
        }
    }

    // Assigning column visibility top to bottom
    for curr_col in 1..=97 {
        let mut highest = tree_grid[[0,curr_col]];

        for index in 1..=97 {
            let current_height = tree_grid[[index,curr_col]];
            if current_height > highest {
                highest = current_height;
                elem_add_1(index, curr_col);
            }
        }
    }

    // Assigning column visibility bottom to top
    for curr_col in (1..=97).rev() {
        let mut highest = tree_grid[[98, curr_col]];

        for index in (1..=97).rev() {
            let current_height = tree_grid[[index, curr_col]];
            if current_height > highest {
                highest = current_height;
                elem_add_1(index, curr_col);
            }
        }
    }

    println!("{:?}", visibility_grid);
    println!("{}", visibility_grid.iter().filter(|&score| *score == 0).count());
}
