use poly_log::{VecPoly};

pub fn x() -> VecPoly { VecPoly::var(0) }
pub fn y() -> VecPoly { VecPoly::var(1) }

// evaluations

#[test]
fn poly_test_eval_01() {
    let p1 = VecPoly::from(1);
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
