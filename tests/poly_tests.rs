use poly_log::{Polynomial};

pub fn x() -> Polynomial { Polynomial::var(0) }
pub fn y() -> Polynomial { Polynomial::var(1) }

#[test]
fn poly_test_01() {
    let p1 = Polynomial::from(0);
    let p2 = Polynomial::from(0);
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_02() {
    let p1 = Polynomial::from(0);
    let p2 = Polynomial::from(1);
    assert!(p1 != p2);
}

#[test]
fn poly_test_03() {
    let p1 = x();
    let p2 = x();
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_04() {
    let p1 = Polynomial::from(1);
    let p2 = Polynomial::from(2);
    assert_eq!(p1 + 1,p2);
}

#[test]
fn poly_test_04b() {
    let p1 = Polynomial::from(-1);
    let p2 = Polynomial::from(0);
    assert_eq!(p1 + 1,p2);
}

#[test]
fn poly_test_05() {
    let p1 = x() + 1;
    let p2 = x() + 1;
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_06() {
    let p1 = x() + x();
    let p2 = x() * 2;
    assert_eq!(p1, p2);
}

#[test]
fn poly_test_06b() {
    let p1 = x() - x();
    let p2 = Polynomial::from(0);
    assert_eq!(p1, p2);
}

#[test]
fn poly_test_06c() {
    let p1 = (x() * 2) - x();
    let p2 = x();
    assert_eq!(p1, p2);
}

#[test]
fn poly_test_07() {
    let p1 = x() + 1;
    let p2 = p1.clone() * p1;
    // x^2 + 2x + 1
    let p3 = (x() * x()) + (x() * 2) + 1;
    assert_eq!(p2, p3);
}

#[test]
fn poly_test_08() {
    let p1 = x() * y();
    let p2 = y() * x();
    assert_eq!(p1, p2);
}

#[test]
fn poly_test_09() {
    let p1 = x() + y();
    let p2 = y() + x();
    assert_eq!(p1, p2);
}

// is zero

#[test]
fn poly_test_is_zero_01() {
    let p = Polynomial::from(0);
    assert_eq!(p.is_zero(), Some(true));
}

#[test]
fn poly_test_is_zero_02() {
    let p = Polynomial::from(1);
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn poly_test_is_zero_03() {
    let p = Polynomial::from(-1);
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn poly_test_is_zero_04() {
    let p = x();
    assert_eq!(p.is_zero(), None);
}

#[test]
fn poly_test_is_zero_05() {
    let p = x() + 1;
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn poly_test_is_zero_06() {
    let p = x() - 1;
    assert_eq!(p.is_zero(), None);
}

// above zero

#[test]
fn poly_test_above_zero_01() {
    let p = Polynomial::from(0);
    assert_eq!(p.above_zero(), Some(false));
}

#[test]
fn poly_test_above_zero_02() {
    let p = Polynomial::from(1);
    assert_eq!(p.above_zero(), Some(true));
}

#[test]
fn poly_test_above_zero_03() {
    let p = Polynomial::from(-1);
    assert_eq!(p.above_zero(), Some(false));
}

#[test]
fn poly_test_above_zero_04() {
    let p = x();
    assert_eq!(p.above_zero(), None);
}

#[test]
fn poly_test_above_zero_05() {
    let p = x() + 1;
    assert_eq!(p.above_zero(), Some(true));
}

// below zero

#[test]
fn poly_test_below_zero_01() {
    let p = Polynomial::from(0);
    assert_eq!(p.below_zero(), Some(false));
}

#[test]
fn poly_test_below_zero_02() {
    let p = Polynomial::from(1);
    assert_eq!(p.below_zero(), Some(false));
}

#[test]
fn poly_test_below_zero_03() {
    let p = Polynomial::from(-1);
    assert_eq!(p.below_zero(), Some(true));
}

#[test]
fn poly_test_below_zero_04() {
    let p = x();
    assert_eq!(p.below_zero(), Some(false));
}

#[test]
fn poly_test_below_zero_05() {
    let p = x() + 1;
    assert_eq!(p.below_zero(), Some(false));
}

#[test]
fn poly_test_below_zero_06() {
    let p = x() -1 ;
    assert_eq!(p.below_zero(), None);
}

// evaluations

#[test]
fn poly_test_eval_01() {
    let p1 = Polynomial::from(1);
    assert_eq!(p1.eval(&[123]), 1);
}

#[test]
fn poly_test_eval_02() {
    let p1 = x() + 1;
    assert_eq!(p1.eval(&[123]), 124);
}

#[test]
fn poly_test_eval_03() {
    let p1 = x() + 1;
    let p2 = p1.clone() * p1;
    assert_eq!(p2.eval(&[3]), 16);
}

// substitutions

#[test]
fn poly_test_substitute_01() {
    let p1 = Polynomial::from(123);
    let p2 = p1.substitute(0,&Polynomial::from(0));
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_substitute_02() {
    let p1 = x();
    let p2 = p1.substitute(0,&Polynomial::from(1));
    assert_eq!(p2,1.into());
}

#[test]
fn poly_test_substitute_03() {
    let p1 = y();
    let p2 = p1.substitute(0,&Polynomial::from(1));
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_substitute_04() {
    let p1 = x();
    let p2 = p1.substitute(1,&Polynomial::from(1));
    assert_eq!(p1,p2);
}

#[test]
fn poly_test_substitute_05() {
    let p1 = y();
    let p2 = p1.substitute(1,&Polynomial::from(1));
    assert_eq!(p2,1.into());
}

#[test]
fn poly_test_substitute_06() {
    let p1 = x();
    let p2 = x() + 1; // x => x + 1
    let p3 = p1.substitute(0,&p2);
    assert_eq!(p3,p2);
}

#[test]
fn poly_test_substitute_07() {
    // x^2[x:=x+1] => x^2 + 2x + 1
    let p1 = x() * x();
    let p2 = x() + 1;
    let p3 = p1.substitute(0,&p2);
    let p4 = p1 + (x() * 2) + 1;
    assert_eq!(p3,p4);
}

#[test]
fn poly_test_substitute_08() {
    // xy[x:=x+1] => xy + y
    let p1 = x() * y();
    let p2 = x() + 1;
    let p3 = p1.substitute(0,&p2);
    let p4 = (x() * y()) + y();
    assert_eq!(p3,p4);
}
