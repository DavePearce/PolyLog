use std::mem;
use crate::{Polynomial};

#[derive(Clone,Debug,PartialEq)]
pub struct Constraint {
    term: Term
}

impl Constraint {
    pub const FALSE: Constraint = Constraint{term: Term::Lit(false)};
    pub const TRUE: Constraint = Constraint{term: Term::Lit(true)};

    pub fn eq_zero(poly: Polynomial) -> Self {
        Self{term: Term::Eq(poly).simplify()}
    }
    pub fn neq_zero(poly: Polynomial) -> Self {
        Self{term: Term::Neq(poly).simplify()}
    }
    pub fn gt_zero(poly: Polynomial) -> Self {
        Self{term: Term::Gt(poly).simplify()}
    }
    pub fn gteq_zero(poly: Polynomial) -> Self {
        Self{term: Term::GtEq(poly).simplify()}
    }
    pub fn and(mut self, cs: Constraint) -> Self {
	let term = Term::Conjunct(vec![self.term,cs.term]);
	Self{term: term.simplify()}
    }
    pub fn or(mut self, cs: Constraint) -> Self {
	let term = Term::Disjunct(vec![self.term,cs.term]);
	Self{term: term.simplify()}
    }
    pub fn not(mut self) -> Self {
        Self{term:self.term.not().simplify()}
    }
    pub fn implies(mut self, rhs: Constraint) -> Self {
        self.not().or(rhs)
    }
    pub fn substitute(&mut self, var: usize, val: &Polynomial) {
        let mut tmp = Term::Lit(false);
        self.term.substitute(var,val);
        mem::swap(&mut tmp,&mut self.term);
        self.term = tmp.simplify();
    }
}

impl From<bool> for Constraint {
    fn from(b: bool) -> Constraint {
	Constraint{term: Term::Lit(b)}
    }
}

// =================================

#[derive(Clone,Debug,PartialEq)]
enum Term {
    Conjunct(Vec<Term>),
    Disjunct(Vec<Term>),
    // p == 0
    Eq(Polynomial),
    Neq(Polynomial),
    Gt(Polynomial),
    GtEq(Polynomial),
    Lit(bool)
}

impl Term {
    pub fn substitute(&mut self, var: usize, val: &Polynomial) {
        match self {
            Term::Disjunct(cs)|Term::Conjunct(cs) => {
                for p in cs.iter_mut() {
                    p.substitute(var,val);
                }
            }
            Term::Eq(p)|Term::Neq(p)|Term::Gt(p)|Term::GtEq(p) => {
                *p = p.substitute(var,val);
            }
            Term::Lit(_) => { }
        }
    }

    // Attempt to simplify constraint
    pub fn simplify(mut self) -> Self {
	match self {
	    Term::Disjunct(cs) => { Term::simplify_or(cs) }
	    Term::Conjunct(cs) => { Term::simplify_and(cs) }
	    Term::Eq(p) => { Term::simplify_eq(p) }
	    Term::Neq(p) => { Term::simplify_neq(p) }
	    Term::Gt(p) => { Term::simplify_gt(p) }
	    Term::GtEq(p) => { Term::simplify_gteq(p) }
	    Term::Lit(b) => self
	}
    }

    pub fn not(self) -> Self {
        match self {
            Term::Eq(p) => Term::Neq(p),
            Term::Neq(p) => Term::Eq(p),
            Term::Gt(p) => Term::GtEq(p.negate()),
            Term::GtEq(p) => Term::Gt(p.negate()),
            Term::Disjunct(cs) => Term::Conjunct(Self::negate(cs)),
            Term::Conjunct(cs) => Term::Disjunct(Self::negate(cs)),
            Term::Lit(b) => Term::Lit(!b)
        }
    }

    fn negate(cs: Vec<Term>) -> Vec<Term> {
        cs.iter().map(|t| t.clone().not()).collect()
    }

    fn simplify_and(mut cs: Vec<Term>) -> Term {
	for i in 0..cs.len() {
            // FIXME: need to fix this :)
	    let ith = cs[i].clone().simplify();
	    //
	    if &ith == &Term::Lit(false) {
		// Evaluates to false
		return ith;
	    } else {
                cs[i] = ith;
            }
	}
	// Strip anything unnecessary
	cs.retain(|item| item != &Term::Lit(true));
        // FIXME: dedup!
	//
	if cs.is_empty() {
	    Term::Lit(true)
	} else if cs.len() == 1 {
            // FIXME: get rid of this :)
            cs[0].clone()
        } else {
	    Term::Conjunct(cs)
	}
    }

    fn simplify_or(mut cs: Vec<Term>) -> Term {
	for i in 0..cs.len() {
            // FIXME: need to fix this :)
	    let ith = cs[i].clone().simplify();
	    //
	    if &ith == &Term::Lit(true) {
		// Evaluates to false
		return ith;
	    } else {
                cs[i] = ith;
            }
	}
	// Strip anything unnecessary
	cs.retain(|item| item != &Term::Lit(false));
        // FIXME: dedup!
	//
	if cs.is_empty() {
	    Term::Lit(false)
	} else if cs.len() == 1 {
            // FIXME: get rid of this :)
            cs[0].clone()
        } else {
	    Term::Disjunct(cs)
	}
    }

    fn simplify_eq(p: Polynomial) -> Term {
	match p.is_zero() {
	    Some(b) => Term::Lit(b),
	    None => Term::Eq(p)
	}
    }

    fn simplify_neq(p: Polynomial) -> Term {
	match p.is_zero() {
	    Some(b) => Term::Lit(!b),
	    None => Term::Neq(p)
	}
    }

    fn simplify_gt(p: Polynomial) -> Term {
	match p.above_zero() {
	    Some(b) => Term::Lit(b),
	    None => Term::Gt(p)
	}
    }
    fn simplify_gteq(p: Polynomial) -> Term {
	match p.below_zero() {
            Some(b) => Term::Lit(!b),
            None => Term::GtEq(p)
	}
    }
}
