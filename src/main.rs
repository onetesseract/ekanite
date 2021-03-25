use ekparser;
mod env;
mod eval;
fn main() {
    println!("{:?}",ekparser::new_tree("file.ek"));
}