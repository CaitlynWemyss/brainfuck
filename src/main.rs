use std::{env, fs, io::{self, Write}, time::Instant};
use console::Term;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    
    if args_len < 2 { print_help(); return; }

    match args[1].as_str() {
        "r" | "run" => {
            if args_len < 3 { print_help(); return; }
            match fs::read_to_string(args[2].as_str()) {
                Err(_) => { println!("Invalid path."); },
                Ok(bf) => { dumb_brainfuck(bf.as_str()); },
            }
        }
        "b" | "bench" => {
            if args_len < 3 { print_help(); return; }
            match fs::read_to_string(args[2].as_str()) {
                Err(_) => { println!("Invalid path."); },
                Ok(bf) => {
                    let instant = Instant::now();
                    dumb_brainfuck(bf.as_str());
                    println!("Program executed in {}s", instant.elapsed().as_secs_f64())
                },
            }
        }
        _ => print_help(),
    }

    fn print_help() {
        println!("Usage: <command> run <file>");
    }
}

fn dumb_brainfuck(code: &str) { // The naive approach to Brainfuck interpretation
    let mut code_ptr = 0usize;
    let chars: Vec<char> = code.chars().collect();
    let code_len = chars.len();
    let mut mem_ptr = 0usize;
    let mut mem = [0u8; 2usize.pow(16)]; // The original specification is 30,000 bytes of memory but I don't care    
    let term = Term::stdout();

    while code_ptr < code_len {
        match chars[code_ptr] {
            '+' => mem[mem_ptr] = mem[mem_ptr].wrapping_add(1),
            '-' => mem[mem_ptr] = mem[mem_ptr].wrapping_sub(1),
            '>' => mem_ptr += 1,
            '<' => mem_ptr -= 1,
            '.' => { print!("{}", mem[mem_ptr] as char); io::stdout().flush().unwrap(); },
            ',' => mem[mem_ptr] = term.read_char().unwrap() as u8,
            '[' => {
                if mem[mem_ptr] != 0 { code_ptr += 1; continue; }

                let mut new_ptr = code_ptr;
                let mut depth = 1u16;

                while depth != 0 {
                    new_ptr += 1;
                    if chars[new_ptr] == '[' { depth += 1; }
                    else if chars[new_ptr] == ']' { depth -= 1; }
                }

                code_ptr = new_ptr + 1;
                continue;
            },
            ']' => {
                if mem[mem_ptr] == 0 { code_ptr += 1; continue; }

                let mut new_ptr = code_ptr;
                let mut depth = 1u16;

                while depth != 0 {
                    new_ptr -= 1;
                    if chars[new_ptr] == ']' { depth += 1; }
                    else if chars[new_ptr] == '[' { depth -= 1; }
                }

                code_ptr = new_ptr + 1;
                continue;
            },
            _ => (),
        }
        code_ptr += 1;
    }
    println!("");
}
