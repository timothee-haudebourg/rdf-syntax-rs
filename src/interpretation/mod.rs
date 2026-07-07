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

/// RDF resource interpretation.
pub trait GroundInterpretation: Domain {
    fn iri(&self, iri: &Iri) -> Option<Self::Resource>;

    fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource>;

    fn ground_term<'a>(&self, term: impl Into<GroundTermRef<'a>>) -> Option<Self::Resource> {
        match term.into() {
            GroundTermRef::Iri(iri) => self.iri(iri),
            GroundTermRef::Literal(l) => self.literal(l),
        }
    }
}

/// Mutable interpretation.
pub trait GroundInterpretationMut: GroundInterpretation {
    fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource;

    fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource;

    fn insert_ground_term<'a>(&mut self, term: impl Into<CowGroundTerm<'a>>) -> Self::Resource {
        match term.into() {
            CowGroundTerm::Iri(iri) => self.insert_iri(iri),
            CowGroundTerm::Literal(literal) => self.insert_literal(literal),
        }
    }
}

/// Reverse interpretation.
pub trait ReverseGroundInterpretation: GroundInterpretation {
    type Iris<'a>: Iterator<Item = &'a Iri>
    where
        Self: 'a;
    type Literals<'a>: Iterator<Item = LiteralRef<'a>>
    where
        Self: 'a;

    fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a>;

    fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a>;

    fn terms_of<'a>(&'a self, resource: &'a Self::Resource) -> GroundTermsOf<'a, Self> {
        GroundTermsOf {
            iris: self.iris_of(resource),
            literals: self.literals_of(resource),
        }
    }

    fn is_anonymous(&self, resource: &Self::Resource) -> bool {
        self.terms_of(resource).next().is_none()
    }
}

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

pub trait Interpretation: GroundInterpretation {
    fn blank_id(&self, blank_id: &BlankId) -> Option<Self::Resource>;

    fn term<'a>(&self, term: impl Into<TermRef<'a>>) -> Option<Self::Resource> {
        match term.into() {
            TermRef::BlankId(blank_id) => self.blank_id(blank_id),
            TermRef::Ground(term) => self.ground_term(term),
        }
    }

    fn id<'a>(&self, id: impl Into<IdRef<'a>>) -> Option<Self::Resource> {
        match id.into() {
            IdRef::BlankId(blank_id) => self.blank_id(blank_id),
            IdRef::Iri(iri) => self.iri(iri),
        }
    }
}

pub trait InterpretationMut: Interpretation + GroundInterpretationMut {
    fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource;

    fn insert_term<'a>(&mut self, term: impl Into<CowTerm<'a>>) -> Self::Resource {
        match term.into() {
            CowTerm::BlankId(blank_id) => self.insert_blank_id(blank_id),
            CowTerm::Ground(term) => self.insert_ground_term(term),
        }
    }

    fn insert_id<'a>(&mut self, term: impl Into<CowId<'a>>) -> Self::Resource {
        match term.into() {
            CowId::BlankId(blank_id) => self.insert_blank_id(blank_id),
            CowId::Iri(iri) => self.insert_iri(iri),
        }
    }
}

pub trait ReverseInterpretation: ReverseGroundInterpretation {
    type BlankIds<'a>: Iterator<Item = &'a BlankId>
    where
        Self: 'a;

    fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a>;

    fn local_terms_of<'a>(&'a self, resource: &'a Self::Resource) -> TermsOf<'a, Self> {
        TermsOf {
            terms: self.terms_of(resource),
            blank_ids: self.blank_ids_of(resource),
        }
    }
}

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
