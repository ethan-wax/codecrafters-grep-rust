use std::collections::VecDeque;

pub enum Single {
    LIT(char),
    DOT,
    DIGIT,
    WORD,
}

pub enum Brackets {
    POS(Vec<Single>),
    NEG(Vec<Single>),
}

pub enum Paren {
    NIL,
    CONS(Expr, Box<Paren>),
}

pub enum Expr {
    START,
    END,
    BRACKET(Brackets),
    PLUS(Box<Expr>),
    QMARK(Box<Expr>),
    PAREN(Box<Paren>),
    SEQUENCE(Box<Expr>, Box<Expr>),
    SINGLE(Single),
    BLANK
}

pub fn parse(pattern: &String) -> Expr {
    let mut char_vec = pattern.chars().collect();
    parse_expr(&mut char_vec)
}

fn parse_expr(char_vec: &mut VecDeque<char>) -> Expr {
    if char_vec.is_empty() {
        return Expr::BLANK;
    };
    let first = char_vec.pop_front().unwrap();
    let mut left_expr = match first {
        '^' => Expr::START,
        '?' => Expr::QMARK(Box::new(Expr::BLANK)),
        '+' => Expr::PLUS(Box::new(Expr::BLANK)),
        '$' => Expr::END,
        '(' => Expr::PAREN(Box::new(parse_paren(char_vec))),
        '[' => Expr::BRACKET(parse_bracket(char_vec)),
        _ => {
            char_vec.push_front(first);
            Expr::SINGLE(parse_single(char_vec))
        }
    };
    
    let mut right_expr = parse_expr(char_vec);
    while matches!(right_expr, Expr::QMARK(ref b) if matches!(**b, Expr::BLANK)) || matches!(right_expr, Expr::PLUS(ref b) if matches!(**b, Expr::BLANK)) {
        match right_expr {
            Expr::QMARK(_) => left_expr = Expr::QMARK(Box::new(left_expr)),
            Expr::PLUS(_) => left_expr = Expr::PLUS(Box::new(left_expr)),
            _ => panic!("Impossible!")
        }
        right_expr = parse_expr(char_vec)
    }
    Expr::SEQUENCE(Box::new(left_expr), Box::new(right_expr))
}

fn parse_paren(char_vec: &mut VecDeque<char>) -> Paren {
    let mut paren = Paren::NIL;
    let mut char_buf = VecDeque::new();
    while !char_vec.is_empty() {
        let first = char_vec.pop_front().unwrap();
        if first == '|' || first == ')' {
            paren = Paren::CONS(parse_expr(&mut char_buf), Box::new(paren))
        } else {
            char_buf.push_back(first);
        }
    };
    paren
}

fn parse_bracket(char_vec: &mut VecDeque<char>) -> Brackets {
    let mut v = Vec::new();
    let first = char_vec.pop_front().unwrap();
    let pos_group = first != '^';
    if first == ']' {
        return Brackets::POS(Vec::new());
    }
    char_vec.push_front(first);
    v.push(parse_single(first));
    while !char_vec.is_empty() {
        let c = char_vec.pop_front().unwrap();
        if c == ']' {
            break;
        }
        char_vec.push_front(c);
        v.push(parse_single(char_vec));
    };
    match pos_group {
        true => Brackets::POS(v),
        false => Brackets::NEG(v)
    }
}

fn parse_single(char_vec: &mut VecDeque<char>) -> Single {
    let first = char_vec.pop_front().unwrap();
    match first {
        '.' => Single::DOT,
        c if c != '\\' => Single::LIT(c),
        _ => {
            let second = char_vec.pop_front();
            match second {
                None => Single::LIT('\\'),
                Some(c) if c != 'w' && c != 'd' => {
                    char_vec.push_front(c);
                    Single::LIT('\\')
                },
                Some(c) if c == 'w' => Single::WORD,
                _ => Single::DIGIT
            }
        }
    }
}