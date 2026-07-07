use std::borrow::Cow;

use iref::Iri;
use rdf_types::StaticDomain;

use crate::{
	CowLiteral, GroundInterpretationMut, GroundTerm, LiteralRef, Term,
	interpretation::{GroundInterpretation, ReverseGroundInterpretation},
};

impl GroundInterpretation for StaticDomain<Term> {
	fn iri(&self, iri: &Iri) -> Option<Term> {
		Some(Term::iri(iri.to_owned()))
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Term> {
		Some(Term::literal(literal.into().to_owned()))
	}
}

impl GroundInterpretationMut for StaticDomain<Term> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		Term::iri(iri.into().into_owned())
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		Term::literal(literal.into().into_owned())
	}
}

impl ReverseGroundInterpretation for StaticDomain<Term> {
	type Iris<'a> = std::option::IntoIter<&'a Iri>;
	type Literals<'a> = std::option::IntoIter<LiteralRef<'a>>;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		match resource {
			Term::Ground(GroundTerm::Iri(iri)) => Some(iri.as_iri()).into_iter(),
			_ => None.into_iter(),
		}
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		match resource {
			Term::Ground(GroundTerm::Literal(l)) => Some(l.as_ref()).into_iter(),
			_ => None.into_iter(),
		}
	}
}
