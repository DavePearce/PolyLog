use poly_log::{Parser,VecPoly,Term};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn m() -> VecPoly { VecPoly::var(1) }
pub fn n() -> VecPoly { VecPoly::var(2) }

// evaluations
fn parse(contents: &str) -> VecPoly {
    let mut parser = Parser::new(&contents);
    parser.parse_poly().unwrap()
}

// Construct `x+1`
fn x_p1() -> VecPoly {
    x().add(&VecPoly::from(1))
}

#[test]
fn eg_01() {
    let p = parse("forall(x) x < 10");
    assert_eq!(p.eval(&[0,9]), 0);    
    assert_eq!(p.eval(&[1,8]), 0);
    assert_eq!(p.eval(&[2,7]), 0);
    assert_eq!(p.eval(&[3,6]), 0);
    assert_eq!(p.eval(&[4,5]), 0);
    assert_eq!(p.eval(&[5,4]), 0);
    assert_eq!(p.eval(&[10,0]), 1);
    assert_eq!(p.eval(&[11,0]), 2);
    assert_eq!(p.eval(&[11,1]), 3);
    
}

#[test]
fn eg_02() {
    let p = parse("forall(x) x >= 10");    
    assert_eq!(p.eval(&[10,0]), 0);
    assert_eq!(p.eval(&[11,1]), 0);    
}

#[test]
fn eg_induct() {
    let p1 = parse("forall(x) x < 10 || x > 10");
    let p2 = p1.substitute(0,&x_p1());
    let mp1 = m().sub(&VecPoly::from(1));
    let np1 = n().add(&VecPoly::from(1));
    //
    print!("Target: ");
    printer(&p1);
    print!("Inductive: ");    
    printer(&p2);
    print!("Substitute [m=>{mp1}]: ");
    let p3 = p2.substitute(1,&mp1);    
    printer(&p3);
    print!("Substitute [n=>{np1}]: ");
    let p4 = p3.substitute(2,&np1);        
    printer(&p4);
    let p5 = p4.sub(&p1);    
    print!("Remainder: ");
    printer(&p5);        
}

fn printer(poly: &VecPoly) {
    if poly.terms().len() == 0 {
	println!("0");
    } else {
	let mut first = true;
	for (i,t) in poly.terms().iter().enumerate() {
	    if t.coefficient() >= 0 {
		if !first { print!("+"); }
		first = false;
		print_term(&t);
	    }
	}
	print!(" = ");
	first=true;
	for (i,t) in poly.terms().iter().enumerate() {
	    if t.coefficient() < 0 {
		if !first { print!("+"); }
		first = false;
		print_term(&t);	    	    
	    }
	}    
	println!();
    }
}

fn print_term(term: &Term) {
    match term.coefficient() {
	-1|1 => {}
	c => {
	    print!("{}",c.abs());
	}
    }
    for idx in term.vars() {
	let v = match idx {
	    0 => "x",
	    1 => "m",
	    2 => "n",
	    _ => { panic!(); }
	};
	print!("{v}");
    }    
}
