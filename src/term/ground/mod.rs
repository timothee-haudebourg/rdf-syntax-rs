use std::cmp::Ordering;
use std::hash::Hash;
use std::{borrow::Cow, fmt};

use iref::{Iri, IriBuf};

use crate::{Literal, LiteralRef, RdfDisplay};

mod cow;
mod r#ref;

pub use cow::*;
pub use r#ref::*;

/// Term excluding blank node identifiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GroundTerm {
	/// IRI.
	Iri(IriBuf),

	/// Literal value.
	Literal(Literal),
}

impl GroundTerm {
	/// Checks if this is an IRI.
	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	/// Checks if this is a literal.
	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	/// Returns this ground term as a literal, if it is one.
	pub fn as_literal(&self) -> Option<LiteralRef<'_>> {
		match self {
			Self::Literal(lit) => Some(lit.as_ref()),
			_ => None,
		}
	}

	/// Turns this ground term into a literal, if it is one.
	pub fn into_literal(self) -> Option<Literal> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}

	/// Turns this ground term into a literal, or returns the IRI if it isn't
	/// one.
	pub fn try_into_literal(self) -> Result<Literal, IriBuf> {
		match self {
			Self::Literal(lit) => Ok(lit),
			Self::Iri(id) => Err(id),
		}
	}

	/// Returns this ground term as an IRI, if it is one.
	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(id) => Some(id),
			_ => None,
		}
	}

	/// Turns this ground term into an IRI, or returns it unchanged if it
	/// isn't one.
	pub fn try_into_iri(self) -> Result<IriBuf, Self> {
		match self {
			Self::Iri(iri) => Ok(iri),
			other => Err(other),
		}
	}

	/// Turns this ground term into an IRI, if it is one.
	pub fn into_iri(self) -> Option<IriBuf> {
		self.try_into_iri().ok()
	}

	/// Returns a reference to this ground term.
	pub fn as_ref(&self) -> GroundTermRef<'_> {
		match self {
			Self::Iri(id) => GroundTermRef::Iri(id),
			Self::Literal(l) => GroundTermRef::Literal(l.as_ref()),
		}
	}

	/// Returns a copy-on-write reference to this ground term.
	pub fn as_cow(&self) -> CowGroundTerm<'_> {
		match self {
			Self::Iri(id) => CowGroundTerm::Iri(Cow::Borrowed(id)),
			Self::Literal(l) => CowGroundTerm::Literal(l.as_cow()),
		}
	}

	/// Turns this ground term into a copy-on-write ground term.
	pub fn into_cow(self) -> CowGroundTerm<'static> {
		match self {
			Self::Iri(id) => CowGroundTerm::Iri(Cow::Owned(id)),
			Self::Literal(l) => CowGroundTerm::Literal(l.into_cow()),
		}
	}
}

impl Hash for GroundTerm {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Iri(id) => id.hash(state),
			Self::Literal(l) => l.hash(state),
		}
	}
}

impl fmt::Display for GroundTerm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(id) => id.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for GroundTerm {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(id) => id.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}

impl<'a> PartialEq<GroundTermRef<'a>> for GroundTerm {
	fn eq(&self, other: &GroundTermRef<'a>) -> bool {
		match (self, other) {
			(Self::Iri(a), GroundTermRef::Iri(b)) => a == *b,
			(Self::Literal(a), GroundTermRef::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl<'a> PartialOrd<GroundTermRef<'a>> for GroundTerm {
	fn partial_cmp(&self, other: &GroundTermRef<'a>) -> Option<Ordering> {
		match (self, other) {
			(Self::Iri(a), GroundTermRef::Iri(b)) => (*a).partial_cmp(b),
			(Self::Iri(_), GroundTermRef::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(_), GroundTermRef::Iri(_)) => Some(Ordering::Greater),
			(Self::Literal(a), GroundTermRef::Literal(b)) => (*a).partial_cmp(b),
		}
	}
}
