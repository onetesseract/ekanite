mod lexer;
mod parser;
mod parser_helper;

fn main() {
    let mut s: Vec<char> = "hi: bye;\nif {\nhi: bye;\n} else {pog:eth;}; some(ting);".chars().collect();
    let mut tokens: Vec<lexer::LexToken> = Vec::new();
    let mut x = lexer::LexToken::ML_COMMENT(String::from("hi"));
    while x != lexer::LexToken::EOF {
        x = lexer::read_next(&mut s);
        tokens.push(x.clone());
    }
    println!("{:?}", tokens);
    let mut file = parser::File{tokens: tokens, index: 0};
    println!("{:?}", parser::parse_toplevel(&mut file));
}