//! The [Resource Description Framework (RDF)][rdf] is a very simple graph data
//! model defined by the [World Wide Web Consortium (W3C)][w3c] to represent
//! arbitrary pieces of information, primarily intended for the web. Nodes of
//! the graph are called *resources*, and resources are connected together using
//! *relations*, which are resources themselves.
//!
//! This crate is built on top of [`rdf-types`](https://crates.io/crates/rdf-types)
//! to provide the lexical (syntactic) representation of resources (IRIs,
//! blank node identifiers and literals) as defined by the
//! [RDF 1.1 Concepts and Abstract Syntax][rdf-concepts] recommendation,
//! along with the term definitions from [RDF Schema (RDFS)][rdfs]:
//! - [`Id`] and [`BlankId`] represent, respectively, a resource identifier
//!   (an IRI or a blank node identifier) and a blank node identifier alone;
//! - [`Literal`] and [`LiteralType`] represent RDF literals and their
//!   datatype (either an arbitrary IRI or the `rdf:langString` type paired
//!   with a [`LangTag`]);
//! - [`Term`] and [`GroundTerm`] represent, respectively, a lexical RDF term
//!   (an [`Id`] or a [`Literal`]) and a *ground* term, which excludes blank
//!   node identifiers (an IRI or a [`Literal`]);
//! - [`RdfTriple`] and [`RdfQuad`] are triples/quads built from those lexical
//!   terms, with `Grdf`-prefixed variants ([`GrdfTriple`], [`GrdfQuad`])
//!   allowing a blank node identifier anywhere in the triple/quad, for
//!   generalized RDF;
//! - the [`Interpretation`] trait family maps lexical terms to the semantic
//!   resources of an [`rdf_types::Domain`], turning lexical RDF data into the
//!   purely semantic representation manipulated by the `rdf-types` crate.
//!
//! Every lexical type above comes in three flavors: an owned version (e.g.
//! [`Term`]), a borrowed version (e.g. [`TermRef`]), and a copy-on-write
//! version (e.g. [`CowTerm`]).
//!
//! [rdf]: <https://w3c.github.io/rdf-primer/spec/>
//! [w3c]: <https://www.w3.org/>
//! [rdf-concepts]: <https://www.w3.org/TR/rdf11-concepts/>
//! [rdfs]: <https://www.w3.org/TR/rdf-schema/>

/// IRI types re-export.
pub use iref;
pub use iref::{InvalidIri, Iri, IriBuf, IriError, IriRef, IriRefBuf, iri, iri_ref};

/// RDF types re-export.
pub use rdf_types;
pub use rdf_types::*;

/// Language tag types re-export.
pub use langtag;
pub use langtag::{LangTag, LangTagBuf};

mod blankid;
mod display;
mod id;
mod interpretation;
mod literal;
mod macros;
mod quad;
mod schema;
mod term;
mod triple;

pub use blankid::*;
pub use display::*;
pub use id::*;
pub use interpretation::*;
pub use literal::*;
pub use quad::*;
pub use schema::*;
pub use term::*;
pub use triple::*;
