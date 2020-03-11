use crate::{
	add_style, chalk_trait_fns, enum_default, enum_display, enum_fmt_impl,
	enum_impls, fn_alias, set_style,
	style::{ChalkStyle, Style},
	Chalk,
};

use std::ops::Add;
use std::ops::AddAssign;

/** A chalk with 256 colors */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AnsiChalk {
	pub color: u8,
	pub background: u8,
	pub styles: Vec<Style>,
}

trait ChalkAnsiColor {

	fn ansi(&mut self, color: u8) -> Self;
	fn bg_ansi(&mut self, color: u8) -> Self;
}