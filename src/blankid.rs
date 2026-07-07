use std::borrow::{Borrow, ToOwned};
use std::fmt;

use str_newtype::StrNewType;

use crate::RdfDisplay;

#[macro_export]
macro_rules! blankid {
	($input:literal) => {
		const {
			match $crate::BlankId::from_str($input) {
				Ok(b) => b,
				Err(_) => panic!("invalid blank identifier"),
			}
		}
	};
}

/// Blank node identifier.
///
/// A blank node identifier is a string matching
/// the `BLANK_NODE_LABEL` production in the following [EBNF](http://www.w3.org/TR/REC-xml/#sec-notation) grammar:
///
/// ```ebnf
/// [141s] BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
/// [157s] PN_CHARS_BASE    ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
/// [158s] PN_CHARS_U       ::= PN_CHARS_BASE | '_' | ':'
/// [160s] PN_CHARS         ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, StrNewType)]
#[newtype(owned(BlankIdBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct BlankId(str);

impl BlankId {
	/// Checks that `input` is a valid blank node identifier, in its UTF-8
	/// byte representation.
	pub const fn validate_bytes(input: &[u8]) -> bool {
		let mut i = 0;

		if !matches!(
			utf8_decode::try_decode_char(input, &mut i),
			Ok(Some(('_', _)))
		) {
			return false;
		}

		if !matches!(
			utf8_decode::try_decode_char(input, &mut i),
			Ok(Some((':', _)))
		) {
			return false;
		}

		if !matches!(
			utf8_decode::try_decode_char(input, &mut i),
			Ok(Some((c, _))) if c.is_ascii_digit() || is_pn_char_u(c)
		) {
			return false;
		}

		loop {
			match utf8_decode::try_decode_char(input, &mut i) {
				Ok(Some((c, _))) if is_pn_char(c) => (),
				Ok(None) => break true,
				_ => break false,
			}
		}
	}

	/// Checks that `input` is a valid blank node identifier.
	pub const fn validate_str(input: &str) -> bool {
		Self::validate_bytes(input.as_bytes())
	}

	/// Returns the suffix part (after `_:`) of the blank node identifier.
	#[inline(always)]
	pub fn suffix(&self) -> &str {
		&self.0[2..]
	}
}

impl RdfDisplay for BlankId {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl PartialEq<str> for BlankId {
	#[inline(always)]
	fn eq(&self, other: &str) -> bool {
		self.0 == *other
	}
}

impl BlankIdBuf {
	/// Creates a blank node identifier using the given `u8` as suffix.
	#[inline(always)]
	pub fn from_u8(i: u8) -> Self {
		unsafe { Self::new_unchecked(format!("_:{i}")) }
	}

	/// Creates a blank node identifier using the given `u16` as suffix.
	#[inline(always)]
	pub fn from_u16(i: u16) -> Self {
		unsafe { Self::new_unchecked(format!("_:{i}")) }
	}

	/// Creates a blank node identifier using the given `u32` as suffix.
	#[inline(always)]
	pub fn from_u32(i: u32) -> Self {
		unsafe { Self::new_unchecked(format!("_:{i}")) }
	}

	/// Creates a blank node identifier using the given `u64` as suffix.
	#[inline(always)]
	pub fn from_u64(i: u64) -> Self {
		unsafe { Self::new_unchecked(format!("_:{i}")) }
	}

	/// Creates a blank node identifier using the given suffix.
	#[inline(always)]
	pub fn from_suffix(suffix: &str) -> Result<Self, InvalidBlankId<String>> {
		Self::new(format!("_:{suffix}"))
	}
}

impl Borrow<str> for BlankIdBuf {
	#[inline(always)]
	fn borrow(&self) -> &str {
		self.0.as_str()
	}
}

impl Borrow<BlankId> for &BlankIdBuf {
	#[inline(always)]
	fn borrow(&self) -> &BlankId {
		self.as_blank_id()
	}
}

impl Borrow<str> for &BlankIdBuf {
	#[inline(always)]
	fn borrow(&self) -> &str {
		self.0.as_str()
	}
}

impl<'a> From<&'a BlankIdBuf> for &'a BlankId {
	#[inline(always)]
	fn from(b: &'a BlankIdBuf) -> Self {
		b.as_ref()
	}
}

impl RdfDisplay for BlankIdBuf {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.as_blank_id().rdf_fmt(f)
	}
}

const fn is_pn_char_base(c: char) -> bool {
	matches!(c, 'A'..='Z' | 'a'..='z' | '\u{00c0}'..='\u{00d6}' | '\u{00d8}'..='\u{00f6}' | '\u{00f8}'..='\u{02ff}' | '\u{0370}'..='\u{037d}' | '\u{037f}'..='\u{1fff}' | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}' | '\u{3001}'..='\u{d7ff}' | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}' | '\u{10000}'..='\u{effff}')
}

const fn is_pn_char_u(c: char) -> bool {
	is_pn_char_base(c) || matches!(c, '_' | ':')
}

const fn is_pn_char(c: char) -> bool {
	is_pn_char_u(c)
		|| matches!(c, '-' | '0'..='9' | '\u{00b7}' | '\u{0300}'..='\u{036f}' | '\u{203f}'..='\u{2040}')
}
