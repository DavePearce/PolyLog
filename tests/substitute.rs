use poly_log::{VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

// substitutions

#[test]
fn substitute_01() {
    let p1 = VecPoly::from(123);
    let p2 = p1.substitute(0,&VecPoly::from(0));
    assert_eq!(p1,p2);
}

#[test]
fn substitute_02() {
    let p1 = x();
    let p2 = p1.substitute(0,&VecPoly::from(1));
    assert_eq!(p2,1.into());
}

#[test]
fn substitute_03() {
    let p1 = y();
    let p2 = p1.substitute(0,&VecPoly::from(1));
    assert_eq!(p1,p2);
}

#[test]
fn substitute_04() {
    let p1 = x();
    let p2 = p1.substitute(1,&VecPoly::from(1));
    assert_eq!(p1,p2);
}

#[test]
fn substitute_05() {
    let p1 = y();
    let p2 = p1.substitute(1,&VecPoly::from(1));
    assert_eq!(p2,1.into());
}

#[test]
fn substitute_06() {
    let p1 = x();
    let p2 = x() + 1; // x => x + 1
    let p3 = p1.substitute(0,&p2);
    assert_eq!(p3,p2);
}

#[test]
fn substitute_07() {
    // x^2[x:=x+1] => x^2 + 2x + 1
    let p1 = x() * x();
    let p2 = x() + 1;
    let p3 = p1.substitute(0,&p2);
    let p4 = p1 + (x() * 2) + 1;
    assert_eq!(p3,p4);
}

#[test]
fn substitute_08() {
    // xy[x:=x+1] => xy + y
    let p1 = x() * y();
    let p2 = x() + 1;
    let p3 = p1.substitute(0,&p2);
    let p4 = (x() * y()) + y();
    assert_eq!(p3,p4);
}
