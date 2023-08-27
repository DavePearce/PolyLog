use poly_log::{Constraint,Polynomial};

pub fn x() -> Polynomial { Polynomial::var(0) }
pub fn y() -> Polynomial { Polynomial::var(1) }

// Equality constraints

#[test]
fn constraint_equals_01() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.equals(Polynomial::from(0));
    // Check holds
    assert_eq!(c1,Constraint::from(true));
}

#[test]
fn constraint_equals_02() {
    let p1 = Polynomial::from(1);
    let mut c1 = p1.equals(Polynomial::from(0));
    // Check doesn't hold
    assert_eq!(c1,Constraint::from(false));
}

#[test]
fn constraint_equals_03() {
    let p1 = x();
    let mut c1 = p1.equals(Polynomial::from(0));
    let c2 = c1.clone();
    // Check no change
    assert_eq!(c1,c2);
}

#[test]
fn constraint_equals_04() {
    let mut c1 = Constraint::eq_zero(x() + 1);
    // Check doesn't hold
    assert_eq!(c1,Constraint::from(false));
}

#[test]
fn constraint_not_equals_01() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.not_equals(0.into());
    // Check doesn't hold
    assert_eq!(c1,false.into());
}

#[test]
fn constraint_not_equals_02() {
    let p1 = Polynomial::from(1);
    let mut c1 = p1.not_equals(0.into());
    // Check does hold
    assert_eq!(c1,Constraint::from(true));
}

#[test]
fn constraint_not_equals_03() {
    let p1 = x();
    let mut c1 = p1.not_equals(0.into());
    let c2 = c1.clone();
    // Check no change
    assert_eq!(c1,c2);
}

#[test]
fn constraint_not_equals_04() {
    let p1 = x() + 1;
    let mut c1 = p1.not_equals(0.into());
    // Check does hold
    assert_eq!(c1,Constraint::from(true));
}

#[test]
fn constraint_less_than_01() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.less_than(0.into());
    // Check doesn't hold
    assert_eq!(c1,false.into());
}

#[test]
fn constraint_less_than_02() {
    let p1 = Polynomial::from(1);
    let mut c1 = p1.less_than(0.into());
    // Check doesn't hold
    assert_eq!(c1,false.into());
}

#[test]
fn constraint_less_than_03() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.less_than(1.into());
    // Check doesn't hold
    assert_eq!(c1,true.into());
}

#[test]
fn constraint_less_than_04() {
    let p1 = x();
    let mut c1 = p1.less_than(0.into());
    // Check doesn't hold
    assert_eq!(c1,false.into());
}

#[test]
fn constraint_less_than_05() {
    let p1 = x() + 1;
    let mut c1 = Polynomial::from(0).less_than(p1);
    let c2 = c1.clone();
    // Holds
    assert_eq!(c1,true.into());
}

#[test]
fn constraint_less_than_or_equals_01() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.less_than_or_equals(0.into());
    // Check doesn't hold
    assert_eq!(c1,true.into());
}

#[test]
fn constraint_less_than_or_equals_02() {
    let p1 = Polynomial::from(1);
    let mut c1 = p1.less_than_or_equals(0.into());
    // Check doesn't hold
    assert_eq!(c1,false.into());
}

#[test]
fn constraint_less_than_or_equals_03() {
    let p1 = Polynomial::from(0);
    let mut c1 = p1.less_than_or_equals(1.into());
    // Check does hold
    assert_eq!(c1,true.into());
}

#[test]
fn constraint_less_than_or_equals_04() {
    let p1 = x();
    let mut c1 = p1.less_than_or_equals(0.into());
    // Check uncertain
    assert!(c1 != false.into());
    assert!(c1 != true.into());
}

#[test]
fn constraint_less_than_or_equals_05() {
    let p1 = x();
    let mut c1 = Polynomial::from(0).less_than_or_equals(p1);
    // Holds
    assert_eq!(c1,true.into());
}

#[test]
fn constraint_less_than_or_equals_06() {
    let p1 = x() + 1;
    let mut c1 = Polynomial::from(0).less_than_or_equals(p1);
    // Holds
    assert_eq!(c1,true.into());
}

// Conjunctions

#[test]
fn constraint_conjunct_01() {
    let c1 = Constraint::eq_zero(Polynomial::from(0));
    let mut c2 = c1.and(Constraint::from(true));
    // Check holds
    assert_eq!(c2,Constraint::from(true));
}

#[test]
fn constraint_conjunct_02() {
    let c1 = Constraint::eq_zero(Polynomial::from(1));
    let mut c2 = c1.and(Constraint::from(true));
    // Check holds
    assert_eq!(c2,Constraint::from(false));
}

#[test]
fn constraint_conjunct_03() {
    let c1 = Constraint::eq_zero(Polynomial::from(0));
    let c2 = Constraint::eq_zero(Polynomial::from(1));
    let mut c3 = c1.and(c2);
    // Check holds
    assert_eq!(c3,Constraint::from(false));
}

#[test]
fn constraint_conjunct_04() {
    let c1 = Constraint::eq_zero(Polynomial::from(1));
    let c2 = Constraint::eq_zero(Polynomial::from(0));
    let mut c3 = c1.and(c2);
    // Check holds
    assert_eq!(c3,Constraint::from(false));
}

#[test]
fn constraint_conjunct_05() {
    let c1 = Constraint::eq_zero(x());
    let c2 = Constraint::eq_zero(Polynomial::from(0));
    let mut c3 = c1.clone().and(c2);
    // Check reduced
    assert_eq!(c3,c1);
}

// Disjunctions

#[test]
fn constraint_disjunct_01() {
    let c1 = Constraint::eq_zero(Polynomial::from(0));
    let mut c2 = c1.or(Constraint::from(false));
    // Check holds
    assert_eq!(c2,Constraint::from(true));
}

#[test]
fn constraint_disjunct_02() {
    let c1 = Constraint::eq_zero(Polynomial::from(1));
    let mut c2 = c1.or(Constraint::from(false));
    // Check holds
    assert_eq!(c2,Constraint::from(false));
}

#[test]
fn constraint_disjunct_03() {
    let c1 = Constraint::eq_zero(Polynomial::from(0));
    let c2 = Constraint::eq_zero(Polynomial::from(1));
    let mut c3 = c1.or(c2);
    // Check holds
    assert_eq!(c3,Constraint::from(true));
}

#[test]
fn constraint_disjunct_04() {
    let c1 = Constraint::eq_zero(Polynomial::from(1));
    let c2 = Constraint::eq_zero(Polynomial::from(0));
    let mut c3 = c1.or(c2);
    // Check holds
    assert_eq!(c3,Constraint::from(true));
}

#[test]
fn constraint_disjunct_05() {
    let c1 = Constraint::eq_zero(x());
    let c2 = Constraint::eq_zero(Polynomial::from(0));
    let mut c3 = c1.or(c2);
    // Check reduced
    assert_eq!(c3,Constraint::from(true));
}

// Negation

#[test]
fn constraint_negation_01() {
    let c1 = Constraint::eq_zero(x());
    assert_eq!(c1.not(),Constraint::neq_zero(x()));
}

#[test]
fn constraint_negation_02() {
    let c1 = Constraint::neq_zero(x());
    assert_eq!(c1.not(),Constraint::eq_zero(x()));
}

#[test]
fn constraint_negation_03() {
    let c1 = Constraint::gt_zero(x());
    assert_eq!(c1.not(),Constraint::gteq_zero(x().negate()));
}

#[test]
fn constraint_negation_04() {
    let c1 = Constraint::gteq_zero(x());
    assert_eq!(c1.not(),Constraint::gt_zero(x().negate()));
}

#[test]
fn constraint_negation_05() {
    let c1 = Constraint::gt_zero(x());
    let c2 = Constraint::eq_zero(Polynomial::var(1));
    let c3 = c1.clone().and(c2.clone()).not();
    assert_eq!(c3,c1.not().or(c2.not()));
}

// Substitution

#[test]
fn constraint_substitute_01() {
    let mut c1 = Constraint::eq_zero(x());
    c1.substitute(0, &1.into());
    assert_eq!(c1, Constraint::FALSE);
}

#[test]
fn constraint_substitute_02() {
    let mut c1 = Constraint::eq_zero(x());
    c1.substitute(0, &0.into());
    assert_eq!(c1, Constraint::TRUE);
}

#[test]
fn constraint_substitute_03() {
    let c1 = Constraint::eq_zero(x());
    let c2 = Constraint::gt_zero(x());
    let mut c3 = c1.and(c2);
    c3.substitute(0, &0.into());
    assert_eq!(c3, Constraint::FALSE);
}

#[test]
fn constraint_substitute_04() {
    let c1 = Constraint::eq_zero(x());
    let c2 = Constraint::gt_zero(x());
    let mut c3 = c1.or(c2);
    c3.substitute(0, &0.into());
    assert_eq!(c3, Constraint::TRUE);
}


// Induction tests

#[test]
fn constraint_induction_01() {
    // (x == y) ==> (x+1 == y+1)
    let c1 = x().equals(y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_02() {
    // (x != y) ==> (x+1 != y+1)
    let c1 = x().not_equals(y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_03() {
    // (x < y) ==> (x+1 < y+1)
    let c1 = x().less_than(y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_04() {
    // (x <= y) ==> (x+1 <= y+1)
    let c1 = x().less_than_or_equals(y());
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 1));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}

#[test]
fn constraint_induction_05() {
    // (x <= 2y) ==> (x+1 <= 2*(y+1))
    let c1 = x().less_than_or_equals(y() * 2);
    let mut c2 = c1.clone();
    c2.substitute(0, &(x() + 2));
    c2.substitute(1, &(y() + 1));
    assert_eq!(c1, c2);
}
