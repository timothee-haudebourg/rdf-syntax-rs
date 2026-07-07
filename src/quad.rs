use std::fmt;

use iref::{Iri, IriBuf};

use crate::{Id, IdRef, Quad, RdfDisplay, Term, TermRef};

/// Lexical RDF quad.
pub type RdfQuad = Quad<Id, IriBuf, Term, Id>;

/// Lexical RDF quad reference.
pub type RdfQuadRef<'a> = Quad<IdRef<'a>, &'a Iri, TermRef<'a>, IdRef<'a>>;

pub type GrdfQuad = Quad<Term>;

pub type GrdfQuadRef<'a> = Quad<TermRef<'a>>;

impl<S: RdfDisplay, P: RdfDisplay, O: RdfDisplay, G: RdfDisplay> RdfDisplay for Quad<S, P, O, G> {
    fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.graph() {
            Some(graph) => write!(
                f,
                "{} {} {} {}",
                self.0.rdf_display(),
                self.1.rdf_display(),
                self.2.rdf_display(),
                graph.rdf_display()
            ),
            None => write!(
                f,
                "{} {} {}",
                self.0.rdf_display(),
                self.1.rdf_display(),
                self.2.rdf_display()
            ),
        }
    }
}
