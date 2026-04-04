use std::fmt;

use iref::{Iri, IriBuf};

use crate::{Id, IdRef, Quad, RdfDisplay, Term, TermRef};

/// Lexical RDF quad.
pub type RdfQuad = Quad<Id, IriBuf, Term, Id>;

/// Lexical RDF quad reference.
pub type RdfQuadRef<'a> = Quad<IdRef<'a>, &'a Iri, TermRef<'a>, IdRef<'a>>;

// impl RdfQuad {
//     pub fn as_lexical_quad_ref(&self) -> RdfQuadRef<'_> {
//         Quad(
//             self.0.as_ref(),
//             self.1.as_iri(),
//             self.2.as_ref(),
//             self.3.as_ref().map(Id::as_ref),
//         )
//     }
// }

// impl RdfQuadRef<'_> {
//     pub fn into_owned(self) -> RdfQuad {
//         Quad(
//             self.0.to_owned(),
//             self.1.to_owned(),
//             self.2.to_owned(),
//             self.3.map(IdRef::into_owned),
//         )
//     }
// }

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
