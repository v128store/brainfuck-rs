use rustyline::Editor;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rl = Editor::<()>::new();

    // Brainfuck instructions
    let mut instructions: Vec<char> = Vec::new();

    // Cycle positions
    let mut opened: Vec<usize> = Vec::new();
    let mut blocks: HashMap<usize, usize> = HashMap::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        loop {
            match rl.readline("> ") {
                Ok(line) => {
                    for c in line.chars() {
                        parse(c, &mut instructions, &mut opened, &mut blocks);
                    }

                    eval(&instructions, &blocks);

                    instructions = Vec::new();
                    opened = Vec::new();
                    blocks = HashMap::new();
                }
                Err(_) => panic!("Read: keyboard read error!"),
            }
        }
    } else {
        let filename = &args[1];
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            for c in line.expect("Parsing: lines failed!").chars() {
                parse(c, &mut instructions, &mut opened, &mut blocks);
            }
        }

        eval(&instructions, &blocks);
    }
}

fn parse(
    c: char,
    instructions: &mut Vec<char>,
    opened: &mut Vec<usize>,
    blocks: &mut HashMap<usize, usize>,
) {
    let i = instructions.len();

    if c == '[' {
        opened.push(i);
    } else if c == ']' {
        let pos = match opened.pop() {
            Some(number) => number,
            None => panic!("Parsing: program execution error!"),
        };

        blocks.insert(i, pos);
        blocks.insert(pos, i);
    }

    instructions.push(c);
}

fn eval(instructions: &Vec<char>, blocks: &HashMap<usize, usize>) {
    let mut rl = Editor::<()>::new();

    let mut cells: Vec<u64> = vec![0; 30000];
    let mut pos = 0;

    let mut output: String = String::new();

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
            output.push_str(format!("{} ", cells[pos]).as_str());
        } else if instructions[i] == ',' {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    cells[pos] = line.parse::<u64>().expect("Read: value parsing error!");
                }
                Err(_) => panic!("Read: keyboard read error!"),
            }
        } else if instructions[i] == '[' {
            if cells[pos] == 0 {
                i = match blocks.get(&i) {
                    Some(number) => *number,
                    None => panic!("Execution: loop execution error!"),
                };
            }
        } else if instructions[i] == ']' {
            if cells[pos] != 0 {
                i = match blocks.get(&i) {
                    Some(number) => *number,
                    None => panic!("Execution: loop execution error!"),
                };
            }
        }
        i += 1;
    }

    println!("{}ok", output);
}
