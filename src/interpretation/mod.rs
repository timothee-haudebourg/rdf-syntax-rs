//! Resource interpretations.
use std::borrow::Cow;

use iref::Iri;
use rdf_types::Domain;

use crate::{
	BlankId, CowGroundTerm, CowId, CowLiteral, CowTerm, GroundTermRef, IdRef, LiteralRef, TermRef,
};

pub mod fallible;
mod r#impl;

pub use fallible::*;
pub use r#impl::*;

/// RDF ground resource interpretation.
///
/// A ground interpretation maps *ground* lexical terms ([`crate::GroundTerm`],
/// i.e. IRIs and literals) to the resources of a [`Domain`]. See
/// [`Interpretation`] for a refinement that also interprets blank node
/// identifiers.
pub trait GroundInterpretation: Domain {
	/// Interprets the given IRI, if possible.
	fn iri(&self, iri: &Iri) -> Option<Self::Resource>;

	/// Interprets the given literal, if possible.
	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource>;

	/// Interprets the given ground term (an IRI or a literal), if possible.
	fn ground_term<'a>(&self, term: impl Into<GroundTermRef<'a>>) -> Option<Self::Resource> {
		match term.into() {
			GroundTermRef::Iri(iri) => self.iri(iri),
			GroundTermRef::Literal(l) => self.literal(l),
		}
	}
}

/// Mutable ground interpretation, able to create new interpreted resources
/// on demand.
pub trait GroundInterpretationMut: GroundInterpretation {
	/// Interprets the given IRI, inserting a fresh resource for it if none
	/// exists yet.
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource;

	/// Interprets the given literal, inserting a fresh resource for it if
	/// none exists yet.
	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource;

	/// Interprets the given ground term, inserting a fresh resource for it
	/// if none exists yet.
	fn insert_ground_term<'a>(&mut self, term: impl Into<CowGroundTerm<'a>>) -> Self::Resource {
		match term.into() {
			CowGroundTerm::Iri(iri) => self.insert_iri(iri),
			CowGroundTerm::Literal(literal) => self.insert_literal(literal),
		}
	}
}

/// Reverse ground interpretation, listing the ground lexical terms of a
/// given resource.
pub trait ReverseGroundInterpretation: GroundInterpretation {
	/// Iterator over the IRIs of a resource.
	type Iris<'a>: Iterator<Item = &'a Iri>
	where
		Self: 'a;

	/// Iterator over the literals of a resource.
	type Literals<'a>: Iterator<Item = LiteralRef<'a>>
	where
		Self: 'a;

	/// Returns an iterator over the IRIs of `resource`.
	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a>;

	/// Returns an iterator over the literals of `resource`.
	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a>;

	/// Returns an iterator over the ground terms (IRIs and literals) of
	/// `resource`.
	fn ground_terms_of<'a>(&'a self, resource: &'a Self::Resource) -> GroundTermsOf<'a, Self> {
		GroundTermsOf {
			iris: self.iris_of(resource),
			literals: self.literals_of(resource),
		}
	}

	/// Checks whether `resource` has no ground term lexical representation.
	fn is_anonymous(&self, resource: &Self::Resource) -> bool {
		self.ground_terms_of(resource).next().is_none()
	}
}

/// Iterator over the ground terms of a resource.
///
/// Created by [`ReverseGroundInterpretation::ground_terms_of`].
pub struct GroundTermsOf<'a, I: 'a + ?Sized + ReverseGroundInterpretation> {
	iris: I::Iris<'a>,
	literals: I::Literals<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseGroundInterpretation> Iterator for GroundTermsOf<'a, I> {
	type Item = GroundTermRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iris
			.next()
			.map(GroundTermRef::Iri)
			.or_else(|| self.literals.next().map(GroundTermRef::Literal))
	}
}

/// RDF resource interpretation.
///
/// In addition to ground terms, a full interpretation also maps blank node
/// identifiers to resources.
pub trait Interpretation: GroundInterpretation {
	/// Interprets the given blank node identifier, if possible.
	fn blank_id(&self, blank_id: &BlankId) -> Option<Self::Resource>;

	/// Interprets the given term (a blank node identifier or a ground term),
	/// if possible.
	fn term<'a>(&self, term: impl Into<TermRef<'a>>) -> Option<Self::Resource> {
		match term.into() {
			TermRef::BlankId(blank_id) => self.blank_id(blank_id),
			TermRef::Ground(term) => self.ground_term(term),
		}
	}

	/// Interprets the given identifier (a blank node identifier or an IRI),
	/// if possible.
	fn id<'a>(&self, id: impl Into<IdRef<'a>>) -> Option<Self::Resource> {
		match id.into() {
			IdRef::BlankId(blank_id) => self.blank_id(blank_id),
			IdRef::Iri(iri) => self.iri(iri),
		}
	}
}

/// Mutable resource interpretation, able to create new interpreted
/// resources on demand.
pub trait InterpretationMut: Interpretation + GroundInterpretationMut {
	/// Interprets the given blank node identifier, inserting a fresh
	/// resource for it if none exists yet.
	fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource;

	/// Interprets the given term, inserting a fresh resource for it if none
	/// exists yet.
	fn insert_term<'a>(&mut self, term: impl Into<CowTerm<'a>>) -> Self::Resource {
		match term.into() {
			CowTerm::BlankId(blank_id) => self.insert_blank_id(blank_id),
			CowTerm::Ground(term) => self.insert_ground_term(term),
		}
	}

	/// Interprets the given identifier, inserting a fresh resource for it
	/// if none exists yet.
	fn insert_id<'a>(&mut self, term: impl Into<CowId<'a>>) -> Self::Resource {
		match term.into() {
			CowId::BlankId(blank_id) => self.insert_blank_id(blank_id),
			CowId::Iri(iri) => self.insert_iri(iri),
		}
	}
}

/// Reverse resource interpretation, listing the lexical terms of a given
/// resource.
pub trait ReverseInterpretation: ReverseGroundInterpretation {
	/// Iterator over the blank node identifiers of a resource.
	type BlankIds<'a>: Iterator<Item = &'a BlankId>
	where
		Self: 'a;

	/// Returns an iterator over the blank node identifiers of `resource`.
	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a>;

	/// Returns an iterator over the terms (ground terms and blank node
	/// identifiers) of `resource`.
	fn terms_of<'a>(&'a self, resource: &'a Self::Resource) -> TermsOf<'a, Self> {
		TermsOf {
			terms: self.ground_terms_of(resource),
			blank_ids: self.blank_ids_of(resource),
		}
	}
}

/// Iterator over the terms of a resource.
///
/// Created by [`ReverseInterpretation::terms_of`].
pub struct TermsOf<'a, I: 'a + ?Sized + ReverseInterpretation> {
	terms: GroundTermsOf<'a, I>,
	blank_ids: I::BlankIds<'a>,
}

impl<'a, I: 'a + ?Sized + ReverseInterpretation> Iterator for TermsOf<'a, I> {
	type Item = TermRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.terms
			.next()
			.map(TermRef::Ground)
			.or_else(|| self.blank_ids.next().map(TermRef::BlankId))
	}
}
