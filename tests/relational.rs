use poly_log::{Polynomial,VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

#[test]
fn relational_is_zero_01() {
    let p = VecPoly::from(0);
    assert_eq!(p.is_zero(), Some(true));
}

#[test]
fn relational_is_zero_02() {
    let p = VecPoly::from(1);
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn relational_is_zero_03() {
    let p = VecPoly::from(-1);
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn relational_is_zero_04() {
    let p = x();
    assert_eq!(p.is_zero(), None);
}

#[test]
fn relational_is_zero_05() {
    let p = x() + 1;
    assert_eq!(p.is_zero(), Some(false));
}

#[test]
fn relational_is_zero_06() {
    let p = x() - 1;
    assert_eq!(p.is_zero(), None);
}

// Equality constraints

#[test]
fn relational_equals_01() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.equals(&VecPoly::from(0));
    // Check holds
    assert_eq!(c1, 1.into());
}

#[test]
fn relational_equals_02() {
    let p1 = VecPoly::from(1);
    let mut c1 = p1.equals(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1, 0.into());
}

#[test]
fn relational_equals_03() {
    let p1 = x();
    let mut c1 = p1.equals(&VecPoly::from(0));
    let c2 = c1.clone();
    // Check no change
    assert_eq!(c1,c2);
}

#[test]
fn relational_equals_04() {
    let mut c1 = (x()+1).equals(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_not_equals_01() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.not_equals(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_not_equals_02() {
    let p1 = VecPoly::from(1);
    let mut c1 = p1.not_equals(&VecPoly::from(0));
    // Check does hold
    assert_eq!(c1, 1.into());
}

#[test]
fn relational_not_equals_03() {
    let p1 = x();
    let mut c1 = p1.not_equals(&VecPoly::from(0));
    let c2 = c1.clone();
    // Check no change
    assert_eq!(c1,c2);
}

#[test]
fn relational_not_equals_04() {
    let p1 = x() + 1;
    let mut c1 = p1.not_equals(&VecPoly::from(0));
    // Check does hold
    assert_eq!(c1,1.into());
}

#[test]
fn relational_less_than_01() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.less_than(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_02() {
    let p1 = VecPoly::from(1);
    let mut c1 = p1.less_than(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_03() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.less_than(&VecPoly::from(1));
    // Check doesn't hold
    assert_eq!(c1,1.into());
}

#[test]
fn relational_less_than_04() {
    let p1 = x();
    let mut c1 = p1.less_than(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_05() {
    let p1 = x() + 1;
    let mut c1 = VecPoly::from(0).less_than(&p1);
    let c2 = c1.clone();
    // Holds
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_or_equals_01() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.less_than_or_equals(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_or_equals_02() {
    let p1 = VecPoly::from(1);
    let mut c1 = p1.less_than_or_equals(&VecPoly::from(0));
    // Check doesn't hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_or_equals_03() {
    let p1 = VecPoly::from(0);
    let mut c1 = p1.less_than_or_equals(&VecPoly::from(1));
    // Check does hold
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_or_equals_04() {
    let p1 = x();
    let mut c1 = p1.less_than_or_equals(&VecPoly::from(1));
    // Check uncertain
    assert!(c1 != 0.into());
    assert!(c1 != 0.into());
}

#[test]
fn relational_less_than_or_equals_05() {
    let p1 = x();
    let mut c1 = VecPoly::from(0).less_than_or_equals(&p1);
    // Holds
    assert_eq!(c1,0.into());
}

#[test]
fn relational_less_than_or_equals_06() {
    let p1 = x() + 1;
    let mut c1 = VecPoly::from(0).less_than_or_equals(&p1);
    // Holds
    assert_eq!(c1,0.into());
}
