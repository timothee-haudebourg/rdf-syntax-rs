use std::fmt;

use iref::{Iri, IriBuf};

use crate::{Id, IdRef, RdfDisplay, Term, TermRef, Triple};

/// Lexical RDF triple.
pub type LexicalTriple = Triple<Id, IriBuf, Term>;

/// Lexical RDF triple reference.
pub type LexicalTripleRef<'a> = Triple<IdRef<'a>, &'a Iri, TermRef<'a>>;

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
