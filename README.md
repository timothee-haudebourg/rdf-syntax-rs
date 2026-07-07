# RDF Syntax Data Types & Traits

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/timothee-haudebourg/rdf-syntax-rs/ci.yml?style=flat-square&logo=github)](https://github.com/timothee-haudebourg/rdf-syntax-rs/actions)
[![Crate informations](https://img.shields.io/crates/v/rdf-syntax.svg?style=flat-square)](https://crates.io/crates/rdf-syntax)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/rdf-syntax?style=flat-square)](https://crates.io/crates/rdf-syntax)
[![License](https://img.shields.io/crates/l/rdf-syntax.svg?style=flat-square)](https://github.com/timothee-haudebourg/rdf-syntax-rs#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/rdf-syntax)

<!-- cargo-rdme start -->

The [Resource Description Framework (RDF)][rdf] is a very simple graph data
model defined by the [World Wide Web Consortium (W3C)][w3c] to represent
arbitrary pieces of information, primarily intended for the web. Nodes of
the graph are called *resources*, and resources are connected together using
*relations*, which are resources themselves.

This crate is built on top of [`rdf-types`](https://crates.io/crates/rdf-types)
to provide the lexical (syntactic) representation of resources (IRIs,
blank node identifiers and literals) as defined by the
[RDF 1.1 Concepts and Abstract Syntax][rdf-concepts] recommendation,
along with the term definitions from [RDF Schema (RDFS)][rdfs]:
- [`Id`](https://docs.rs/rdf-syntax/latest/rdf_syntax/id/enum.Id.html) and [`BlankId`](https://docs.rs/rdf-syntax/latest/rdf_syntax/blankid/struct.BlankId.html) represent, respectively, a resource identifier
  (an IRI or a blank node identifier) and a blank node identifier alone;
- [`Literal`](https://docs.rs/rdf-syntax/latest/rdf_syntax/literal/struct.Literal.html) and [`LiteralType`](https://docs.rs/rdf-syntax/latest/rdf_syntax/literal/type/enum.LiteralType.html) represent RDF literals and their
  datatype (either an arbitrary IRI or the `rdf:langString` type paired
  with a [`LangTag`](https://docs.rs/langtag/latest/langtag/struct.LangTag.html));
- [`Term`](https://docs.rs/rdf-syntax/latest/rdf_syntax/term/enum.Term.html) and [`GroundTerm`](https://docs.rs/rdf-syntax/latest/rdf_syntax/term/ground/enum.GroundTerm.html) represent, respectively, a lexical RDF term
  (an [`Id`](https://docs.rs/rdf-syntax/latest/rdf_syntax/id/enum.Id.html) or a [`Literal`](https://docs.rs/rdf-syntax/latest/rdf_syntax/literal/struct.Literal.html)) and a *ground* term, which excludes blank
  node identifiers (an IRI or a [`Literal`](https://docs.rs/rdf-syntax/latest/rdf_syntax/literal/struct.Literal.html));
- [`RdfTriple`](https://docs.rs/rdf-syntax/latest/rdf_syntax/triple/type.RdfTriple.html) and [`RdfQuad`](https://docs.rs/rdf-syntax/latest/rdf_syntax/quad/type.RdfQuad.html) are triples/quads built from those lexical
  terms, with `Grdf`-prefixed variants ([`GrdfTriple`](https://docs.rs/rdf-syntax/latest/rdf_syntax/triple/type.GrdfTriple.html), [`GrdfQuad`](https://docs.rs/rdf-syntax/latest/rdf_syntax/quad/type.GrdfQuad.html))
  allowing a blank node identifier anywhere in the triple/quad, for
  generalized RDF;
- the [`Interpretation`](https://docs.rs/rdf-syntax/latest/rdf_syntax/interpretation/trait.Interpretation.html) trait family maps lexical terms to the semantic
  resources of an [`rdf_types::Domain`](https://docs.rs/rdf_types/latest/rdf_types/domain/trait.Domain.html), turning lexical RDF data into the
  purely semantic representation manipulated by the `rdf-types` crate.

Every lexical type above comes in three flavors: an owned version (e.g.
[`Term`](https://docs.rs/rdf-syntax/latest/rdf_syntax/term/enum.Term.html)), a borrowed version (e.g. [`TermRef`](https://docs.rs/rdf-syntax/latest/rdf_syntax/term/ref/enum.TermRef.html)), and a copy-on-write
version (e.g. [`CowTerm`](https://docs.rs/rdf-syntax/latest/rdf_syntax/term/cow/enum.CowTerm.html)).

[rdf]: <https://w3c.github.io/rdf-primer/spec/>
[w3c]: <https://www.w3.org/>
[rdf-concepts]: <https://www.w3.org/TR/rdf11-concepts/>
[rdfs]: <https://www.w3.org/TR/rdf-schema/>

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting any contribution.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
