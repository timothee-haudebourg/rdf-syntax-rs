use core::fmt;
use std::borrow::Cow;
use std::cmp::Ordering;

use into_owned_trait::IntoOwned;
use iref::{Iri, IriBuf};

use crate::{BlankId, BlankIdBuf, Id, IdRef, RdfDisplay};

/// Copy-on-write resource identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CowId<'a> {
	/// Blank node identifier.
	BlankId(Cow<'a, BlankId>),

	/// IRI.
	Iri(Cow<'a, Iri>),
}

impl CowId<'_> {
	/// Checks if this is a blank node identifier.
	pub fn is_blank_id(&self) -> bool {
		matches!(self, Self::BlankId(_))
	}

	/// Checks if this is an IRI.
	pub fn is_iri(&self) -> bool {
		matches!(self, Self::Iri(_))
	}

	/// Returns this identifier as a blank node identifier, if it is one.
	pub fn as_blank_id(&self) -> Option<&BlankId> {
		match self {
			Self::BlankId(b) => Some(b),
			_ => None,
		}
	}

	/// Returns this identifier as an IRI, if it is one.
	pub fn as_iri(&self) -> Option<&Iri> {
		match self {
			Self::Iri(iri) => Some(iri),
			_ => None,
		}
	}

	/// Returns a reference to this identifier.
	pub fn as_ref(&self) -> IdRef<'_> {
		match self {
			Self::BlankId(b) => IdRef::BlankId(b),
			Self::Iri(iri) => IdRef::Iri(iri),
		}
	}

	/// Clones the identifier, if needed, to get rid of the borrow.
	pub fn into_owned(self) -> Id {
		match self {
			Self::BlankId(b) => Id::BlankId(b.into_owned()),
			Self::Iri(iri) => Id::Iri(iri.into_owned()),
		}
	}
}

impl AsRef<str> for CowId<'_> {
	fn as_ref(&self) -> &str {
		match self {
			Self::BlankId(b) => b.as_ref(),
			Self::Iri(i) => i.as_str(),
		}
	}
}

impl IntoOwned for CowId<'_> {
	type Owned = Id;

	fn into_owned(self) -> Id {
		match self {
			Self::BlankId(b) => Id::BlankId(b.into_owned()),
			Self::Iri(iri) => Id::Iri(iri.into_owned()),
		}
	}
}

impl equivalent::Equivalent<Id> for CowId<'_> {
	fn equivalent(&self, key: &Id) -> bool {
		self == key
	}
}

impl equivalent::Comparable<Id> for CowId<'_> {
	fn compare(&self, key: &Id) -> std::cmp::Ordering {
		self.partial_cmp(key).unwrap()
	}
}

impl From<Id> for CowId<'_> {
	fn from(value: Id) -> Self {
		value.into_cow()
	}
}

impl<'a> From<&'a Id> for CowId<'a> {
	fn from(value: &'a Id) -> Self {
		value.as_cow()
	}
}

impl<'a> From<IdRef<'a>> for CowId<'a> {
	fn from(value: IdRef<'a>) -> Self {
		value.into_cow()
	}
}

impl From<CowId<'_>> for Id {
	fn from(value: CowId<'_>) -> Self {
		value.into_owned()
	}
}

impl PartialEq<Iri> for CowId<'_> {
	fn eq(&self, other: &Iri) -> bool {
		match self {
			Self::Iri(iri) => iri.as_ref() == other,
			_ => false,
		}
	}
}

impl PartialEq<&Iri> for CowId<'_> {
	fn eq(&self, other: &&Iri) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<IriBuf> for CowId<'_> {
	fn eq(&self, other: &IriBuf) -> bool {
		match self {
			Self::Iri(iri) => iri.as_ref() == other.as_iri(),
			_ => false,
		}
	}
}

impl PartialEq<&IriBuf> for CowId<'_> {
	fn eq(&self, other: &&IriBuf) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<BlankId> for CowId<'_> {
	fn eq(&self, other: &BlankId) -> bool {
		match self {
			Self::BlankId(b) => b.as_ref() == other,
			_ => false,
		}
	}
}

impl PartialEq<BlankIdBuf> for CowId<'_> {
	fn eq(&self, other: &BlankIdBuf) -> bool {
		match self {
			Self::BlankId(b) => b.as_ref() == other.as_blank_id(),
			_ => false,
		}
	}
}

impl PartialEq<&BlankId> for CowId<'_> {
	fn eq(&self, other: &&BlankId) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<&BlankIdBuf> for CowId<'_> {
	fn eq(&self, other: &&BlankIdBuf) -> bool {
		self.eq(*other)
	}
}

impl PartialEq<Id> for CowId<'_> {
	fn eq(&self, other: &Id) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl PartialEq<CowId<'_>> for Id {
	fn eq(&self, other: &CowId<'_>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl PartialEq<IdRef<'_>> for CowId<'_> {
	fn eq(&self, other: &IdRef<'_>) -> bool {
		self.as_ref() == *other
	}
}

impl PartialEq<CowId<'_>> for IdRef<'_> {
	fn eq(&self, other: &CowId<'_>) -> bool {
		*self == other.as_ref()
	}
}

impl PartialOrd<Id> for CowId<'_> {
	fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
		self.as_ref().partial_cmp(&other.as_ref())
	}
}

impl PartialOrd<CowId<'_>> for Id {
	fn partial_cmp(&self, other: &CowId<'_>) -> Option<Ordering> {
		self.as_ref().partial_cmp(&other.as_ref())
	}
}

impl PartialOrd<IdRef<'_>> for CowId<'_> {
	fn partial_cmp(&self, other: &IdRef<'_>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl PartialOrd<CowId<'_>> for IdRef<'_> {
	fn partial_cmp(&self, other: &CowId<'_>) -> Option<Ordering> {
		self.partial_cmp(&other.as_ref())
	}
}

impl fmt::Display for CowId<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.fmt(f),
			Self::Iri(i) => i.fmt(f),
		}
	}
}

impl RdfDisplay for CowId<'_> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::BlankId(b) => b.rdf_fmt(f),
			Self::Iri(i) => i.rdf_fmt(f),
		}
	}
}
