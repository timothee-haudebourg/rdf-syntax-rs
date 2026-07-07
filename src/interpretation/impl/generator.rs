use std::{borrow::Cow, cell::RefCell};

use iref::Iri;
use rdf_types::{ConstGenDomain, Domain};

use crate::{
	BlankId, CowLiteral, Generator, GroundInterpretation, GroundInterpretationMut, LiteralRef,
	Term,
	interpretation::{
		Interpretation, InterpretationMut, ReverseGroundInterpretation, ReverseInterpretation,
	},
};

/// Lexical interpretation based on a term generator.
pub struct GenInterpretation<G>(RefCell<G>);

impl<G> GenInterpretation<G> {
	pub fn new(generator: G) -> Self {
		Self(RefCell::new(generator))
	}

	pub fn into_generator(self) -> G {
		self.0.into_inner()
	}
}

impl<G> Domain for GenInterpretation<G> {
	type Resource = Term;
}

impl<G> GroundInterpretation for GenInterpretation<G> {
	fn iri(&self, iri: &Iri) -> Option<Self::Resource> {
		Some(iri.to_owned().into())
	}

	fn literal<'a>(&self, literal: impl Into<LiteralRef<'a>>) -> Option<Self::Resource> {
		Some(literal.into().to_owned().into())
	}
}

impl<G> Interpretation for GenInterpretation<G> {
	fn blank_id(&self, blank_id: &BlankId) -> Option<Self::Resource> {
		Some(blank_id.to_owned().into())
	}
}

impl<G> GroundInterpretationMut for GenInterpretation<G> {
	fn insert_iri<'a>(&mut self, iri: impl Into<Cow<'a, Iri>>) -> Self::Resource {
		iri.into().into_owned().into()
	}

	fn insert_literal<'a>(&mut self, literal: impl Into<CowLiteral<'a>>) -> Self::Resource {
		literal.into().into_owned().into()
	}
}

impl<G> InterpretationMut for GenInterpretation<G> {
	fn insert_blank_id<'a>(&mut self, blank_id: impl Into<Cow<'a, BlankId>>) -> Self::Resource {
		blank_id.into().into_owned().into()
	}
}

impl<G> ReverseGroundInterpretation for GenInterpretation<G> {
	type Iris<'a>
		= std::option::IntoIter<&'a Iri>
	where
		Self: 'a;
	type Literals<'a>
		= std::option::IntoIter<LiteralRef<'a>>
	where
		Self: 'a;

	fn iris_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Iris<'a> {
		resource.as_iri().into_iter()
	}

	fn literals_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::Literals<'a> {
		resource.as_literal().into_iter()
	}
}

impl<G> ReverseInterpretation for GenInterpretation<G> {
	type BlankIds<'a>
		= std::option::IntoIter<&'a BlankId>
	where
		Self: 'a;

	fn blank_ids_of<'a>(&'a self, resource: &'a Self::Resource) -> Self::BlankIds<'a> {
		resource.as_blank_id().into_iter()
	}
}

impl<G: Generator> ConstGenDomain for GenInterpretation<G> {
	fn new_resource(&self) -> Self::Resource {
		let mut generator = self.0.borrow_mut();
		generator.next_id().into()
	}
}
