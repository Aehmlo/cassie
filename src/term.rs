use variable::Variable;
use std::collections::HashMap;
use std::ops::Add;

type VariableValues = HashMap<char, f64>;

/// Terms are basic mathematical building blocks, from which are formed expressions and more complex entities.
///
/// The `Term` data type (currently) represents basic polynomial components, which can be assigned a numeric value with `Term::evaluate`/`Term::reduce`.
#[derive(Clone)]
pub enum Term {
	Variable(Variable),
	Constant(f64),
	Sum(Vec<Term>),
	Difference(Vec<Term>),
	Product(Vec<Term>),
	Quotient(Vec<Term>) // Look into limiting vector sizes to avoid confusion (due to bad input)
}

impl Term {
	/// Evaluates a term to its numerical value.
	///
	/// # Examples
	/// ```
	/// use cassie::{Term, Variable};
	/// use std::collections::HashMap;
	///
	/// let x: Variable = "x".parse().unwrap();
	/// let x = Term::Variable(x);
	/// let c = Term::Constant(100.0);
	/// let s = x + c;
	/// let mut values = HashMap::new();
	/// values.insert('x', 28.0);
	/// assert!((s.evaluate(&values).unwrap() - 128.0).abs() < 0.00001);
	/// ```
	pub fn evaluate(&self, values: &VariableValues) -> Result<f64, String> {
		self.eval(Some(values))
	}
	/// Evaluates a term to its numerical value, assuming only constants (no variables specified).
	///
	/// # Panics
	/// This method is functionally identical to using `Term::evaluate` with an empty value table, so it inherits the panic conditions from `Term::evaluate`.
	/// Most significantly, **if a variable is present in `self`, this function will panic**, since the variable value will not be resolved.
	/// 
	/// # Examples
	/// ```
	/// use cassie::Term;
	/// let c = Term::Constant(64.0);
	/// assert!((c.reduce().unwrap() - 64.0) < 0.00001);
	///
	/// let b = Term::Constant(64.0);
	/// let a = Term::Constant(36.0);
	/// let c = &a + &b;
	/// assert!(a.reduce().unwrap() - 36.0 < 0.00001);
	/// assert!(b.reduce().unwrap() - 64.0 < 0.00001);
	/// assert!(c.reduce().unwrap() - 100.0 < 0.00001);
	/// ```
	pub fn reduce(&self) -> Result<f64, String> {
		self.eval(None)
	}

	fn eval(&self, values: Option<&VariableValues>) -> Result<f64, String> {
		use Term::*;
		match *self {
			Constant(value) => Ok(value),
			Sum(ref terms) => {
				let mut sum = 0.0;
				for term in terms {
					match term.eval(values) {
						Ok(value) => {
							sum += value;
						}, Err(e) => {
							return Err(e);
						}
					};
				}
				Ok(sum) // dim sum for a twosome
			}, Difference(ref terms) => {
				let first = terms[0].eval(values);
				if first.is_err() { return first; }
				let mut difference = first.unwrap();
				for term in terms[1..].iter() {
					match term.eval(values) {
						Ok(value) => {
							difference -= value;
						}, Err(e) => {
							return Err(e);
						}
					};
				}
				Ok(difference)
			}, Product(ref terms) => {
				let mut product = 1.0;
				for term in terms {
					match term.eval(values) {
						Ok(value) => {
							product *= value;
						}, Err(e) => {
							return Err(e);
						}
					};
				}
				Ok(product)
			}, Quotient(ref terms) => {
				let first = terms[0].eval(values);
				if first.is_err() { return first; }
				let mut quotient = first.unwrap();
				for term in terms {
					match term.eval(values) {
						Ok(dividend) => {
							if dividend.abs() <  0.00001 {
								return Err("Attempted division by zero.".to_string());
							}
							quotient /= dividend;
						}, Err(e) => {
							return Err(e);
						}
					};
				}
				Ok(quotient)
			}, Variable(ref variable) => {
				if let Some(v) = values {
					if let Some(value) = v.get(&variable.symbol) {
						Ok(*value)
					} else {
						Err(format!("No value provided for variable {}", variable.symbol))
					}
				} else {
					Err(format!("No variable values provided (looking for {})", variable.symbol))
				}
			}
		}
	}
}

impl<'a, 'b> Add<&'b Term> for &'a Term { // We clone things a lot just in case a mutable operation is later defined on Term; we don't want to be chasing those bugs!

	type Output = Term;

	fn add(self, another: &'b Term) -> Term {
		match *self {
			Term::Sum(ref terms) => {
				match *another {
					Term::Sum(ref more) => {
						let mut terms = terms.clone();
						for term in more {
							terms.push(term.clone());
						}
						Term::Sum(terms)
					}, _ => {
						let mut terms = terms.clone();
						terms.push(another.clone());
						Term::Sum(terms)
					}
				}
			}, _ => {
				match *another {
					Term::Sum(ref terms) => {
						let mut terms = terms.clone();
						terms.push(self.clone());
						Term::Sum(terms)
					}, _ => {
						Term::Sum(vec!(self.clone(), another.clone()))
					}
				}
			}
		}
	}
}

impl Add for Term { // We clone things a lot just in case a mutable operation is later defined on Term; we don't want to be chasing those bugs!

	type Output = Term;

	fn add(self, another: Term) -> Term {
		&self + &another
	}
}

mod tests {
	#[allow(unused_imports)]
	use Term::Constant;
	#[test]
	fn term_sums() {
		let a = Constant(36.0);
		let b = Constant(64.0);
		let c = &a + &b;
		assert!(a.reduce().unwrap() - 36.0 < 0.00001);
		assert!(b.reduce().unwrap() - 64.0 < 0.00001);
		assert!(c.reduce().unwrap() - 100.0 < 0.00001);
	}
}
