use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use into_owned_trait::IntoOwned;
use iref::Iri;

use crate::{LiteralRef, RdfDisplay};

use super::{CowGroundTerm, GroundTerm};

/// Ground term reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroundTermRef<'a> {
	/// IRI.
	Iri(&'a Iri),

	/// Lexical value.
	Literal(LiteralRef<'a>),
}

impl<'a> GroundTermRef<'a> {
	/// Checks if this is an IRI.
	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	/// Checks if this is a literal.
	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Literal(_))
	}

	/// Returns this ground term as an IRI, if it is one.
	pub fn as_iri(&self) -> Option<&'a Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	/// Returns this ground term as a literal, if it is one.
	pub fn as_literal(&self) -> Option<LiteralRef<'a>> {
		match self {
			Self::Literal(l) => Some(*l),
			_ => None,
		}
	}

	/// Turns this ground term reference into a copy-on-write ground term.
	pub fn into_cow(self) -> CowGroundTerm<'a> {
		match self {
			Self::Iri(iri) => CowGroundTerm::Iri(Cow::Borrowed(iri)),
			Self::Literal(l) => CowGroundTerm::Literal(l.into()),
		}
	}
}

impl GroundTermRef<'_> {
	/// Clones the referenced ground term.
	pub fn to_owned(&self) -> GroundTerm {
		(*self).into_owned()
	}
}

impl IntoOwned for GroundTermRef<'_> {
	type Owned = GroundTerm;

	fn into_owned(self) -> GroundTerm {
		match self {
			Self::Iri(iri) => GroundTerm::Iri(iri.to_owned()),
			Self::Literal(l) => GroundTerm::Literal(l.into_owned()),
		}
	}
}

impl equivalent::Equivalent<GroundTerm> for GroundTermRef<'_> {
	fn equivalent(&self, key: &GroundTerm) -> bool {
		self == key
	}
}

impl equivalent::Comparable<GroundTerm> for GroundTermRef<'_> {
	fn compare(&self, key: &GroundTerm) -> std::cmp::Ordering {
		self.partial_cmp(key).unwrap()
	}
}

impl PartialEq<GroundTerm> for GroundTermRef<'_> {
	fn eq(&self, other: &GroundTerm) -> bool {
		match (self, other) {
			(Self::Iri(a), GroundTerm::Iri(b)) => *a == b,
			(Self::Literal(a), GroundTerm::Literal(b)) => a == b,
			_ => false,
		}
	}
}

impl PartialOrd<GroundTerm> for GroundTermRef<'_> {
	fn partial_cmp(&self, other: &GroundTerm) -> Option<Ordering> {
		match (self, other) {
			(Self::Iri(a), GroundTerm::Iri(b)) => (*a).partial_cmp(b),
			(Self::Iri(_), GroundTerm::Literal(_)) => Some(Ordering::Less),
			(Self::Literal(_), GroundTerm::Iri(_)) => Some(Ordering::Greater),
			(Self::Literal(a), GroundTerm::Literal(b)) => (*a).partial_cmp(b),
		}
	}
}

impl fmt::Display for GroundTermRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(iri) => iri.fmt(f),
			Self::Literal(lit) => lit.fmt(f),
		}
	}
}

impl RdfDisplay for GroundTermRef<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Iri(iri) => iri.rdf_fmt(f),
			Self::Literal(lit) => lit.rdf_fmt(f),
		}
	}
}
