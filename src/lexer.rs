const PUNCS: [&str; 11] = ["{", "}", "::", ":", ";", ".", ",", "(", "[", "]", ")"];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CharClass {
    ID_START,
    OP,
    WHITESPACE,
    LETTER,
    DIGIT,
    PUNC,
    UNKNOWN,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum LexToken {
    ID(String),
    NUMBER(f64),
    PUNC(String),
    OP(String),
    SL_COMMENT(String),
    ML_COMMENT(String),
    EOF,
    ERROR,
}

fn classify(c: char) -> CharClass {
    match c {
        'A'..='Z' | 'a'..='z' => CharClass::LETTER,
        'A'..='Z' | 'a'..='z' | '_' => CharClass::ID_START,
        '0'..='9' => CharClass::DIGIT,
        '\n' | '\t' | '\r' | ' ' => CharClass::WHITESPACE,
        _ => {
            if "+-=/<>*%^&|".contains(c) {
                CharClass::OP
            } else if "(){}.,;:\"'".contains(c) {
                CharClass::PUNC
            } else {
                println!("Unknown {}", c);
                CharClass::UNKNOWN
            }
        }
    }
}

pub fn read_next(s: &mut Vec<char>) -> LexToken {
    if s.len() == 0 {
        return LexToken::EOF;
    }
    let mut ret = String::new();
    let c = classify(s[0]);
    if c == CharClass::PUNC {
        let mut pnc = String::new();
        while s.len() != 0 && classify(s[0]) == CharClass::PUNC && !PUNCS.contains(&(&pnc as &str)) {
            pnc.push(s[0]);
            s.remove(0);
        }
        return LexToken::PUNC(pnc);
    }
    if s[0] == '/' {
        if s.len() == 1 {}
        else if s[1] == '/' { // hit a sl comment
            s.remove(0);
            s.remove(0);
            let mut comment = String::new();
            while s.len() != 0 && s[0] != '\n' {
                comment.push(s[0]);
                s.remove(0);
            }
            return LexToken::SL_COMMENT(comment);
        } else if s[1] == '*' { // hit a multiline comment
            s.remove(0);
            s.remove(0);
            let mut comment = String::new();
            while s.len() > 1 {
                if s[0] == '*' && s[1] == '/' {
                    s.remove(0);
                    s.remove(0);
                    break;
                } else {
                    comment.push(s[0]);
                    s.remove(0);
                }
            }
            return LexToken::ML_COMMENT(comment);
        }
    }
    if classify(s[0]) == CharClass::ID_START {
        while s.len() != 0 && classify(s[0]) == CharClass::ID_START || classify(s[0]) == CharClass::LETTER || classify(s[0]) == CharClass::DIGIT   {
            ret.push(s[0]);
            s.remove(0);
        }
    }
    while s.len() != 0 && classify(s[0]) == c  {
        ret.push(s[0]);
        s.remove(0);
    }
    if c == CharClass::DIGIT {
        if s[0] == '.' {
            ret.push('.');
            s.remove(0);
            while s.len() != 0 && classify(s[0]) == c  {
                ret.push(s[0]);
                s.remove(0);
            }
        }
    }
    match c {
        CharClass::OP => LexToken::OP(ret),
        CharClass::DIGIT => LexToken::NUMBER(ret.parse().unwrap()),
        CharClass::LETTER => LexToken::ID(ret),
        CharClass::WHITESPACE => read_next(s),
        CharClass::PUNC => LexToken::ERROR, //never called
        _ => { println!("Error at {}", ret); LexToken::ERROR },
    }
}