use std::fs;

fn move_hand(coords: &(i32, i32), direction: &char) -> (i32, i32) {
    match direction {
        'R' => (coords.0, (coords.1 + 1).clamp(0, 2)),
        'L' => (coords.0, (coords.1 - 1).clamp(0, 2)),
        'U' => ((coords.0 - 1).clamp(0, 2), coords.1),
        'D' => ((coords.0 + 1).clamp(0, 2), coords.1),
        _ => coords.clone(),
    }
}

fn part1(content: &String) {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut code = String::from("");
    let mut hand = (1, 1);
    for line in content.lines() {
        println!("Starting line at {}", keypad[hand.0 as usize][hand.1 as usize]);
        for c in line.chars() {
            hand = move_hand(&hand, &c);
            println!("Moved in {} direction to {}", c, keypad[hand.0 as usize][hand.1 as usize]);
        }
        code.push(keypad[hand.0 as usize][hand.1 as usize]);
    }

    println!("The code is {}", code);
}

fn run_with_input(content: String) {
    part1(&content);
}

pub fn run() {
    println!("--- Day 2: Bathroom Security ---");

    match fs::read_to_string("assets/day2.txt") {
        Err(_) => println!("Unable to read file for day 2"),
        Ok(content) => run_with_input(content),
    }
}
