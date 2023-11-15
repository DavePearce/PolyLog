mod vec_poly;

pub use vec_poly::*;

// ===================================================================
// Polynomial
// ===================================================================

/// An _extended_ polynomial.
pub trait Polynomial : Sized+Clone {
    /// The field over which this polynomial works.
    type Field;
	
    /// Evaluate this `Polynomial` at a given point.    
    fn eval(&self, vals: &[Self::Field]) -> Self::Field;

    /// Determine whether or not this polynomial could be zero (or
    /// not).  For example, `2x+1` cannot be zero (i.e. given that `x`
    /// cannot be negative).  However, `x - y` can be zero (e.g. when
    /// `x=y`).  Note: just because this polynomial could evaluate to
    /// zero, it does not mean that it will.
    fn is_zero(&self) -> Option<bool>;
	
    /// Substitute all occurrenes of a given variable in `self` with a
    /// given `Polynomial`.  For example, substituting `x:=x+1` into
    /// `2x + xy` gives `2+2x+xy+y`.
    fn substitute(&self, var: usize, val: &Self) -> Self;
    
    /// Negate this polynomial.  This is achieved by negating each
    /// term within the polynomial.
    fn neg(self) -> Self;

    /// Add a given `Polynomial` onto this polynomial.  For example,
    /// adding `x+2` to `2x+1` gives `3x+3`.
    fn add(self, rhs: &Self) -> Self;

    /// Subtract a given `Polynomial` from this polynomial.
    fn sub(self, rhs: &Self) -> Self;

    /// Multiply a given `Polynomial` by this polynomial.  For
    /// example, multiplying `x+2` by `2x+1` gives `x+2+4x^2+4x`.
    fn mul(self, rhs: &Self) -> Self;

    /// Divide a given `Polynomial` into this polynomial.
    fn div(self, rhs: &Self) -> Self;

    /// Constrain a given `Polynomial` with this polynomial.
    /// Specifically, the resulting polynomial evaluates to `1` when
    /// they are equal and `0` otherwise.
    fn equals(self, rhs: &Self) -> Self;

    /// Constrain a given `Polynomial` to be not equal to this
    /// polynomial.  Specifically, the resulting polynomial evaluates
    /// to `1` when they are _not_ equal and `0` otherwise.
    fn not_equals(self, rhs: &Self) -> Self {
	self.equals(rhs).not()
    }
    
    /// Constrain a given `Polynomial` to be above this polynomial.
    /// Specifically, the resulting polynomial evaluates to `1` when
    /// this is true and `0` otherwise.
    fn less_than(self, rhs: &Self) -> Self;

    /// Constrain a given `Polynomial` to be above or equal to this
    /// polynomial.  Specifically, the resulting polynomial evaluates
    /// to `1` when this is true and `0` otherwise.
    fn less_than_or_equals(self, rhs: &Self) -> Self {
	// Default implementation.  This is perhaps not super
	// efficient as it may perform more memory allocations than
	// necessary.
	let tmp = self.clone().equals(rhs);
	self.less_than(rhs).or(&tmp)
    }

    /// Constrain a given `Polynomial` to be below this polynomial.
    /// Specifically, the resulting polynomial evaluates to `1` when
    /// this is true and `0` otherwise.
    fn greater_than(self, rhs: &Self) -> Self;

    /// Constrain a given `Polynomial` to be below or equal to this
    /// polynomial.  Specifically, the resulting polynomial evaluates
    /// to `1` when this is true and `0` otherwise.
    fn greater_than_or_equals(self, rhs: &Self) -> Self {
	// Default implementation.  This is perhaps not super
	// efficient as it may perform more memory allocations than
	// necessary.
	let tmp = self.clone().equals(rhs);
	self.greater_than(rhs).or(&tmp)
    }
    
    /// Construct the logical disjunction of a given `Polynomial` with
    /// this polynomial.  Specifically, the resulting polynomial: (i)
    /// evaluates to `1` when either polynomial evaluates to `1`; and
    /// (ii) `0` when both polynonmials evaluate to `0`.  Observe
    /// that, should either polynomial evaluate to something other
    /// than `1` or `0` then the result is _undefined_ (i.e. could
    /// evaluate to anything).
    fn or(self, rhs: &Self) -> Self {
	// x + y - xy
	todo!()
    }

    /// Construct the logical conjunction of a given `Polynomial` with
    /// this polynomial.  Specifically, the resulting polynomial: (i)
    /// evaluates to `1` when both polynomials evaluate to `1`; and
    /// (ii) `0` when either polynonmial evaluate to `0`.  Observe
    /// that, should either polynomial evaluate to something other
    /// than `1` or `0` then the result is _undefined_ (i.e. could
    /// evaluate to anything).
    fn and(self, rhs: &Self) -> Self {
	self.mul(rhs)
    }
    
    /// Construct the logical inversion (i.e. not) of this
    /// `Polynomial`.  Specifically, the resulting polynomial: (i)
    /// evaluates to `1` when the original evaluates to `0`; and (ii)
    /// evaluates to `0` when the original evaluates to `1`.  Observe
    /// that, should the original polynomial evaluate to something
    /// other than `1` or `0` then the result is _undefined_
    /// (i.e. could evaluate to anything).
    fn not(self) -> Self {
	// 1 - x
	todo!()
    }
}
