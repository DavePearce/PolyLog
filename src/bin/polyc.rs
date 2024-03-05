use std::env;
use std::fs;
use poly_log::{Parser,VecPoly};

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Reading file {}.",args[1]);
    let contents = fs::read_to_string(&args[1]).unwrap();
    //
    let parser = Parser::new(&contents);
    //
    let constraints = parser.parse_all().unwrap();
    //
    for c in constraints {
	println!("{c}");
    }
}
