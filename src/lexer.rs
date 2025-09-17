pub enum Token {
    CHAR(char),
    DOT,
    LBRACKET,
    RBRACKET,
    STAR,
    QUESTIONMARK,
    DIGIT,
    WORD,
    LPAREN,
    RPAREN,
    PLUS,
    OR,
    CARET,
    CASH,
}

use Token::{*};

pub fn tokenize(pattern: &str) -> Vec<Token> {
    let mut token_vec = Vec::new();
    let mut last_char_backslash = false;
    for pat in pattern.chars() {
        let token = match pat {
            '[' => LBRACKET,
            ']' => RBRACKET,
            '(' => LPAREN,
            ')' => RPAREN,
            '*' => STAR,
            '?' => QUESTIONMARK,
            '.' => DOT,
            '|' => OR,
            '^' => CARET,
            '$' => CASH,
            '\\' => {
                last_char_backslash = true;
                CHAR('\\')
            },
            'd' => {
                if last_char_backslash {
                    token_vec.pop().unwrap();
                    DIGIT
                } else {
                    CHAR('d')
                } 
            },
            'w' => {
                if last_char_backslash {
                    token_vec.pop().unwrap();
                    WORD
                } else {
                    CHAR('w')
                }
            }
            c => CHAR(c),
        };
        token_vec.push(token); 
    }
    token_vec
}