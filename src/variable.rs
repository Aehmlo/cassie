use std::str::FromStr;
use std::fmt;

/// A variable represents a value which is arbitrary or unknown.
///
/// The assumption that this object will eventually be assigned a meaningful value is the basis of algebraic manipulation.
#[derive(PartialEq)]
pub struct Variable {
	pub symbol: char
}

impl fmt::Debug for Variable {
	/// Variables may be printed using the fmt::Debug trait.
	/// # Examples
	/// ```
	/// use cassie::Variable;
	/// let var = Variable { symbol: 'x' };
	/// assert_eq!(&format!("{:?}", var), "x");
	/// let var = Variable { symbol: 'α' };
	/// assert_eq!(&format!("{:?}", var), "α");
	/// ```
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.symbol)
	}
}

impl FromStr for Variable {
	type Err = String;
	/// Variables may be constructed from string literals, provided that they are well-formed.
	/// # Examples
	/// ```
	/// use cassie::Variable;
	/// assert_eq!(Variable { symbol: 'x' }, "x".parse::<Variable>().unwrap());
	/// assert_eq!(Variable { symbol: 'Γ' }, "Γ".parse::<Variable>().unwrap());
	/// // Note that variable names must comprise exactly one character.
	///	assert!("".parse::<Variable>().is_err());
	///	assert!("xy".parse::<Variable>().is_err());
	/// ```
	fn from_str(s: &str) -> Result<Variable, Self::Err> {
		let chars = s.chars().collect::<Vec<_>>();
		match chars.len() {
			0 => Err(format!("Variables must be one character long (none given).")),
			1 => Ok(Variable { symbol: chars[0] }),
			_ => Err(format!("Variables cannot be longer than one character ({} found).", chars.len()))
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
	}
}