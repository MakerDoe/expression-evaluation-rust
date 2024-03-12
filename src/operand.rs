use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Operand {
	pub is_decimal: bool,
	pub is_negative: bool,
	pub initialized: bool,

	pub hole_number: i32,
	pub fraction_part: u32,
	pub decimal_placement: u32,
}

impl Default for Operand {
	#[inline(always)]
	fn default() -> Self {
		Self {
			is_decimal: false,
			is_negative: false,
			initialized: false,

			hole_number: 0,
			fraction_part: 0,
			decimal_placement: 1,
		}
	}
}

impl fmt::Display for Operand {
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.parse())
	}
}

impl Operand {
	#[inline(always)]
	pub fn parse(&self) -> f64 {
		let result = self.hole_number as f64 + ((self.fraction_part as f64) / (self.decimal_placement as f64));

		match self.is_negative {
			true => result * -1.0,
			false => result,
		}
	}

	#[inline(always)]
	pub fn add_digit(&mut self, digit: u32) {
		if self.is_decimal {
			self.fraction_part = self.fraction_part * 10 + digit;
			self.decimal_placement *= 10;
		} else {
			self.hole_number = self.hole_number * 10 + digit as i32;
		}

		self.initialized = true;
	}
}
