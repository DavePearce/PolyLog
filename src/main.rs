mod lexer;
use std::env;
use std::fs;

mod parser;
mod poly;
mod vec_poly;

pub use lexer::*;
pub use parser::*;
pub use poly::*;
pub use vec_poly::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Reading file {}.",args[1]);
    let contents = fs::read_to_string(&args[1]).unwrap();
    //
    let parser = Parser::new(&contents);
    //
    let constraints = parser.parse().unwrap();
    //
    for c in constraints {
	println!("{c}");
    }
}
