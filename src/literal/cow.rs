use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use into_owned_trait::IntoOwned;
use langtag::LangTag;

use crate::RdfDisplay;

use super::{CowLiteralType, Literal, LiteralRef};

/// Copy-on-write RDF literal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CowLiteral<'a> {
	/// Literal value.
	pub value: Cow<'a, str>,

	/// Literal type.
	pub r#type: CowLiteralType<'a>,
}

impl<'a> CowLiteral<'a> {
	/// Creates a new copy-on-write literal with the given value and type.
	pub fn new(value: impl Into<Cow<'a, str>>, type_: impl Into<CowLiteralType<'a>>) -> Self {
		Self {
			value: value.into(),
			r#type: type_.into(),
		}
	}

	/// Returns the literal value as a string slice.
	pub fn as_str(&self) -> &str {
		&self.value
	}

	/// Returns the literal value as a byte slice.
	pub fn as_bytes(&self) -> &[u8] {
		self.value.as_bytes()
	}

	/// Checks if this is a language-tagged string (`rdf:langString`).
	pub fn is_lang_string(&self) -> bool {
		self.r#type.is_lang_string()
	}

	/// Returns the language tag of this literal, if it is a language-tagged
	/// string.
	pub fn lang_tag(&self) -> Option<&LangTag> {
		self.r#type.lang_tag()
	}

	/// Returns a reference to this literal.
	pub fn as_ref(&self) -> LiteralRef<'_> {
		LiteralRef::new(&self.value, self.r#type.as_ref())
	}

	/// Clones the literal, if needed, to get rid of the borrow.
	pub fn into_owned(self) -> Literal {
		Literal::new(self.value, self.r#type)
	}
}

impl IntoOwned for CowLiteral<'_> {
	type Owned = Literal;

	fn into_owned(self) -> Literal {
		Literal::new(self.value, self.r#type)
	}
}

impl<'a> From<&'a Literal> for CowLiteral<'a> {
	fn from(value: &'a Literal) -> Self {
		Self::new(&value.value, value.r#type.as_ref())
	}
}

impl<'a> From<LiteralRef<'a>> for CowLiteral<'a> {
	fn from(value: LiteralRef<'a>) -> Self {
		Self::new(value.value, value.r#type)
	}
}

impl From<Literal> for CowLiteral<'_> {
	fn from(value: Literal) -> Self {
		value.into_cow()
	}
}

impl From<CowLiteral<'_>> for Literal {
	fn from(value: CowLiteral<'_>) -> Self {
		value.into_owned()
	}
}

impl PartialEq<Literal> for CowLiteral<'_> {
	fn eq(&self, other: &Literal) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowLiteral<'_>> for Literal {
	fn eq(&self, other: &CowLiteral<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl<'a> PartialEq<LiteralRef<'a>> for CowLiteral<'_> {
	fn eq(&self, other: &LiteralRef<'a>) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowLiteral<'_>> for LiteralRef<'_> {
	fn eq(&self, other: &CowLiteral<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl PartialOrd<Literal> for CowLiteral<'_> {
	fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowLiteral<'_>> for Literal {
	fn partial_cmp(&self, other: &CowLiteral<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl<'a> PartialOrd<LiteralRef<'a>> for CowLiteral<'_> {
	fn partial_cmp(&self, other: &LiteralRef<'a>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowLiteral<'_>> for LiteralRef<'_> {
	fn partial_cmp(&self, other: &CowLiteral<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl equivalent::Equivalent<Literal> for CowLiteral<'_> {
	fn equivalent(&self, key: &Literal) -> bool {
		self == key
	}
}

impl equivalent::Comparable<Literal> for CowLiteral<'_> {
	fn compare(&self, key: &Literal) -> std::cmp::Ordering {
		self.partial_cmp(key).unwrap()
	}
}

impl AsRef<str> for CowLiteral<'_> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl AsRef<[u8]> for CowLiteral<'_> {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl fmt::Display for CowLiteral<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.r#type.is_xsd_string() {
			Ok(())
		} else {
			self.r#type.rdf_fmt(f)
		}
	}
}

impl RdfDisplay for CowLiteral<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.rdf_fmt(f)?;
		if self.r#type.is_xsd_string() {
			Ok(())
		} else {
			self.r#type.rdf_fmt(f)
		}
	}
}
