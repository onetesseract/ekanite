use std::any::type_name;

use builtin::{print_num, print_str};
use ekparser::parser;
use ekparser::lexer;
use crate::env::{Envir, env_def, env_set};
use crate::env;
use crate::builtin;

const BUILTINS: [&str; 1] = ["print"];

// this is bad but it's the only way I can think of
enum ArbitraryData<'a> {
    Assign(AssignArbtraryData<'a>),
    LeftBinary(LeftBinaryArbitraryData<'a>),
    RightBinary(RightBinaryArbitraryData<'a>),
    If(IfArbitraryData<'a>),
    CallParams(CallParamsArbitraryData<'a>),
    CallArgs(CallArgsArbitraryData<'a>),
    Nothing
}
struct AssignArbtraryData<'a> {
    env: &'a mut Envir<'a>,
    name: String,
}
struct LeftBinaryArbitraryData<'a> {
    right: parser::Node,
    op: usize,
    finalCallback: &'a dyn Fn(&mut Envir, env::TypeContent, ArbitraryData),
    finalAData: Box<ArbitraryData<'a>>,
}

struct RightBinaryArbitraryData<'a> {
    left: env::TypeContent,
    lb: LeftBinaryArbitraryData<'a>,
}
struct IfArbitraryData<'a> {
    finalCallback: &'a dyn Fn(&mut Envir, env::TypeContent, ArbitraryData),
    finalAData: Box<ArbitraryData<'a>>,
    then: Box<parser::Node>,
    els: Box<parser::Node>,
}

struct CallParamsArbitraryData<'a> {
    finalCallback: &'a dyn Fn(&mut Envir, env::TypeContent, ArbitraryData),
    finalAData: Box<ArbitraryData<'a>>,
    oldEnv: &'a mut Envir<'a>,
    args: Vec<parser::Node>,
    idx: usize,
}
struct CallArgsArbitraryData<'a> {
    finalCallback: &'a dyn Fn(&mut Envir, env::TypeContent, ArbitraryData),
    finalAData: Box<ArbitraryData<'a>>,
    newEnv: &'a mut Envir<'a>,
    args: Vec<parser::Node>,
    idx: usize,
}
fn oneBigcallback(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    match a {
        ArbitraryData::Assign(x) => {
            env_set(x.env, x.name, c);
        }
        ArbitraryData::LeftBinary(x) => {
            eval(e, &x.right, a, &RightBinaryCallBack);
        }
    }
}

fn RightBinaryCallBack(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    if let ArbitraryData::RightBinary(x) = a {
        let applied = apply_op(x.lb.op, x.left, c);
        (x.lb.finalCallback)(e, applied, *x.lb.finalAData); //todo: figure out what A should be
    }
}

fn IfCallback(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    if let ArbitraryData::If(x) = a {
        if let env::TypeContent::bool(env::BoolVals::Some(y)) = c {
            if y {
                eval(e, &*x.then, *x.finalAData, x.finalCallback);
            } else {
                eval(e, &*x.els, *x.finalAData, x.finalCallback);
            }
            // return parser::Literal::None;
        } else if let env::TypeContent::bool(env::BoolVals::None) = c {
            println!("If condition is not defined!");
            panic!();
        }
        else {
            println!("If condition does not return a boolean!");
            panic!();
        }
    } else {
        println!("ArbitraryData supplied to IfCallback not of type If");
        panic!();
    }
}

fn BuiltinPrintCallback(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    match c {
        env::TypeContent::f64(nm) => { if let env::F64Vals::Some(x) = nm { builtin::print_num(x)} else { println!("Cannot use undefined value!"); panic!() } },
        env::TypeContent::str(s) => { if let env::StrVals::Some(x) = s { builtin::print_str(x.to_string())} else { println!("Cannot use undefined value!"); panic!() } },
        _ => panic!(),
    }
}

fn CallParamsCallback(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    if let ArbitraryData::CallParams(x) = a {
        x.idx += 1;
        if x.idx == x.args.len() {
            (x.finalCallback)()
        } else {
            eval(e, &x.args[x.idx], a, &CallParamsCallback);
        }
    } else {
        println!("ArbitraryData supplied to CallParamsCallback not of type CallParams");
        panic!();
    }
}

fn CallArgsCallback(e: &mut Envir, c: env::TypeContent, a: ArbitraryData) {
    if let ArbitraryData::CallParams(x) = a {
        x.idx += 1;
        if let ekparser::parser::Node::Dec(na, _t) = x.args[x.idx].clone() {
            if let lexer::LexToken::ID(nam) = na {
                eval(&mut e.clone(), &params[n]); //  todo: sort out immutable borrows
                env::env_set(&mut en, nam, tmp);
            }
        }
    }
}

pub fn eval(e: &mut Envir, ex: &parser::Node, a: ArbitraryData, callback: &dyn Fn(&mut Envir, env::TypeContent, ArbitraryData)) {
    match ex {
        parser::Node::Literal(x) => {
            match x {
                parser::Literal::Num(x) => {callback(e, env::TypeContent::f64(env::F64Vals::Some(*x)), a)}
                parser::Literal::Bool(x) => {callback(e, env::TypeContent::bool(env::BoolVals::Some(*x)), a)}
                parser::Literal::String(x) => {
                    callback(e, env::TypeContent::str(env::StrVals::Some(x.to_string())),a)
                }
                parser::Literal::Undef => callback(e, env::TypeContent::void, a),
                parser::Literal::None => callback(e, env::TypeContent::void, a),
            }

        },
        parser::Node::Prog(x) => {
            for i in x.iter() {
                eval(e, i);
            }
            return env::TypeContent::void;
        }
        parser::Node::ID(x) => {
            callback(e, env::env_get(e, x.to_string()), a);
        },
        parser::Node::Binary(x, y, z) => {
            if *y == 0 {
                if let parser::Node::ID(x) = &**x {
                    //eval(e, z);
                    let a = ArbitraryData::Assign(AssignArbtraryData{env: e, name: x.to_string()});
                    eval(e, z, a, &oneBigcallback);
                }
                println!("Cannot assign to  non-ID");
                panic!();
            }
            let p = ArbitraryData::LeftBinary(LeftBinaryArbitraryData{right: **z, op: *y, finalAData: Box::new(a), finalCallback: callback});
            eval(e, x, p, &oneBigcallback);
        }

        parser::Node::Dec(name, typ) => {
            if let ekparser::lexer::LexToken::ID(x) = name {
                if let ekparser::lexer::LexToken::ID(y) = typ {
                    env::env_def(e, x.to_string(), y.to_string());
                    callback(e, env::TypeContent::void, a);
                }
            }
            println!("Bad name/types for declaration!");
            panic!();
            
        }
        parser::Node::If(cond, then, els) => {
            let p = ArbitraryData::If(IfArbitraryData{then: *then, els: *els, finalAData: Box::new(a), finalCallback: callback});
            eval(e, cond, p, &IfCallback);
        }
        parser::Node::Call(name, params) => { 
            // ok so what do we need to do?
            // first does the fn even exist
            if let ekparser::parser::Node::ID(x) = &**name {
                let st: &str = &x;
                if BUILTINS.contains(&st) {
                    match st {
                        "print" => {
                            eval(e, &params[0], a, &BuiltinPrintCallback);
                            
                        }
                        _ => { println!("Undefined builtin {}", st); panic!();}
                    }
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