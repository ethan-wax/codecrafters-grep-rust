use std::env;
use std::io;
use std::process;

pub mod parser;
use parser::{Expr, Single, Brackets, Paren, parse};

fn match_qmark(input_line: &str, expr: &Expr, pos: usize) -> (bool, usize) {
    let (m, p) = match_expr(input_line, expr, pos);
    if m {
        return (true, p);
    }
    (true, pos)
}

fn match_single(input_line: &str, s: &Single, pos: usize) -> (bool, usize) {
    let c = match input_line.chars().nth(pos) {
        Some(n) => n,
        None => {
            return (false, pos);
        }
    };
    match s {
        Single::DOT => (true, pos + 1),
        Single::DIGIT => (c.is_numeric(), pos + 1),
        Single::WORD => (c.is_alphanumeric() || c == '_', pos + 1),
        Single::LIT(n) => (c == *n, pos + 1),
    }
}

fn match_paren(input_line: &str, p: &Paren, pos: usize) -> (bool, usize) {
    match p {
        Paren::NIL => (false, pos),
        Paren::CONS(hd, tl) => {
            let (m, p) = match_expr(input_line, hd, pos);
            if m {
                return (true, p);
            };
            match_paren(input_line, &tl, pos)
        }
    }
}

fn match_brackets(input_line: &str, b: &Brackets, pos: usize) -> (bool, usize) {
    match b {
        Brackets::POS(v) => {
            for s in v {
                let (m, p) = match_single(input_line, s, pos);
                if m {
                    return (true, p);
                };
            };
            (false, pos)
        }
        Brackets::NEG(v) => {
            for s in v {
                let (m, _) = match_single(input_line, s, pos);
                if m {
                    return (false, pos);
                };
            };
            (true, pos + 1)
        }
    }
}

fn match_end(input_line: &str, pos: usize) -> (bool, usize) {
    (pos == input_line.len(), pos)
}

fn match_start(pos: usize) -> (bool, usize) {
    (pos == 0, pos)
}

fn match_sequence(input_line: &str, left: &Expr, right: &Expr, pos: usize) -> (bool, usize) {
    if pos > input_line.len() {
        return (false, pos);
    }
    let (m, p) = match_expr(input_line, left, pos);
    if !m {
        return (false, pos);
    };
    if matches!(left, Expr::PLUS(_)) {
        let (m_plus, p_plus) = match_sequence(input_line, left, right, pos + 1);
        if m_plus {
            return (m_plus, p_plus);
        }
    }
    return match_expr(input_line, right, p)
}

fn match_expr(input_line: &str, expr: &Expr, pos: usize) -> (bool, usize){
    match expr {
        Expr::BLANK => (true, pos),
        Expr::START => match_start(pos),
        Expr::END => match_end(input_line, pos),
        Expr::BRACKET(b) => match_brackets(input_line, b, pos),
        Expr::PAREN(p) => match_paren(input_line, &**p, pos),
        Expr::SEQUENCE(left, right) => match_sequence(input_line, left, right, pos),
        Expr::PLUS(e) => match_expr(input_line, e, pos),
        Expr::QMARK(e) => match_qmark(input_line, e, pos),
        Expr::SINGLE(s) => match_single(input_line, s, pos),
    }
}


fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let expr = parse(&pattern.to_string());
    
    if let Expr::BLANK = expr {
        return true;
    }

    for i in 0..input_line.len() {
        if match_expr(input_line, &expr, i).0 {
            return true;
        }
    };
    false
    
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
