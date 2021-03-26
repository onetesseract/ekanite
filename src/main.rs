use ekparser;
use std::collections::HashMap;
mod env;
mod eval;
mod builtin;
fn main() {
    let x = ekparser::new_tree("file.ek");
    println!("{:?}", x);
    let mut en = env::Envir{is_root: true, vars: HashMap::new(), fns: HashMap::new(), parent: env::EnvParent::None};
    eval::eval(&mut en, &x);
}