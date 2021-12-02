use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let instrs = read_input_file().unwrap();
    // println!("{:#?}", instrs);

    let mut cur_pos = Position::default(); // default value for usize & isize is 0

    for i in instrs {
        cur_pos.apply_instruction(i)
    }

    println!("After applying all instructions: {:#?}", cur_pos);
    println!(
        "Product of depth and horizontal pos is: {}",
        cur_pos.depth * cur_pos.h_pos as isize
    );
}
#[derive(Debug, Default)]
struct Position {
    depth: isize,
    h_pos: usize,
    aim: isize,
}
impl Position {
    fn apply_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Down(d) => self.aim += d as isize,
            Instruction::Up(d) => self.aim -= d as isize,
            Instruction::Forward(d) => {
                self.h_pos += d;
                self.depth += self.aim * d as isize;
            }
        }
    }
}
#[derive(Debug)]
enum Instruction {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir = split.next().unwrap();
        let dist = split.next().unwrap().parse::<usize>().unwrap();

        match dir {
            "forward" => Ok(Instruction::Forward(dist)),
            "up" => Ok(Instruction::Up(dist)),
            "down" => Ok(Instruction::Down(dist)),
            _ => Err(format!("'{}' is not a valid value for Instruction", s)),
        }
    }
}

fn read_input_file() -> Result<Vec<Instruction>, io::Error> {
    let mut path = env::current_dir().expect("failed to get pwd");
    path.push(r"src\input.txt");
    // path.push(r"src\test_input.txt");
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line.trim().parse().unwrap(); // Could do error mapping here
        v.push(n)
    }
    Ok(v)
}
