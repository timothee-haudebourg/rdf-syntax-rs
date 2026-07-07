use std::fmt;

use iref::{Iri, IriBuf};

use crate::{Id, IdRef, RdfDisplay, Term, TermRef, Triple};

/// RDF triple.
pub type RdfTriple = Triple<Id, IriBuf, Term>;

/// RDF triple reference.
pub type RdfTripleRef<'a> = Triple<IdRef<'a>, &'a Iri, TermRef<'a>>;

/// gRDF triple.
pub type GrdfTriple = Triple<Term>;

/// gRDF triple reference.
pub type GrdfTripleRef<'a> = Triple<TermRef<'a>>;

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay> RdfDisplay for Triple<S, P, O> {
	fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{} {} {}",
			self.0.rdf_display(),
			self.1.rdf_display(),
			self.2.rdf_display()
		)
	}
}
