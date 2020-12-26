use std::collections::HashMap;
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

    // Brainfuck instructions
    let mut instructions: Vec<char> = Vec::new();

    // Cycle positions
    let mut opened: Vec<usize> = Vec::new();
    let mut blocks: HashMap<usize, usize> = HashMap::new();

    for line in reader.lines() {
        for c in line.expect("Lines failed!").chars() {
            let i = instructions.len();
            if c == '[' {
                opened.push(i);
            } else if c == ']' {
                let pos = match opened.pop() {
                    Some(number) => number,
                    None => panic!("Program execution error!"),
                };
                blocks.insert(i, pos);
                blocks.insert(pos, i);
            }
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
        } else if instructions[i] == '[' {
            if cells[pos] == 0 {
                i = match blocks.get(&i) {
                    Some(number) => *number,
                    None => panic!("Program execution error"),
                };
            }
        } else if instructions[i] == ']' {
            if cells[pos] != 0 {
                i = match blocks.get(&i) {
                    Some(number) => *number,
                    None => panic!("Program execution error"),
                };
            }
        }
        i += 1;
    }
}
