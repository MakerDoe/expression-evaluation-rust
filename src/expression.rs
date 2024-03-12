use std::fmt::Display;

use crate::{operand::Operand, operators::Operator};

#[derive(Debug)]
pub enum Postfix {
	Operand(Operand),
	Operator(Operator),
}

impl Display for Postfix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Postfix::Operand(operand) => write!(f, "{}", operand),
			Postfix::Operator(operator) => write!(f, "{}", operator),
		}
	}
}

#[derive(Debug)]
pub struct Expression {
	pub stack: Vec<Operator>,
	pub postfix: Vec<Postfix>,
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut output = String::default();

		for postfix in &self.postfix {
			output.push_str(&format!("{} ", postfix));
		}

		write!(f, "{}", output)
	}
}

impl Default for Expression {
	#[inline(always)]
	fn default() -> Self {
		Self {
			stack: Vec::new(),
			postfix: Vec::new(),
		}
	}
}

impl Expression {
	pub fn evaluate(&self) -> Result<f64, &str> {
		let mut stack = Vec::new();

		for postfix in &self.postfix {
			match postfix {
				Postfix::Operand(operand) => stack.push(operand.parse()),
				Postfix::Operator(operator) => {
					let right = match stack.pop() {
						Some(value) => value,
						None => return Err("No right operand"),
					};

					let left = match stack.pop() {
						Some(value) => value,
						None => return Err("No left operand"),
					};

					stack.push(operator.evaluate(left, right));
				}
			}
		}

		match stack.pop() {
			Some(result) => Ok(result),
			None => Err("No result"),
		}
	}
}

impl Expression {
	pub fn parse(input: &str) -> Self {
		let mut expression = Self::default();

		let mut operand = Operand::default();

		for ch in input.chars() {
			match ch {
				'0'..='9' => operand.add_digit(ch.to_digit(10).unwrap()),
				'.' => operand.is_decimal = true,
				_ => expression.parse_other_chars(ch, &mut operand),
			}
		}

		if operand.initialized {
			expression.postfix.push(Postfix::Operand(operand));
		}

		while let Some(op) = expression.stack.pop() {
			if op.is_operator_not_parenthesis() {
				expression.postfix.push(Postfix::Operator(op));
			}
		}

		expression
	}

	fn parse_close_parenthesis(&mut self) {
		while let Some(op) = self.stack.pop() {
			if op == Operator::OpenParenthesis {
				break;
			}

			self.postfix.push(Postfix::Operator(op));
		}
	}

	fn parse_operators(&mut self, operator: Operator) {
		let precedence = operator.precedence();

		while let Some(op) = self.stack.last() {
			let other_precedence = op.precedence();

			if precedence < other_precedence || (precedence == other_precedence && operator.associativity()) {
				self.postfix.push(Postfix::Operator(self.stack.pop().unwrap()));
			} else {
				break;
			}
		}

		self.stack.push(operator);
	}

	fn parse_other_chars(&mut self, ch: char, operand: &mut Operand) {
		let operator = Operator::from(ch);

		if operator.is_operator() && operand.initialized {
			self.postfix.push(Postfix::Operand(operand.clone()));
			*operand = Operand::default();
		}

		match operator {
			Operator::None => return,
			Operator::OpenParenthesis => self.stack.push(operator),
			Operator::CloseParenthesis => self.parse_close_parenthesis(),
			_ => self.parse_operators(operator),
		}
	}
}
