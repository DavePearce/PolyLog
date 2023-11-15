use poly_log::{VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

#[test]
fn arithmetic_01() {
    let p1 = VecPoly::from(0);
    let p2 = VecPoly::from(0);
    assert_eq!(p1,p2);
}

#[test]
fn arithmetic_02() {
    let p1 = VecPoly::from(0);
    let p2 = VecPoly::from(1);
    assert!(p1 != p2);
}

#[test]
fn arithmetic_03() {
    let p1 = x();
    let p2 = x();
    assert_eq!(p1,p2);
}

#[test]
fn arithmetic_04() {
    let p1 = VecPoly::from(1);
    let p2 = VecPoly::from(2);
    assert_eq!(p1 + 1,p2);
}

#[test]
fn arithmetic_04b() {
    let p1 = VecPoly::from(-1);
    let p2 = VecPoly::from(0);
    assert_eq!(p1 + 1,p2);
}

#[test]
fn arithmetic_05() {
    let p1 = x() + 1;
    let p2 = x() + 1;
    assert_eq!(p1,p2);
}

#[test]
fn arithmetic_06() {
    let p1 = x() + x();
    let p2 = x() * 2;
    assert_eq!(p1, p2);
}

#[test]
fn arithmetic_07() {
    let p1 = x() - x();
    let p2 = VecPoly::from(0);
    assert_eq!(p1, p2);
}

#[test]
fn arithmetic_08() {
    let p1 = (x() * 2) - x();
    let p2 = x();
    assert_eq!(p1, p2);
}

#[test]
fn arithmetic_09() {
    let p1 = x() + 1;
    let p2 = p1.clone() * p1;
    // x^2 + 2x + 1
    let p3 = (x() * x()) + (x() * 2) + 1;
    assert_eq!(p2, p3);
}

#[test]
fn arithmetic_10() {
    let p1 = x() * y();
    let p2 = y() * x();
    assert_eq!(p1, p2);
}

#[test]
fn arithmetic_11() {
    let p1 = x() + y();
    let p2 = y() + x();
    assert_eq!(p1, p2);
}
