fn main() {
    let aoc_input: String = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
    let easy_string: String = "aabbcc".to_string();
    let first_h: &str = &easy_string[0..easy_string.len()/2];
    println!("{}", aoc_input);
    println!("{}", first_h);
    
    for item in first_h.chars() {
        println!("{}", item);
        if easy_string[easy_string.len()/2..].contains(item) {
            println!("JESUS CHRIST, WE FOUND IT");
        }
    }
    
    let duplicate: char = 'B';
    
    let found_a = ('a'..='z').into_iter().position(|x| x == duplicate);
    //let found_A = ('A'..='A').iter()
    
    let priority = match found_a {
        Some(index) => index + 1,
        None => ('A'..='Z').into_iter().position(|y| y == duplicate).unwrap() + 27,
    };
    
    println!("{}", priority.to_string());
}