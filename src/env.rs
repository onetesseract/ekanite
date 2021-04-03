use std::collections::HashMap;
use ekparser::parser::Literal;
use ekparser::parser::Node;

#[derive(Clone)]
pub enum EnvParent<'a> {
    Parent(&'a Envir<'a>),
    None,
}
#[derive(Clone)]
pub struct Envir<'a> {
    pub is_root: bool,
    pub vars: HashMap<String, TypeContent>,
    pub fns: HashMap<String, FnContent>,
    pub parent: EnvParent<'a>,
}

#[derive(Clone)]
pub struct FnContent {
    pub args: Box<Vec<Node>>,
    typ: Types,
    pub body: Node,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum F64Vals {
    Some(f64),
    None,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum BoolVals {
    Some(bool),
    None,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum StrVals{
    Some(String),
    None,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum TypeContent{
    f64(F64Vals),
    bool(BoolVals),
    str(StrVals),
    void,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Types {
    f64,
    bool,
    str,
    void,
}

// find the scope in which a variable is defined
fn env_lookup<'a>(e: &'a Envir, name: String) -> Result<&'a Envir<'a>, bool> {
    let mut current_env = e;
    while !current_env.is_root {
        if current_env.vars.contains_key(&name) {
            return Ok(current_env);
        }
        match current_env.parent {
            EnvParent::None => return Err(false),
            EnvParent::Parent(x) => current_env = x
        }
    }
    if current_env.vars.contains_key(&name) {
        return Ok(current_env);
    }
    return Err(false);
}

fn is_undef(x: TypeContent) -> bool {
    match x {
        TypeContent::f64(y) => {!matches!(y, F64Vals::Some(_))},
        TypeContent::bool(y) => {!matches!(y, BoolVals::Some(_))},
        TypeContent::str(y) => {!matches!(y, StrVals::Some(_))},
        TypeContent::void => true,
    }
}
pub(crate) fn env_get<'a>(e: &'a Envir, name: String) -> TypeContent {
    if !e.vars.contains_key(&name) {
        println!("Undefined variable {}", name);
        panic!();
    }
    let x = e.vars.get(&name).expect("Undefined variable!");
    if is_undef(x.clone()) {
        println!("Reading an undefined variable!");
        panic!();
    }
    x.clone()
}

pub(crate) fn env_set<'a> (e: &mut Envir<'a>, name: String, l: TypeContent) {
    if !e.vars.contains_key(&name) {
        println!("Undefined variable {}", name);
        panic!();
    }
    let _x = e.vars.get(&name).expect("Undefined variable!");
    e.vars.insert(name, l);
    
}

pub(crate) fn env_def(e: &mut Envir, name: String, typ: String) {
    let x = match &typ as &str {
        "f64" => TypeContent::f64(F64Vals::None),
        "bool" => TypeContent::bool(BoolVals::None),
        "str" => TypeContent::str(StrVals::None),
        "void" => TypeContent::void,
        _ => panic!("Unknown type {}", typ),
    };
    e.vars.insert(name, x);
}

pub(crate) fn env_extend<'a>(e: &'a Envir) -> Envir<'a> {
    return Envir{is_root: false, parent: EnvParent::Parent(e), vars: HashMap::new(), fns: HashMap::new()}
}

// everything but for functions

fn fenv_lookup<'a>(e: &'a Envir, name: String) -> Result<&'a Envir<'a>, bool> {
    let mut current_env = e;
    while !current_env.is_root {
        if current_env.fns.contains_key(&name) {
            return Ok(current_env);
        }
        match current_env.parent {
            EnvParent::None => return Err(false),
            EnvParent::Parent(x) => current_env = x
        }
    }
    if current_env.fns.contains_key(&name) {
        return Ok(current_env);
    }
    return Err(false);
}

pub(crate) fn fenv_get(e: &Envir, name: String) -> FnContent {
    if !e.fns.contains_key(&name) {
        println!("Undefined function {}", name);
        panic!();
    }
    let x = e.fns.get(&name).expect("Undefined function!");
    x.clone()
}

pub(crate) fn fenv_def(e: &mut Envir, name: String, typ: String, args: Box<Vec<Node>>, prog: Node) {
    let x = match &typ as &str {
        "f64" => Types::f64,
        "bool" => Types::bool,
        "str" => Types::str,
        "void" => Types::void,
        _ => panic!("Unknown type {}", typ),
    };
    e.fns.insert(name, FnContent{args: args, body: prog, typ: x});
}