use poly_log::{Polynomial,VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

// Induction tests

#[test]
fn constraint_induction_01() {
    // (x == y) ==> (x+1 == y+1)
    let c1 = x().equals(&y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_02() {
    // (x != y) ==> (x+1 != y+1)
    let c1 = x().not_equals(&y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_03() {
    // (x < y) ==> (x+1 < y+1)
    let c1 = x().less_than(&y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_04() {
    // (x <= y) ==> (x+1 <= y+1)
    let c1 = x().less_than_or_equals(&y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_05() {
    // (x <= 2y) ==> (x+1 <= 2*(y+1))
    let c1 = x().less_than_or_equals(&(y() * 2));
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 2));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

