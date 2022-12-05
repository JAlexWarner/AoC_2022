fn main() {
    let aoc_input: String = "A X
B X
C X
C Y
B Y
C Z
C X
C Y
C Y
B Y
B Y
C Z
C Y"
    .to_string();

    let rounds = aoc_input.split("\n");
    let str_vec: Vec<&str> = rounds.collect();

    for r in str_vec {
        
        println!(
            "\n{}, {}",
            r.chars().nth(0).unwrap(),
            r.chars().nth(2).unwrap()
        );
        
        let pov_choice = r.chars().nth(2).unwrap();
        let elf_choice = r.chars().nth(0).unwrap();
        
        let choice_points = match pov_choice {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Dear god!"),
        };
        
        println!("Choice Points: {}", choice_points);
        
        let win_points = match elf_choice {
            'A' => match pov_choice {
                'X' => 3,
                'Y' => 6,
                'Z' => 0,
                _ => panic!("Dear god!"),
            },
            'B' => match pov_choice {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => panic!("Dear god!"),
            },
            'C' => match pov_choice {
                'X' => 6,
                'Y' => 0,
                'Z' => 3,
                _ => panic!("Dear god!"),
            },
            _ => panic!("Dear god!")
        };
        
        let final_points = match pov_choice {
            'X' => 1 + match elf_choice {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => panic!("Dear god!"),
            },
            'Y' => 2 + match elf_choice {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => panic!("Dear god!"),
            },
            'Z' => 3 + match elf_choice {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => panic!("Dear god!"),
            },
            _ => panic!("Dear god!")
            
        };
        
        println!("Win Points: {}", win_points);
        println!("Final Points: {}", final_points);
    }
    // 6 win, 3 draw, 0 loss
}
