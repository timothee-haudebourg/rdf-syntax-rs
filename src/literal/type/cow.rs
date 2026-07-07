use core::fmt;
use std::{borrow::Cow, cmp::Ordering, fmt::Write};

use into_owned_trait::IntoOwned;
use iref::{Iri, IriBuf};
use langtag::{LangTag, LangTagBuf};

use crate::{RDF_LANG_STRING, RdfDisplay};

use super::{LiteralType, LiteralTypeRef};

/// Owned or referenced RDF literal type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum CowLiteralType<'a> {
	/// Any type.
	Any(Cow<'a, Iri>),

	/// Language string.
	LangString(Cow<'a, LangTag>),
}

impl CowLiteralType<'_> {
	/// Checks if this is the general `Any` case (as opposed to a language
	/// string).
	pub fn is_any(&self) -> bool {
		matches!(self, Self::Any(_))
	}

	/// Checks if this is a language-tagged string (`rdf:langString`).
	pub fn is_lang_string(&self) -> bool {
		matches!(self, Self::LangString(_))
	}

	/// Returns the datatype IRI, unless this is a language-tagged string.
	pub fn as_any(&self) -> Option<&Iri> {
		match self {
			Self::Any(iri) => Some(iri),
			_ => None,
		}
	}

	/// Returns the language tag, if this is a language-tagged string.
	pub fn as_lang_string(&self) -> Option<&LangTag> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}

	/// Returns the language tag, if this is a language-tagged string.
	pub fn lang_tag(&self) -> Option<&LangTag> {
		self.as_lang_string()
	}

	/// Checks if this is the `xsd:string` datatype.
	pub fn is_xsd_string(&self) -> bool {
		self.is_iri(crate::XSD_STRING)
	}

	/// Checks if this is the given datatype IRI.
	pub fn is_iri(&self, iri: &Iri) -> bool {
		match self {
			Self::Any(i) => i.as_ref() == iri,
			Self::LangString(_) => false,
		}
	}

	/// Returns a reference to this literal type.
	pub fn as_ref(&self) -> LiteralTypeRef<'_> {
		match self {
			Self::Any(i) => LiteralTypeRef::Any(i),
			Self::LangString(l) => LiteralTypeRef::LangString(l),
		}
	}

	/// Clones the literal type, if needed, to get rid of the borrow.
	pub fn into_owned(self) -> LiteralType {
		match self {
			Self::Any(iri) => LiteralType::Any(iri.into_owned()),
			Self::LangString(lang) => LiteralType::LangString(lang.into_owned()),
		}
	}
}

impl<'a> From<CowLiteralType<'a>> for LiteralType {
	fn from(value: CowLiteralType<'a>) -> Self {
		value.into_owned()
	}
}

impl<'a> From<LiteralTypeRef<'a>> for CowLiteralType<'a> {
	fn from(value: LiteralTypeRef<'a>) -> Self {
		match value {
			LiteralTypeRef::Any(i) => Self::Any(Cow::Borrowed(i)),
			LiteralTypeRef::LangString(t) => Self::LangString(Cow::Borrowed(t)),
		}
	}
}

impl From<LiteralType> for CowLiteralType<'_> {
	fn from(value: LiteralType) -> Self {
		value.into_cow()
	}
}

impl<'a> From<&'a LiteralType> for CowLiteralType<'a> {
	fn from(value: &'a LiteralType) -> Self {
		value.as_cow()
	}
}

impl<'a> From<&'a Iri> for CowLiteralType<'a> {
	fn from(value: &'a Iri) -> Self {
		Self::Any(Cow::Borrowed(value))
	}
}

impl From<IriBuf> for CowLiteralType<'_> {
	fn from(value: IriBuf) -> Self {
		Self::Any(Cow::Owned(value))
	}
}

impl<'a> From<Cow<'a, Iri>> for CowLiteralType<'a> {
	fn from(value: Cow<'a, Iri>) -> Self {
		Self::Any(value)
	}
}

impl<'a> From<&'a LangTag> for CowLiteralType<'a> {
	fn from(value: &'a LangTag) -> Self {
		Self::LangString(Cow::Borrowed(value))
	}
}

impl From<LangTagBuf> for CowLiteralType<'_> {
	fn from(value: LangTagBuf) -> Self {
		Self::LangString(Cow::Owned(value))
	}
}

impl<'a> From<Cow<'a, LangTag>> for CowLiteralType<'a> {
	fn from(value: Cow<'a, LangTag>) -> Self {
		Self::LangString(value)
	}
}

impl PartialEq<Iri> for CowLiteralType<'_> {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Any(ty) => ty.as_ref() == other,
			Self::LangString(_) => RDF_LANG_STRING == other,
		}
	}
}

impl<'a> PartialEq<&'a Iri> for CowLiteralType<'_> {
	fn eq(&self, other: &&'a Iri) -> bool {
		match self {
			Self::Any(ty) => ty.as_ref() == other,
			Self::LangString(_) => RDF_LANG_STRING == other,
		}
	}
}

impl IntoOwned for CowLiteralType<'_> {
	type Owned = LiteralType;

	fn into_owned(self) -> LiteralType {
		match self {
			Self::Any(iri) => LiteralType::Any(iri.into_owned()),
			Self::LangString(lang) => LiteralType::LangString(lang.into_owned()),
		}
	}
}

impl equivalent::Equivalent<LiteralType> for CowLiteralType<'_> {
	fn equivalent(&self, key: &LiteralType) -> bool {
		self == key
	}
}

impl equivalent::Comparable<LiteralType> for CowLiteralType<'_> {
	fn compare(&self, key: &LiteralType) -> std::cmp::Ordering {
		self.partial_cmp(key).unwrap()
	}
}

impl PartialEq<LiteralType> for CowLiteralType<'_> {
	fn eq(&self, other: &LiteralType) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl PartialEq<CowLiteralType<'_>> for LiteralType {
	fn eq(&self, other: &CowLiteralType<'_>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a> PartialEq<LiteralTypeRef<'a>> for CowLiteralType<'_> {
	fn eq(&self, other: &LiteralTypeRef<'a>) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowLiteralType<'_>> for LiteralTypeRef<'_> {
	fn eq(&self, other: &CowLiteralType<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl PartialOrd<LiteralType> for CowLiteralType<'_> {
	fn partial_cmp(&self, other: &LiteralType) -> Option<Ordering> {
		self.as_ref().partial_cmp(&other.as_ref())
	}
}

impl PartialOrd<CowLiteralType<'_>> for LiteralType {
	fn partial_cmp(&self, other: &CowLiteralType<'_>) -> Option<Ordering> {
		self.as_ref().partial_cmp(&other.as_ref())
	}
}

impl<'a> PartialOrd<LiteralTypeRef<'a>> for CowLiteralType<'_> {
	fn partial_cmp(&self, other: &LiteralTypeRef<'a>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowLiteralType<'_>> for LiteralTypeRef<'_> {
	fn partial_cmp(&self, other: &CowLiteralType<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl fmt::Display for CowLiteralType<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Any(ty) => {
				f.write_str("^^")?;
				ty.rdf_fmt(f)
			}
			Self::LangString(tag) => {
				f.write_char('@')?;
				tag.rdf_fmt(f)
			}
		}
	}
}

impl RdfDisplay for CowLiteralType<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}
