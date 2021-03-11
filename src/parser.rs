use crate::lexer;
use crate::parser_helper;

#[derive(Debug)]
pub enum Node {
    Prog(Box<Vec<Node>>),
    // ID then type
    Dec(lexer::LexToken, lexer::LexToken),
    // Condition, then, else
    If(Box<Node>, Box<Node>, Box<Node>),
    // left, op, right
    Binary(Box<Node>, lexer::LexToken, Box<Node>),
    // for calls?
    ID(String), // todo: add resources (namespace, name)
    // Name, args
    Call(lexer::LexToken, Box<Vec<Node>>),
    Null, //TODO: remove

}

pub struct File {
    pub tokens: Vec<lexer::LexToken>,
    pub index: usize,
}

const OPS: [&str; 14] = ["=", "||", "&&", /* now all 7 */ "<", ">", "<=", ">=", "==", "!=", /* all 10 */ "+", "-", /* 20s */ "*", "/", "%"];
const OP_STRENGTH: [usize; 14] = [1, 2, 3, 7, 7, 7, 7, 7, 7, 10, 10, 20, 20, 20];

fn parse_dec(f: &mut File) -> Node {
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        println!("Unexpected {:?}, expected ID", f.tokens[f.index]);
        panic!();
    }
    if f.tokens[f.index+1] != lexer::LexToken::PUNC(String::from(":")) {
        println!("Unexpected {:?}, expected :", f.tokens[f.index]);
        panic!();
    }
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        println!("Unexpected {:?}, expected ID", f.tokens[f.index]);
        panic!();
    }
    let x = Node::Dec(f.tokens[f.index].clone(), f.tokens[f.index+2].clone());
    f.index += 3;
    x
}

fn maybe_dec(f: &mut File) -> bool {
    parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) && f.tokens[f.index+1] == lexer::LexToken::PUNC(String::from(":"))
}

fn parse_if(f: &mut File) -> Node {
    parser_helper::skip(lexer::LexToken::ID(String::from("if")), f);
    // aaaa somehow parse conditions
    let x = Node::Null;
    let y = parse_prog(f);
    let z;
    if f.tokens[f.index] == lexer::LexToken::ID(String::from("else")) {
        f.index += 1;
        z = parse_prog(f);
    } else { z = Node::Null }
    return Node::If(Box::new(x), Box::new(y), Box::new(z));

}
fn maybe_if(f: &mut File) -> bool{
    f.tokens[f.index] == lexer::LexToken::ID(String::from("if"))
}

fn maybe_call(f: &mut File) -> bool {
    parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) && f.tokens[f.index + 1] == lexer::LexToken::PUNC(String::from("("))
}

fn parse_call(f: &mut File) -> Node {
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        println!("Unexpected {:?}, expected ID", f.tokens[f.index]);
        panic!();
    }
    let x = f.tokens[f.index].clone(); // why?
    f.index += 1;
    let y = parser_helper::delimited(lexer::LexToken::PUNC(String::from("(")), lexer::LexToken::PUNC(String::from(")")), lexer::LexToken::PUNC(String::from(",")), f);
    return Node::Call(x, Box::new(y));
    
}
/*
fn parse_binary(f: &mut File) {
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) && parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::OP(String::new())) {
        println!("Unexpected {}, expected a binary expression")
    }
}
*/

pub fn delimited_parse_expression(f: &mut File) -> Node {
    if maybe_call(f) { return parse_call(f) }
    if parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        
        if let lexer::LexToken::ID(x) = f.tokens[f.index].clone() {
            f.index += 1;
            return Node::ID(x);
        }
    }
    println!("Invalid argument {:?}", f.tokens[f.index]);
    panic!();
}

fn parse_expression(f: &mut File) -> Node {
    if maybe_call(f) { return parse_call(f) }
    if maybe_dec(f) { return parse_dec(f) }
    if maybe_if(f) { return parse_if(f) }
    if f.tokens[f.index] == lexer::LexToken::PUNC(String::from("{")) { 
        return parse_prog(f);
    }

    return Node::Null;
}

fn parse_prog(f: &mut File) -> Node {
    if f.tokens[f.index] != lexer::LexToken::PUNC(String::from("{")) {
        return Node::Prog(Box::new(vec![parse_expression(f)]));
    }
    f.index += 1;
    let mut x: Vec<Node> = Vec::new();
    while f.tokens[f.index] != lexer::LexToken::PUNC(String::from("}")) && f.tokens[f.index] != lexer::LexToken::EOF {
        x.push(parse_expression(f));
        parser_helper::skip(lexer::LexToken::PUNC(String::from(";")),f);
    }
    parser_helper::skip(lexer::LexToken::PUNC(String::from("}")), f);
    Node::Prog(Box::new(x))
}

pub fn parse_toplevel(f: &mut File) -> Node {
    let mut x: Vec<Node> = Vec::new();
    while f.tokens[f.index] != lexer::LexToken::EOF {
        let z = parse_expression(f);
        println!("{:?}", z);
        x.push(z);
        parser_helper::skip(lexer::LexToken::PUNC(String::from(";")),f);
        // f.index += 1;
    }
    Node::Prog(Box::new(x))
}