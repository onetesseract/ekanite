use ekparser::parser;
use ekparser::lexer;
use crate::env::{Envir, env_def, env_set};
use crate::env;
fn eval(e: &mut Envir, ex: & parser::Node) -> parser::Literal {
    match ex {
        parser::Node::Literal(x) => (*x).clone(),
        parser::Node::Prog(x) => {
            for i in x.iter() {
                eval(e, i);
            }
            return parser::Literal::None;
        }
        parser::Node::ID(x) => env::env_get(e, x.to_string()),
        parser::Node::Binary(x, y, z) => {
            if *y == 0 {
                if let parser::Node::ID(x) = &**x {
                    let p = eval(e, z);
                    env::env_set(e, x.to_string(), p);
                    return parser::Literal::None;
                }
            }
            return apply_op(*y, eval(e,x), eval(e,z));
        }

        parser::Node::Dec(name, typ) => {
            if let ekparser::lexer::LexToken::ID(x) = name {
                if let ekparser::lexer::LexToken::ID(y) = typ {
                    env::env_def(e, x.to_string(), y.to_string());
                    return parser::Literal::None
                }
            }
            println!("Bad name/types for declaration!");
            panic!();
            
        }
        parser::Node::If(cond, then, els) => {
            let res = eval(e, cond);
            if let parser::Literal::Bool(y) = res {
                if y {
                    return eval(e, then);
                } else {
                    return eval(e, els);
                }
                // return parser::Literal::None;
            } else {
                println!("If condition does not return a boolean!");
                panic!();
            }
        }
        parser::Node::Call(name, params) => { parser::Literal::None }
        parser::Node::Null => parser::Literal::None,
        // _ => parser::Literal::None,
        parser::Node::FnDef(name, args, typ, body) => {
            if let parser::Node::ID(x) = &**name {
                if let lexer::LexToken::ID(y) = typ {
                    env::fenv_def(e, x.to_string(), y.clone(), args.clone(), *body.clone());
                    return parser::Literal::None;
                }
                println!("Invalid type type!");
                panic!();
            }
            println!("Invalid fn name!");
            panic!();
        }
    }
}

fn apply_op(op: usize, left: parser::Literal, right: parser::Literal) -> parser::Literal {
    if op < 9 {
        let x = match op {
            1 => chk_bool(left) || chk_bool(right),
            2 => chk_bool(left) && chk_bool(right),
            3 => chk_num(left) < chk_num(right),
            4 => chk_num(left) > chk_num(right),
            5 => chk_num(left) <= chk_num(right),
            6 => chk_num(left) >= chk_num(right),
            7 => chk_num(left) == chk_num(right),
            8 => chk_num(left) != chk_num(right),
            _ => { println!("Unknown opkey {}", op); panic!(); }
        }; 
        return parser::Literal::Bool(x)
    } else {
        let x = match op {
            9 => chk_num(left) + chk_num(right),
            10 => chk_num(left) - chk_num(right),
            11 => chk_num(left) * chk_num(right),
            12 => chk_num(left) / chk_num(right),
            13 => chk_num(left) % chk_num(right),
            _ => { println!("Unknown opkey {}", op); panic!(); }
        };
        return parser::Literal::Num(x)
    }
}

fn chk_num(val: parser::Literal) -> f64 {
    if let parser::Literal::Num(x) = val {
        return x;
    }
    println!("Cannot perform this operation on a non-number");
    panic!();
}

fn chk_bool(val: parser::Literal) -> bool {
    if let parser::Literal::Bool(x) = val {
        return x;
    }
    println!("Cannot perform this operation on a non-number");
    panic!();
}