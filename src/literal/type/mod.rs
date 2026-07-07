use core::fmt;
use iref::{Iri, IriBuf};
use langtag::{LangTag, LangTagBuf};
use std::{borrow::Cow, fmt::Write};

use crate::{RdfDisplay, XSD_STRING};

mod cow;
mod r#ref;

pub use cow::*;
pub use r#ref::*;

/// RDF literal type.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiteralType {
	/// Any type.
	Any(IriBuf),

	/// Language string.
	LangString(LangTagBuf),
}

impl LiteralType {
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

	/// Turns this type into the datatype IRI, unless this is a
	/// language-tagged string.
	pub fn into_any(self) -> Option<IriBuf> {
		match self {
			Self::Any(iri) => Some(iri),
			_ => None,
		}
	}

	/// Turns this type into its language tag, if it is a language-tagged
	/// string.
	pub fn into_lang_string(self) -> Option<LangTagBuf> {
		match self {
			Self::LangString(tag) => Some(tag),
			_ => None,
		}
	}

	/// Checks if this is the `xsd:string` datatype.
	pub fn is_xsd_string(&self) -> bool {
		self.is_iri(XSD_STRING)
	}

	/// Checks if this is the given datatype IRI.
	pub fn is_iri(&self, iri: &Iri) -> bool {
		match self {
			Self::Any(i) => i == iri,
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

	/// Returns a copy-on-write reference to this literal type.
	pub fn as_cow(&self) -> CowLiteralType<'_> {
		match self {
			Self::Any(i) => CowLiteralType::Any(Cow::Borrowed(i)),
			Self::LangString(l) => CowLiteralType::LangString(Cow::Borrowed(l)),
		}
	}

	/// Turns this literal type into a copy-on-write literal type.
	pub fn into_cow(self) -> CowLiteralType<'static> {
		match self {
			Self::Any(i) => CowLiteralType::Any(Cow::Owned(i)),
			Self::LangString(l) => CowLiteralType::LangString(Cow::Owned(l)),
		}
	}
}

impl From<&Iri> for LiteralType {
	fn from(value: &Iri) -> Self {
		Self::Any(value.to_owned())
	}
}

impl From<IriBuf> for LiteralType {
	fn from(value: IriBuf) -> Self {
		Self::Any(value)
	}
}

impl From<&LangTag> for LiteralType {
	fn from(value: &LangTag) -> Self {
		Self::LangString(value.to_owned())
	}
}

impl From<LangTagBuf> for LiteralType {
	fn from(value: LangTagBuf) -> Self {
		Self::LangString(value)
	}
}

impl fmt::Display for LiteralType {
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

impl RdfDisplay for LiteralType {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self, f)
	}
}
