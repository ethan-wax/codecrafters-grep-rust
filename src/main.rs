use std::env;
use std::io;
use std::process;

fn match_helper(input_line: &str, pattern: &str) -> bool {
    let c = input_line.chars().nth(0).unwrap();
    if pattern.chars().count() == 1 {
        c.to_string() == pattern
    } else if pattern == r"\d" {
        c.is_digit(10)
    } else if pattern == r"\w" {
        c.is_alphanumeric() || c.to_string() == "_"
    } else if pattern.starts_with("[^") && pattern.ends_with(']') {
        let start = 1 as usize;
        let end = pattern.len() - 1;
        !pattern[start..end].contains(c)
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let start = 1 as usize;
        let end = pattern.len() - 1;
        pattern[start..end].contains(c)
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "" {
        return true;
    } else if input_line == "" {
        return false;
    }

    let first_pattern = if pattern.starts_with('[') {
        let start = 0 as usize;
        let end = pattern.find(']').expect("Missing ]");
        &pattern[start..end + 1]
    } else if pattern.starts_with(r"\") {
        match pattern.chars().nth(1) {
            Some(c) if c == 'd' || c == 'w' => &pattern[0..2],
            Some(_) | None => {
                return false;
            }
        }
    } else {
        &pattern[0..1]
    };

    if !match_helper(&input_line, first_pattern) {
        let new_input : String= input_line.chars().skip(1).collect();
        return match_pattern(new_input.as_str(), pattern);
    }

    let pattern_len = first_pattern.chars().count();
    return match_pattern(&input_line[1..], &pattern[pattern_len..]);
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
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
