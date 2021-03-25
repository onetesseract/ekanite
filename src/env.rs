use std::collections::HashMap;
use ekparser::parser::Literal;
use ekparser::parser::Node;

struct EnvContent {
    l: Literal,
    t: Types,
}
pub(crate) struct Envir<'a> {
    is_root: bool,
    vars: HashMap<String, EnvContent>,
    fns: HashMap<String, FnContent>,
    parent: &'a Envir<'a>,
}

#[derive(Clone)]
pub struct FnContent {
    args: Box<Vec<Node>>,
    typ: Types,
    body: Node,
}
#[derive(Clone)]
enum Types {
    f64,
    bool,
}

// find the scope in which a variable is defined
fn env_lookup<'a>(e: &'a Envir, name: String) -> Result<&'a Envir<'a>, bool> {
    let mut current_env = e;
    while !current_env.is_root {
        if current_env.vars.contains_key(&name) {
            return Ok(current_env);
        }
        current_env = current_env.parent;
    }
    if current_env.vars.contains_key(&name) {
        return Ok(current_env);
    }
    return Err(false);
}

pub(crate) fn env_get(e: &Envir, name: String) -> Literal {
    if !e.vars.contains_key(&name) {
        println!("Undefined variable {}", name);
        panic!();
    }
    let x = e.vars.get(&name).expect("Undefined variable!");
    if x.l == Literal::Undef {
        println!("Reading an undefined variable!");
        panic!();
    }
    x.l.clone()
}

pub(crate) fn env_set(e: &mut Envir, name: String, l: Literal) {
    if !e.vars.contains_key(&name) {
        println!("Undefined variable {}", name);
        panic!();
    }
    let x = e.vars.get(&name).expect("Undefined variable!");
    e.vars.insert(name, EnvContent{t: x.t.clone(), l: l});
}

pub(crate) fn env_def(e: &mut Envir, name: String, typ: String) {
    let x = match &typ as &str {
        "f64" => Types::f64,
        "bool" => Types::bool,
        _ => panic!("Unknown type {}", typ),
    };
    e.vars.insert(name, EnvContent{l: Literal::Undef, t: x});
}

fn env_extend<'a>(e: &'a Envir) -> Envir<'a> {
    return Envir{is_root: false, parent: e, vars: HashMap::new(), fns: HashMap::new()}
}

// everything but for functions

fn fenv_lookup<'a>(e: &'a Envir, name: String) -> Result<&'a Envir<'a>, bool> {
    let mut current_env = e;
    while !current_env.is_root {
        if current_env.fns.contains_key(&name) {
            return Ok(current_env);
        }
        current_env = current_env.parent;
    }
    if current_env.fns.contains_key(&name) {
        return Ok(current_env);
    }
    return Err(false);
}

pub(crate) fn fenv_get(e: &Envir, name: String) -> FnContent {
    if !e.fns.contains_key(&name) {
        println!("Undefined variable {}", name);
        panic!();
    }
    let x = e.fns.get(&name).expect("Undefined function!");
    x.clone()
}

pub(crate) fn fenv_def(e: &mut Envir, name: String, typ: String, args: Box<Vec<Node>>, prog: Node) {
    let x = match &typ as &str {
        "f64" => Types::f64,
        "bool" => Types::bool,
        _ => panic!("Unknown type {}", typ),
    };
    e.vars.insert(name, EnvContent{l: Literal::Undef, t: x});
}