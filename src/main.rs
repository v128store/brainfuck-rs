use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Unknown file name!");
    }

    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut instructions: Vec<char> = Vec::new();
    for line in reader.lines() {
        for c in line.expect("Lines failed!").chars() {
            instructions.push(c);
        }
    }

    // character
    let mut cells: Vec<u64> = vec![0; 30000];
    let mut pos = 0;

    let mut i = 0;
    while i < instructions.len() {
        if instructions[i] == '>' {
            pos += 1;
        } else if instructions[i] == '<' {
            pos -= 1;
        } else if instructions[i] == '+' {
            cells[pos] += 1;
        } else if instructions[i] == '-' {
            cells[pos] -= 1;
        } else if instructions[i] == '.' {
            print!("{} ", cells[pos]);
        } else if instructions[i] == ',' {
            // read
        }
        i += 1;
    }
}
