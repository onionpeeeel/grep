use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        // if (pattern.split_whitespace().map(|s| s.parse())) {
        //     return true;
        // } else {
            
        // }
        return input_line.contains(pattern);
    } else if pattern.chars().count() == 2 {
        let mut parts = pattern.split_whitespace().map(|s| s.parse::<i32>());

        match parts.next() {
            Some(i) => {
                println!("{:?}", i);
                return true
            }
            None => return false
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
