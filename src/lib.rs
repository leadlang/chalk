#![allow(clippy::tabs_in_doc_comments)]

pub mod ansi_chalk;
pub mod basic_chalk;
pub mod rgb_chalk;
pub mod style;

mod utils;

//#[cfg(test)]
mod tests;

use std::string::ToString;

/**
 * For all Chalks with different color types
 */
pub trait Chalk: Sized + ToString + Default {
	fn new() -> Self {
		Self::default()
	}

	/**
	 * Formats a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```ignore
	 * chalk.string(&"this is yellow");
	 * ```
	 */
	fn string(&self, string: &dyn ToString) -> String {
		format!("{}{}\x1b[m", self.to_string(), string.to_string())
	}

	/**
	 * Prints a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```ignore
	 * chalk.string(&"this is yellow");
	 * ```
	 */
	fn print(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		print!("{}", output);
		output
	}

	/**
	 * Prints a line using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```ignore
	 * chalk.string(&"this is yellow");
	 * ```
	 */
	fn println(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}
