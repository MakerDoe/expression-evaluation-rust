use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Exponent,
	OpenParenthesis,
	CloseParenthesis,
	None,
}

impl From<char> for Operator {
	#[inline(always)]
	fn from(value: char) -> Operator {
		match value {
			'+' => Operator::Add,
			'-' => Operator::Subtract,
			'*' => Operator::Multiply,
			'/' => Operator::Divide,
			'^' => Operator::Exponent,
			'(' => Operator::OpenParenthesis,
			')' => Operator::CloseParenthesis,
			_ => Operator::None,
		}
	}
}

impl fmt::Display for Operator {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Operator::Add => "+",
				Operator::Subtract => "-",
				Operator::Multiply => "*",
				Operator::Divide => "/",
				Operator::Exponent => "^",
				Operator::OpenParenthesis => "(",
				Operator::CloseParenthesis => ")",
				Operator::None => "",
			}
		)
	}
}

impl Operator {
	#[inline(always)]
	pub fn precedence(&self) -> u8 {
		match self {
			Operator::Add | Operator::Subtract => 1,
			Operator::Multiply | Operator::Divide => 2,
			Operator::Exponent => 3,
			_ => 0,
		}
	}

	#[inline(always)]
	pub fn associativity(&self) -> bool {
		match self {
			Operator::Exponent => false,
			_ => true,
		}
	}

	#[inline(always)]
	pub fn is_operator(&self) -> bool {
		!matches!(self, Operator::None)
	}

	#[inline(always)]
	pub fn is_operator_not_parenthesis(&self) -> bool {
		self.is_operator() && !matches!(self, Operator::OpenParenthesis | Operator::CloseParenthesis)
	}

	#[inline(always)]
	pub fn evaluate(&self, left: f64, right: f64) -> f64 {
		match self {
			Operator::Add => left + right,
			Operator::Subtract => left - right,
			Operator::Multiply => left * right,
			Operator::Divide => left / right,
			Operator::Exponent => left.powf(right),
			_ => 0.0,
		}
	}
}
