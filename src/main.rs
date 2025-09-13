use std::env;
use std::io;
use std::process;

fn match_char(c: &char, pattern: &str) -> bool {
    if pattern == "." {
        true
    } else if pattern.chars().count() == 1 {
        c.to_string() == pattern
    } else if pattern == r"\d" {
        c.is_digit(10)
    } else if pattern == r"\w" {
        c.is_alphanumeric() || c.to_string() == "_"
    } else if pattern.starts_with("[^") && pattern.ends_with(']') {
        let start = 1 as usize;
        let end = pattern.len() - 1;
        !pattern[start..end].contains(*c)
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let start = 1 as usize;
        let end = pattern.len() - 1;
        pattern[start..end].contains(*c)
    } else if pattern.ends_with('+') || pattern.ends_with('?') {
        if pattern.starts_with(".") {
            return true;
        }
        let start = pattern.chars().nth(0).unwrap();
        *c == start
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

fn match_here(input_line: &str, patterns: &Vec<String>) -> bool {
    let mut i = 0;
    for pat in patterns {
        let c = match input_line.chars().nth(i) {
            Some(c) => c,
            None => return pat == "$",
        };

        let m = match_char(&c, pat.as_str());
        let q = pat.ends_with('?');

        if !m && ! q {
            return false;
        }

        if pat.ends_with('+') {
            let new_input: String = input_line.chars().skip(i + 1).collect();
            let pos = patterns.iter().position(|s| s == pat).unwrap();
            let new_patterns = patterns.iter().skip(pos).map(|s| s.clone()).collect();
            if match_here(&new_input.as_str(), &new_patterns) {
                return true;
            }
        }

        if m {
            i = i + 1;
        }
    }
    return true;
}

fn check_clear(buf: &String, c: &char) -> bool {
    if buf.starts_with('[') && buf.ends_with(']') {
        return true;
    } else if *buf == "\\d" || buf == "\\w" {
        return true;
    } else if buf.starts_with('\\') && *c != 'd' && *c != 'w' {
        return true;
    } else if buf.chars().count() == 1 {
        let first = buf.chars().nth(0).unwrap();
        if first == '[' || first == '\\' || *c == '+' || *c == '?' {
            return false;
        }
        return true;
    } else if buf.contains('+') || buf.contains('?') {
        return true;
    } else {
        return false;
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "" {
        return true;
    } else if input_line == "" {
        return false;
    }

    let mut patterns: Vec<String> = Vec::new();
    let mut buf = String::new();
    for c in pattern.chars() {
        if check_clear(&buf, &c) {
            patterns.push(buf.clone());
            buf.clear();
        }
        buf.push(c);
    }
    patterns.push(buf.clone());

    if patterns.iter().nth(0).unwrap() == "^" {
        return match_here(input_line, &patterns[1..].to_vec());
    }

    for i in 0..input_line.chars().count() {
        let new_input: String = input_line.chars().skip(i).collect();
        if match_here(&new_input.as_str(), &patterns) {
            return true;
        }
    }
    return false;
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
