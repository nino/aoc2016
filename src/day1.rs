use std::fs;

enum Rotation {
    L,
    R,
}

impl Rotation {
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "L" => Ok(Rotation::L),
            "R" => Ok(Rotation::R),
            other => Err(format!("'{}' is not a valid rotation.", other)),
        }
    }
}

struct Instruction {
    rotation: Rotation,
    distance: i32,
}

impl Instruction {
    fn from_string(s: &str) -> Result<Self, &str> {
        let mut rotation = String::from(s.trim());
        let distance = rotation.split_off(1);
        match (
            Rotation::from_string(rotation.as_str()),
            distance.parse::<i32>(),
        ) {
            (Ok(rot), Ok(dist)) => Ok(Instruction {
                rotation: rot,
                distance: dist,
            }),
            _ => Err("shrug"),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, rot: &Rotation) -> Direction {
        // This is kind of dumb because I implemented all of these as enums
        match (self, rot) {
            (Direction::North, Rotation::L) => Direction::West,
            (Direction::South, Rotation::L) => Direction::East,
            (Direction::East, Rotation::L) => Direction::North,
            (Direction::West, Rotation::L) => Direction::South,
            (Direction::North, Rotation::R) => Direction::East,
            (Direction::South, Rotation::R) => Direction::West,
            (Direction::East, Rotation::R) => Direction::South,
            (Direction::West, Rotation::R) => Direction::North,
        }
    }
}

struct State {
    direction: Direction,
    x: i32,
    y: i32,
}

impl State {
    fn new() -> Self {
        State {
            direction: Direction::North,
            x: 0,
            y: 0,
        }
    }

    fn turn(&mut self, rot: &Rotation) {
        self.direction = self.direction.turn(rot);
    }

    fn walk(&mut self, distance: &i32) {
        match self.direction {
            Direction::North => self.y += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x += distance,
            Direction::East => self.x -= distance,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        self.turn(&instruction.rotation);
        self.walk(&instruction.distance);
    }

    fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}


fn part1(instructions: &Vec<Instruction>) {
    let mut state = State::new();
    for instruction in instructions {
        state.execute_instruction(instruction);
    }

    println!("We need to walk {} steps", state.distance_from_origin());
}

fn part2(instructions: &Vec<Instruction>) {
    use std::collections::HashSet;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut state = State::new();
    for instruction in instructions {
        state.turn(&instruction.rotation);
        for _ in 0..instruction.distance {
            seen.insert((state.x, state.y));
            state.walk(&1);
            if seen.contains(&(state.x, state.y)) {
                println!("We need to walk {} steps", state.distance_from_origin());
                return;
            }
        }

    }

}

fn run_with_input(content: String) {
    let instructions: Result<Vec<Instruction>, &str> = content
        .split(",")
        .map(|part| Instruction::from_string(part))
        .collect();

    match instructions {
        Err(err) => println!("Unable to parse instructions: {}", err),
        Ok(instructions) => {
            part1(&instructions);
            part2(&instructions);
        }
    }
}

pub fn run() {
    println!("--- Day 1: No Time for a Taxicab ---");

    match fs::read_to_string("assets/day1.txt") {
        Err(_) => println!("Unable to read file for day 1"),
        Ok(content) => run_with_input(content),
    }
}
