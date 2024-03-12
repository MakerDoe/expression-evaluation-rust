use std::io::{self, Write};

mod expression;
mod operand;
mod operators;

fn main() {
	loop {
		let mut input = String::default();

		print!("Expression: ");
		io::stdout().flush().expect("Failure to flush stdout");

		match io::stdin().read_line(&mut input) {
			Ok(_) => {
				let expression = expression::Expression::parse(&input);

				println!("\nPostfix: {expression}");

				match expression.evaluate() {
					Ok(result) => println!("Result: {result}\n"),
					Err(e) => println!("Oops! Something went wrong: {e}\n"),
				}
			}
			Err(e) => println!("Oops! Something went wrong: {}", e),
		}
	}
}
