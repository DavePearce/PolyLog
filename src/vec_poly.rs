use std::fmt;
use crate::Polynomial;

// =============================================================================
// Polynomial
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct VecPoly {
    terms: Vec<Term>
}

impl VecPoly {
    pub fn var(vindex: usize) -> VecPoly {
	let term = Term::new(1,&[vindex]);
	Self{terms: vec![term]}
    }

    /// Determine the _sign_ of this polynomial (if known).  Here,
    /// _positive_ sign indicates the polynomial can never evaluate to
    /// a negative number.  In constrast, a _negative_ sign indicates
    /// it cannot evalute to a (strictly) positive number.  For
    /// example, `1+2x` has positive sign, whilst `-2x` has negative
    /// sign.
    pub fn sign(&self) -> Option<bool> {
        let mut sign = false;
        for (i,t) in self.terms.iter().enumerate() {
            let ith = t.coefficient >= 0;
            if i == 0 { sign = ith; }
            else if sign != ith {
                return None;
            }
        }
	Some(sign)
    }

    /// Determine the _constant_ coefficient of this polynomial.  For
    /// example, `1` is the constant component of `2x+1`.
    pub fn constant(&self) -> isize {
	for t in &self.terms {
	    if t.vars.len() == 0 {
		return t.coefficient;
	    }
	}
	0
    }

    /// Determine whether or not this polynomial could be zero (or
    /// not).  For example, `2x+1` cannot be zero (i.e. given that `x`
    /// cannot be negative).  However, `x - y` can be zero (e.g. when
    /// `x=y`).  Note: just because this polynomial could evaluate to
    /// zero, it does not mean that it will.
    pub fn is_zero(&self) -> Option<bool> {
	// Check whether this is zero (or not)
	if self.terms.len() == 0 { Some(true) }
	// Check whether cannot be zero
	else if self.sign() != None && self.constant() != 0 {
	    // Cannot be zero.
	    Some(false)
	} else {
	    // Still could be zero
	    None
	}
    }

    /// Negate this polynomial.  This is achieved by negating each
    /// term within the polynomial.
    pub fn neg(mut self) -> Self {
        for t in &mut self.terms {
            t.negate();
        }
        self
    }

    /// Add a given `VecPoly` onto this polynomial.  For example,
    /// adding `x+2` to `2x+1` gives `3x+3`.
    pub fn add(mut self, rhs: &VecPoly) -> Self {
        for t in &rhs.terms {
            self.internal_add(t);
        }
        self
    }

    /// Subtract a given `VecPoly` from this polynomial.
    pub fn sub(mut self, rhs: &VecPoly) -> Self {
        for t in &rhs.terms {
            self.internal_sub(t);
        }
        self
    }

    pub fn mul(mut self, rhs: &VecPoly) -> Self {
	let mut ts = Vec::new();
	// Swap them
	std::mem::swap(&mut ts, &mut self.terms);
	//
	for t1 in ts.into_iter() {
	    for t2 in &rhs.terms {
		// FIXME: ugly clone :)
		let t3 = t1.clone().mul(&t2);
		self.internal_add(&t3);
	    }
	}
	self
    }

    /// Evaluate this `VecPoly` at a given point.
    pub fn eval(&self, vals: &[isize]) -> isize {
	let mut acc = 0;
	for t in &self.terms {
	    acc += t.eval(vals);
	}
	acc
    }

    /// Substitute all occurrenes of a given variable in `self` with a
    /// given `VecPoly`.  For example, substituting `x:=x+1` into
    /// `2x + xy` gives `2+2x+xy+y`.
    pub fn substitute(&self, var: usize, val: &VecPoly) -> VecPoly {
        let mut r = Self{terms: Vec::new()};
        //
        for t in &self.terms {
            let p = t.substitute(var,val);
            r = r + p;
        }
        //
        r
    }
    
    // Add a single term into this polynomial.
    fn internal_add(&mut self, term: &Term) {
        for (i,t) in &mut self.terms.iter_mut().enumerate() {
            if t.vars == term.vars {
                t.coefficient += term.coefficient;
                if t.coefficient == 0 {
                    // Remove if empty coefficient.
                    self.terms.remove(i);
                }
                return;
            }
        }
        // Otherwise push back
        self.terms.push(term.clone());
	// Resort
	self.terms.sort();
    }

    // Add a single term into this polynomial.
    fn internal_sub(&mut self, term: &Term) {
        for (i,t) in &mut self.terms.iter_mut().enumerate() {
            if t.vars == term.vars {
                t.coefficient -= term.coefficient;
                if t.coefficient == 0 {
                    // Remove if empty coefficient.
                    self.terms.remove(i);
                }
                return;
            }
        }
        // Otherwise push back
        let mut t = term.clone();
        t.negate();
        self.terms.push(t);
	// Resort
	self.terms.sort();
    }
}

// Polynomial
// -----------------------------------------------------------------------------

impl Polynomial for VecPoly {
    type Field = isize;

    fn eval(&self, vals: &[Self::Field]) -> Self::Field {
	Self::eval(self,vals)
    }

    fn is_zero(&self) -> Option<bool> {
	todo!()
    }
	
    fn substitute(&self, var: usize, val: &Self) -> Self  {
	todo!()
    }
    
    fn neg(self) -> Self  {
	todo!()
    }

    fn add(self, rhs: &Self) -> Self  {
	todo!()
    }

    fn sub(self, rhs: &Self) -> Self  {
	todo!()
    }

    fn mul(self, rhs: &Self) -> Self  {
	todo!()
    }

    fn div(self, rhs: &Self) -> Self  {
	todo!()
    }

    fn equate(self, rhs: &Self) -> Self  {
	todo!()
    }

    fn less_than(self, rhs: &Self) -> Self  {
	todo!()
    }
}

// Formatting
// -----------------------------------------------------------------------------

impl fmt::Display for VecPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	for (i,t) in self.terms.iter().enumerate() {
	    if i != 0 { write!(f,"+")?; }
	    write!(f,"{t}")?;
	}
	Ok(())
    }
}
// Coercions
// -----------------------------------------------------------------------------
impl From<usize> for VecPoly {
    fn from(val: usize) -> Self {
	if val == 0 {
	    Self{terms: Vec::new()}
	} else {
            let term = Term{coefficient: val as isize, vars: Vec::new()};
            Self{terms: vec![term]}
	}
    }
}

impl From<i32> for VecPoly {
    fn from(val: i32) -> Self {
	if val == 0 {
	    Self{terms: Vec::new()}
	} else {
            let term = Term{coefficient: val as isize, vars: Vec::new()};
            Self{terms: vec![term]}
	}
    }
}

// Operator overloading
// -----------------------------------------------------------------------------
impl std::ops::Add<usize> for VecPoly {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
	let r = VecPoly::from(rhs);
	self.add(&r)
    }
}

impl std::ops::Add for VecPoly {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
	self.add(&rhs)
    }
}

impl std::ops::Sub<usize> for VecPoly {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self {
	let r = VecPoly::from(rhs);
	self.sub(&r)
    }
}

impl std::ops::Sub for VecPoly {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
	self.sub(&rhs)
    }
}

impl std::ops::Mul<usize> for VecPoly {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
	let r = VecPoly::from(rhs);
	self.mul(&r)
    }
}

impl std::ops::Mul for VecPoly {
    type Output = Self;

    fn mul(self, rhs: VecPoly) -> Self {
	self.mul(&rhs)
    }
}

// =============================================================================
// VecPoly Term
// =============================================================================
#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Term {
    coefficient: isize,
    vars: Vec<usize>
}

impl Term {
    pub fn new(coefficient: isize, vars: &[usize]) -> Self {
	Self{coefficient, vars: vars.to_vec()}
    }

    pub fn negate(&mut self) {
        self.coefficient = -self.coefficient;
    }

    pub fn mul(mut self, rhs: &Term) -> Self {
	self.coefficient *= rhs.coefficient;
	self.vars.extend_from_slice(&rhs.vars);
	self.vars.sort();
	self
    }

    pub fn eval(&self, vals: &[isize]) -> isize {
	let mut r = self.coefficient;
	for v in &self.vars {
	    r *= vals[*v];
	}
	r
    }

    pub fn substitute(&self, var: usize, val: &VecPoly) -> VecPoly {
        let mut nvars = Vec::new();
        let mut count = 0;
        // Construct inner term.
        for v in &self.vars {
            if *v == var {
                count = count + 1;
            } else {
                nvars.push(*v);
            }
        }
        //
        let nterm = Self{coefficient: self.coefficient, vars: nvars};
        let mut r = VecPoly{terms: vec![nterm]};
        // Multiply it all out
        for i in 0..count {
            r = r * val.clone();
        }
        r
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.coefficient)?;
	for idx in &self.vars {
	    // FIXME: this could fail
	    let v = (*idx as u32) % 3;
	    let w = (*idx as u32) / 3;
	    let a = ('x' as u32) + v;
	    let c = char::from_u32(a).unwrap();
	    write!(f,"*{c}{w}")?;
	}
	Ok(())
    }
}
