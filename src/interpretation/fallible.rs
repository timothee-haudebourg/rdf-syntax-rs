use core::fmt;
use std::{borrow::Cow, convert::Infallible};

use into_owned_trait::MapIntoOwned;
use iref::{Iri, IriBuf};
use rdf_types::Domain;

use super::GroundInterpretation;
use crate::{
	BlankId, BlankIdBuf, CowGroundTerm, CowId, CowLiteral, CowTerm, GroundInterpretationMut,
	GroundTerm, GroundTermRef, IdRef, Literal, LiteralRef, Term, TermRef,
	interpretation::{
		Interpretation, InterpretationMut, ReverseGroundInterpretation, ReverseInterpretation,
	},
	util::InfallibleIterator,
};

/// Fallible ground interpretation.
pub trait TryGroundInterpretation: Domain {
	/// Error type.
	type Error: fmt::Debug + fmt::Display;

	fn try_iri(&self, iri: &Iri) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_literal<'a>(
		&self,
		literal: impl Into<LiteralRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_ground_term<'a>(
		&self,
		term: impl Into<GroundTermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		match term.into() {
			GroundTermRef::Iri(iri) => self.try_iri(iri),
			GroundTermRef::Literal(l) => self.try_literal(l),
		}
	}
}

/// Any non-fallible interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: GroundInterpretation> TryGroundInterpretation for I {
	type Error = Infallible;

	fn try_iri(&self, iri: &Iri) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.iri(iri))
	}

	fn try_literal<'a>(
		&self,
		literal: impl Into<LiteralRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.literal(literal))
	}

	fn try_ground_term<'a>(
		&self,
		term: impl Into<GroundTermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.ground_term(term))
	}
}

/// Mutable fallible interpretation.
pub trait TryGroundInterpretationMut: TryGroundInterpretation {
	fn try_insert_iri<'a>(
		&mut self,
		iri: impl Into<Cow<'a, Iri>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_literal<'a>(
		&mut self,
		literal: impl Into<CowLiteral<'a>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_ground_term<'a>(
		&mut self,
		term: impl Into<CowGroundTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowGroundTerm::Iri(iri) => self.try_insert_iri(iri),
			CowGroundTerm::Literal(literal) => self.try_insert_literal(literal),
		}
	}
}

/// Any mutable ground interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: GroundInterpretationMut> TryGroundInterpretationMut for I {
	fn try_insert_iri<'a>(
		&mut self,
		iri: impl Into<Cow<'a, Iri>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_iri(iri))
	}

	fn try_insert_literal<'a>(
		&mut self,
		literal: impl Into<CowLiteral<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_literal(literal))
	}

	fn try_insert_ground_term<'a>(
		&mut self,
		term: impl Into<CowGroundTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_ground_term(term))
	}
}

/// Fallible reverse ground interpretation.
pub trait TryReverseGroundInterpretation: TryGroundInterpretation {
	type TryIris<'a>: Iterator<Item = Result<IriBuf, Self::Error>>
	where
		Self: 'a;
	type TryLiterals<'a>: Iterator<Item = Result<Literal, Self::Error>>
	where
		Self: 'a;

	fn try_iris_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryIris<'a>, Self::Error>;

	fn try_literals_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryLiterals<'a>, Self::Error>;

	fn try_ground_terms_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<TryGroundTermsOf<'a, Self>, Self::Error> {
		Ok(TryGroundTermsOf {
			iris: self.try_iris_of(resource)?,
			literals: self.try_literals_of(resource)?,
		})
	}

	fn try_is_anonymous(&self, resource: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self
			.try_ground_terms_of(resource)?
			.next()
			.transpose()?
			.is_none())
	}
}

impl<I: ReverseGroundInterpretation> TryReverseGroundInterpretation for I {
	type TryIris<'a>
		= InfallibleIterator<MapIntoOwned<I::Iris<'a>>>
	where
		Self: 'a;

	type TryLiterals<'a>
		= InfallibleIterator<MapIntoOwned<I::Literals<'a>>>
	where
		Self: 'a;

	fn try_iris_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryIris<'a>, Self::Error> {
		Ok(InfallibleIterator(MapIntoOwned(self.iris_of(resource))))
	}

	fn try_literals_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryLiterals<'a>, Self::Error> {
		Ok(InfallibleIterator(MapIntoOwned(self.literals_of(resource))))
	}

	fn try_is_anonymous(&self, resource: &Self::Resource) -> Result<bool, Self::Error> {
		Ok(self.is_anonymous(resource))
	}
}

pub struct TryGroundTermsOf<'a, I: 'a + ?Sized + TryReverseGroundInterpretation> {
	iris: I::TryIris<'a>,
	literals: I::TryLiterals<'a>,
}

impl<'a, I: 'a + ?Sized + TryReverseGroundInterpretation> Iterator for TryGroundTermsOf<'a, I> {
	type Item = Result<GroundTerm, I::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(|r| r.map(GroundTerm::Iri))
			.or_else(|| self.literals.next().map(|r| r.map(GroundTerm::Literal)))
	}
}

/// Fallible interpretation.
///
/// Same as a fallible ground interpretation, but also interprets blank node
/// identifiers.
pub trait TryInterpretation: TryGroundInterpretation {
	fn try_blank_id(&self, blank_id: &BlankId) -> Result<Option<Self::Resource>, Self::Error>;

	fn try_term<'a>(
		&self,
		term: impl Into<TermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		match term.into() {
			TermRef::BlankId(blank_id) => self.try_blank_id(blank_id),
			TermRef::Ground(term) => self.try_ground_term(term),
		}
	}

	fn try_id<'a>(&self, id: impl Into<IdRef<'a>>) -> Result<Option<Self::Resource>, Self::Error> {
		match id.into() {
			IdRef::BlankId(blank_id) => self.try_blank_id(blank_id),
			IdRef::Iri(iri) => self.try_iri(iri),
		}
	}
}

/// Any non-fallible interpretation is can be used a fallible, with the
/// [`Infallible`] error type.
impl<I: Interpretation> TryInterpretation for I {
	fn try_blank_id(&self, blank_id: &BlankId) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.blank_id(blank_id))
	}

	fn try_term<'a>(
		&self,
		term: impl Into<TermRef<'a>>,
	) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.term(term))
	}

	fn try_id<'a>(&self, id: impl Into<IdRef<'a>>) -> Result<Option<Self::Resource>, Self::Error> {
		Ok(self.id(id))
	}
}

/// Mutable fallible interpretation.
pub trait TryInterpretationMut: TryInterpretation + TryGroundInterpretationMut {
	fn try_insert_blank_id<'a>(
		&mut self,
		blank_id: impl Into<Cow<'a, BlankId>>,
	) -> Result<Self::Resource, Self::Error>;

	fn try_insert_term<'a>(
		&mut self,
		term: impl Into<CowTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowTerm::BlankId(blank_id) => self.try_insert_blank_id(blank_id),
			CowTerm::Ground(term) => self.try_insert_ground_term(term),
		}
	}

	fn try_insert_id<'a>(
		&mut self,
		term: impl Into<CowId<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		match term.into() {
			CowId::BlankId(blank_id) => self.try_insert_blank_id(blank_id),
			CowId::Iri(iri) => self.try_insert_iri(iri),
		}
	}
}

/// Any non-fallible mutable interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: InterpretationMut> TryInterpretationMut for I {
	fn try_insert_blank_id<'a>(
		&mut self,
		blank_id: impl Into<Cow<'a, BlankId>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_blank_id(blank_id))
	}

	fn try_insert_term<'a>(
		&mut self,
		term: impl Into<CowTerm<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_term(term))
	}

	fn try_insert_id<'a>(
		&mut self,
		term: impl Into<CowId<'a>>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(self.insert_id(term))
	}
}

/// Fallible reverse interpretation.
pub trait TryReverseInterpretation: TryReverseGroundInterpretation {
	type TryBlankIds<'a>: Iterator<Item = Result<BlankIdBuf, Self::Error>>
	where
		Self: 'a;

	fn try_blank_ids_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryBlankIds<'a>, Self::Error>;

	fn try_terms_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<TryTermsOf<'a, Self>, Self::Error> {
		Ok(TryTermsOf {
			terms: self.try_ground_terms_of(resource)?,
			blank_ids: self.try_blank_ids_of(resource)?,
		})
	}
}

/// Any non-fallible reverse interpretation can be used as fallible, with the
/// [`Infallible`] error type.
impl<I: ReverseInterpretation> TryReverseInterpretation for I {
	type TryBlankIds<'a>
		= InfallibleIterator<MapIntoOwned<I::BlankIds<'a>>>
	where
		Self: 'a;

	fn try_blank_ids_of<'a>(
		&'a self,
		resource: &'a Self::Resource,
	) -> Result<Self::TryBlankIds<'a>, Self::Error> {
		Ok(InfallibleIterator(MapIntoOwned(
			self.blank_ids_of(resource),
		)))
	}
}

pub struct TryTermsOf<'a, I: 'a + ?Sized + TryReverseInterpretation> {
	terms: TryGroundTermsOf<'a, I>,
	blank_ids: I::TryBlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + TryReverseInterpretation> Iterator for TryTermsOf<'a, I> {
	type Item = Result<Term, I::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self.terms
			.next()
			.map(|r| r.map(Term::Ground))
			.or_else(|| self.blank_ids.next().map(|r| r.map(Term::BlankId)))
	}
}
