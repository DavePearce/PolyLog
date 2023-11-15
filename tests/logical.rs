use poly_log::{Polynomial,VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

// Conjunctions

#[test]
fn logical_conjunct_01() {
    let c1 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c2 = c1.and(&VecPoly::from(1));
    // Check holds
    assert_eq!(c2, VecPoly::from(1));
}

#[test]
fn logical_conjunct_02() {
    let c1 = VecPoly::from(1).equals(&VecPoly::from(0));
    let mut c2 = c1.and(&VecPoly::from(1));
    // Check holds
    assert_eq!(c2, VecPoly::from(0));
}

#[test]
fn logical_conjunct_03() {
    let c1 = VecPoly::from(0).equals(&VecPoly::from(0));
    let c2 = VecPoly::from(1).equals(&VecPoly::from(0));
    let mut c3 = c1.and(&c2);
    // Check holds
    assert_eq!(c3,VecPoly::from(0));
}

#[test]
fn logical_conjunct_04() {
    let c1 = VecPoly::from(1).equals(&VecPoly::from(0));
    let c2 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c3 = c1.and(&c2);
    // Check holds
    assert_eq!(c3,VecPoly::from(0));
}

#[test]
fn logical_conjunct_05() {
    let c1 = x().equals(&VecPoly::from(0));
    let c2 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c3 = c1.clone().and(&c2);
    // Check reduced
    assert_eq!(c3,c1);
}

// Disjunctions

#[test]
fn logical_disjunct_01() {
    let c1 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c2 = c1.or(&VecPoly::from(0));
    // Check holds
    assert_eq!(c2,VecPoly::from(1));
}

#[test]
fn logical_disjunct_02() {
    let c1 = VecPoly::from(1).equals(&VecPoly::from(0));
    let mut c2 = c1.or(&VecPoly::from(0));
    // Check holds
    assert_eq!(c2,VecPoly::from(0));
}

#[test]
fn logical_disjunct_03() {
    let c1 = VecPoly::from(0).equals(&VecPoly::from(0));
    let c2 = VecPoly::from(1).equals(&VecPoly::from(0));
    let mut c3 = c1.or(&c2);
    // Check holds
    assert_eq!(c3,VecPoly::from(1));
}

#[test]
fn logical_disjunct_04() {
    let c1 = VecPoly::from(1).equals(&VecPoly::from(0));
    let c2 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c3 = c1.or(&c2);
    // Check holds
    assert_eq!(c3,VecPoly::from(1));
}

#[test]
fn logical_disjunct_05() {
    let c1 = x().equals(&VecPoly::from(0));
    let c2 = VecPoly::from(0).equals(&VecPoly::from(0));
    let mut c3 = c1.or(&c2);
    // Check reduced
    assert_eq!(c3,VecPoly::from(1));
}

// Not

#[test]
fn logical_not_01() {
    let c1 = x().equals(&VecPoly::from(0));
    let c2 = x().not_equals(&VecPoly::from(0));    
    assert_eq!(c1.not(),c2);
}

#[test]
fn logical_not_02() {
    let c1 = x().equals(&VecPoly::from(0));
    let c2 = x().not_equals(&VecPoly::from(0));        
    assert_eq!(c2.not(),c1);
}
