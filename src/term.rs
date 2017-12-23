use variable::Variable;
use std::collections::HashMap;
use std::ops::Add;

type VariableValues = HashMap<char, f64>;

/// Terms are basic mathematical building blocks, from which are formed expressions and more complex entities.
///
/// The `Term` data type (currently) represents basic polynomial components, which can be assigned a numeric value with `Term::evaluate`/`Term::reduce`.
#[derive(Clone)]
pub enum Term {
	/// Represents a term which simply a variable, one of the two foundational term types.
	///
	/// The value of the variable is looked up against the given variable values when `Term::evaluate` is called.
	///
	/// #Examples
	/// ```
	/// use cassie::{Term, Variable};
	/// use std::collections::HashMap;
	///
	/// let mut bindings = HashMap::new();
	/// bindings.insert('φ', 68.0);
	///
	/// let f = Variable::named('φ');
	/// let f = Term::Variable(f);
	/// assert!(f.evaluate(&bindings).unwrap() - 68.0 < 0.00001);
	/// ```
	Variable(Variable),
	/// Represents a constant term, one of the two foundational term types.
	///
	/// The value of this term is fixed and is calculated by simply unpacking the associated value.
	///
	/// #Examples
	/// ```
	/// use cassie::Term;
	///
	/// let c = Term::Constant(24.0);
	/// assert!(c.reduce().unwrap() - 24.0 < 0.00001);
	/// ```
	Constant(f64),
	/// Represents a sum of multiple terms.
	///
	/// To calculate the value of this term, the components are evaluated iteratively from the first to last index.
	///
	/// #Examples
	/// ```
	/// use cassie::Term;
	///
	/// let a = Term::Constant(24.0);
	/// let b = Term::Constant(72.0);
	/// let y = Term::Sum(vec!(a, b)); // Notice that this is very ugly; see below
	/// assert!(y.reduce().unwrap() - 108.0 < 0.00001);
	///
	/// let c = Term::Constant(12.0);
	/// let d = Term::Constant(27.0);
	/// let z = c + d; // Preferred
	/// assert!(z.reduce().unwrap() - 39.0 < 0.00001);
	/// ```
	Sum(Vec<Term>),
	/// Represents a difference of terms.
	///
	/// The first term is used as-is; all others have their signs inverted and are added to the first term in ascending order of index.
	Difference(Vec<Term>),
	/// Represents a product of terms.
	///
	/// All terms are multiplied together after evaluation, with evaluation proceeding in ascending index order.
	Product(Vec<Term>),
	/// Represents a quotient of terms.
	///
	/// The first term is evaluated, then divided by each following term in order of ascending index (each term is used immediately after evaluation). Fairly aggressive sanity checks are performed to prevent division by zero; if this continues to pester you, consider multiplying by the inverse instead.
	///
	/// This variant should be considered unstable; it is only due to typing constraints that simplification is implemented for more than two subterms. **Consider using `Term::Product` instead, if possible.**
	Quotient(Vec<Term>), // Look into limiting vector sizes to avoid confusion (due to bad input).
	/// Represents the sine function.
	///
	/// The associated term is evaluated and passed to a sine function to obtain a result.
	///
	/// Like any self-respecting sine function, this performs operations "in radians."
	Sine(Box<Term>), // TODO: Verify that this is what we want (this uses heap memory).
	/// Represents the cosine function.
	///
	/// The associated term is evaluated and passed to a cosine function to obtain a result.
	///
	/// Like any self-respecting cosine function, this performs operations "in radians."
	Cosine(Box<Term>), // TODO: Verify that this is what we want (this uses heap memory).
	/// Represents the tangent function.
	///
	/// The associated term is evaluated and passed to a tangent function to obtain a result.
	///
	/// Like any self-respecting cosine function, this performs operations "in radians."
	Tangent(Box<Term>), // TODO: Verify that this is what we want (this uses heap memory).
	/// Represents the inverse sine function.
	///
	/// The associated term is evaluated and passed to an inverse sine function to obtain a result.
	///
	/// Like any self-respecting trigonometric function, this performs operations "in radians."
	ArcSine(Box<Term>), // TODO: Verify that this is what we want (this uses heap memory).
	/// Represents the inverse cosine function.
	///
	/// The associated term is evaluated and passed to an inverse cosine function to obtain a result.
	///
	/// Like any self-respecting trigonometric function, this performs operations "in radians."
	ArcCosine(Box<Term>), // TODO: Verify that this is what we want (this uses heap memory).
	/// Represents the inverse tangent function.
	///
	/// The associated term is evaluated and passed to an inverse tangent function to obtain a result.
	///
	/// Like any self-respecting trigonometric function, this performs operations "in radians."
	ArcTangent(Box<Term>) // TODO: Verify that this is what we want (this uses heap memory).
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
							if dividend.abs() <  0.00000000000000001 {
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
			}, Sine(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.sin()),
					Err(e) => Err(e)
				}
			}, Cosine(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.cos()),
					Err(e) => Err(e)
				}
			}, ArcSine(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.asin()),
					Err(e) => Err(e)
				}
			}, ArcCosine(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.acos()),
					Err(e) => Err(e)
				}
			}, Tangent(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.tan()),
					Err(e) => Err(e)
				}
			}, ArcTangent(ref term) => {
				match term.eval(values) {
					Ok(value) => Ok(value.atan()),
					Err(e) => Err(e)
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
