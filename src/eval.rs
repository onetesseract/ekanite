use std::any::type_name;

use builtin::{print_num, print_str};
use ekparser::parser;
use ekparser::lexer;
use crate::env::{Envir, env_def, env_set};
use crate::env;
use crate::builtin;

const BUILTINS: [&str; 1] = ["print"];
pub fn eval(e: &mut Envir, ex: &parser::Node) -> env::TypeContent {
    match ex {
        parser::Node::Literal(x) => {
            match x {
                parser::Literal::Num(x) => {env::TypeContent::f64(env::F64Vals::Some(*x))}
                parser::Literal::Bool(x) => {env::TypeContent::bool(env::BoolVals::Some(*x))}
                parser::Literal::String(x) => {
                    env::TypeContent::str(env::StrVals::Some(x.to_string()))
                }
                parser::Literal::Undef => env::TypeContent::void,
                parser::Literal::None => env::TypeContent::void,
            }

        },
        parser::Node::Prog(x) => {
            for i in x.iter() {
                eval(e, i);
            }
            return env::TypeContent::void;
        }
        parser::Node::ID(x) => env::env_get(e, x.to_string()),
        parser::Node::Binary(x, y, z) => {
            if *y == 0 {
                if let parser::Node::ID(x) = &**x {
                    let p = eval(e, z);
                    env::env_set(e, x.to_string(), p);
                    return env::TypeContent::void;
                }
            }
            return apply_op(*y, eval(e,x), eval(e,z));
        }

        parser::Node::Dec(name, typ) => {
            if let ekparser::lexer::LexToken::ID(x) = name {
                if let ekparser::lexer::LexToken::ID(y) = typ {
                    env::env_def(e, x.to_string(), y.to_string());
                    return env::TypeContent::void;
                }
            }
            println!("Bad name/types for declaration!");
            panic!();
            
        }
        parser::Node::If(cond, then, els) => {
            let res = eval(e, cond);
            if let env::TypeContent::bool(env::BoolVals::Some(y)) = res {
                if y {
                    return eval(e, then);
                } else {
                    return eval(e, els);
                }
                // return parser::Literal::None;
            } else if let env::TypeContent::bool(env::BoolVals::None) = res {
                println!("If condition is not defined!");
                panic!();
            }
            else {
                println!("If condition does not return a boolean!");
                panic!();
            }
        }
        parser::Node::Call(name, params) => { 
            // ok so what do we need to do?
            // first does the fn even exist
            if let ekparser::parser::Node::ID(x) = &**name {
                let st: &str = &x;
                if BUILTINS.contains(&st) {
                    match st {
                        "print" => {
                            let eva = eval(e, &params[0]);
                            match eva {
                                env::TypeContent::f64(nm) => { if let env::F64Vals::Some(x) = nm { builtin::print_num(x)} else { println!("Cannot use undefined value!"); panic!() } },
                                env::TypeContent::str(s) => { if let env::StrVals::Some(x) = s { builtin::print_str(x.to_string())} else { println!("Cannot use undefined value!"); panic!() } },
                                _ => panic!(),
                            }
                        }
                        _ => { println!("Undefined builtin {}", st); panic!();}
                    }
                    return env::TypeContent::void;
                }
                let fnc = env::fenv_get(e, x.to_string());
                // we need to set up a NEW enviroment with the variables
                let mut en = env::env_extend(e);
                for i in fnc.args.iter() {
                    eval(&mut en, i);
                }
                if fnc.args.len() != params.len() {
                    println!("Wrong number of args for call to {} (expected {}, got {})", x, fnc.args.len(), params.len());
                    panic!();
                }
                for n in 0..fnc.args.len() {
                    if let ekparser::parser::Node::Dec(na, _t) = fnc.args[n].clone() {
                        if let lexer::LexToken::ID(nam) = na {
                            let tmp = eval(&mut e.clone(), &params[n]); //  todo: sort out immutable borrows
                            env::env_set(&mut en, nam, tmp);
                        }
                    }
                    
                }
                if let ekparser::parser::Node::Prog(vc) = fnc.body.clone() {
                    for i in vc.iter() {
                        if let ekparser::parser::Node::Return(x) = i {
                            return eval(&mut en, x);
                        }
                        eval(&mut en, i);
                    }
                }

                eval(&mut en, &fnc.body);
            }

            // TODOing: work returns

            env::TypeContent::void
            
            
         }
        parser::Node::Null => env::TypeContent::void,
        // _ => parser::Literal::None,
        parser::Node::FnDef(name, args, typ, body) => {
            for i in args.iter() {
                if !matches!(i, ekparser::parser::Node::Dec(_, _)) {
                    println!("This is not a variable def, it can't be in function defs.");
                    panic!();
                }
            }
            if let parser::Node::ID(x) = &**name {
                if let lexer::LexToken::ID(y) = typ {
                    env::fenv_def(e, x.to_string(), y.to_string(), args.clone(), *body.clone());
                    return env::TypeContent::void;
                } else if matches!(typ, lexer::LexToken::FN_NULL_TYPE) {
                    env::fenv_def(e, x.to_string(), String::from("void"), args.clone(), *body.clone());
                    return env::TypeContent::void;
                } else {
                    println!("Invalid type for function type!");
                    panic!();
                }
                
            }
            println!("Invalid fn name!");
            panic!();
        }
        parser::Node::Return(_) => { println!("Unhandled return (BUG)"); panic!(); } //todo
    }
}

fn apply_op(op: usize, left: env::TypeContent, right: env::TypeContent) -> env::TypeContent {
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
        return env::TypeContent::bool(env::BoolVals::Some(x));
    } else {
        let x = match op {
            9 => chk_num(left) + chk_num(right),
            10 => chk_num(left) - chk_num(right),
            11 => chk_num(left) * chk_num(right),
            12 => chk_num(left) / chk_num(right),
            13 => chk_num(left) % chk_num(right),
            _ => { println!("Unknown opkey {}", op); panic!(); }
        };
        return env::TypeContent::f64(env::F64Vals::Some(x));
    }
}

fn chk_num(val: env::TypeContent) -> f64 {
    if let env::TypeContent::f64(x) = val {
        if let env::F64Vals::Some(y) = x{
            return y;
        }
        println!("Cannot use undefined value!");
        panic!()
    }
    println!("Cannot perform this operation on a non-number");
    panic!();
}

fn chk_bool(val: env::TypeContent) -> bool {
    if let env::TypeContent::bool(x) = val {
        if let env::BoolVals::Some(y) = x{
            return y;
        }
        println!("Cannot use undefined value!");
        panic!()
    }
    println!("Cannot perform this operation on a non-number");
    panic!();
}