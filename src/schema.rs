use iref::{Iri, iri};

/// <http://www.w3.org/2000/01/rdf-schema#Resource> class IRI.
pub const RDFS_RESOURCE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Resource");
/// <http://www.w3.org/2000/01/rdf-schema#Class> class IRI.
pub const RDFS_CLASS: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Class");
/// <http://www.w3.org/2000/01/rdf-schema#Literal> class IRI.
pub const RDFS_LITERAL: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Literal");
/// <http://www.w3.org/2000/01/rdf-schema#Datatype> class IRI.
pub const RDFS_DATATYPE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Datatype");
/// <http://www.w3.org/2000/01/rdf-schema#Container> class IRI.
pub const RDFS_CONTAINER: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Container");
/// <http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty> class IRI.
pub const RDFS_CONTAINER_MEMBERSHIP_PROPERTY: &Iri =
	iri!("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty");

/// <http://www.w3.org/2000/01/rdf-schema#range> property IRI.
pub const RDFS_RANGE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#range");
/// <http://www.w3.org/2000/01/rdf-schema#domain> property IRI.
pub const RDFS_DOMAIN: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#domain");
/// <http://www.w3.org/2000/01/rdf-schema#subClassOf> property IRI.
pub const RDFS_SUB_CLASS_OF: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#subClassOf");
/// <http://www.w3.org/2000/01/rdf-schema#subPropertyOf> property IRI.
pub const RDFS_SUB_PROPERTY_OF: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#subPropertyOf");
/// <http://www.w3.org/2000/01/rdf-schema#label> property IRI.
pub const RDFS_LABEL: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#label");
/// <http://www.w3.org/2000/01/rdf-schema#comment> property IRI.
pub const RDFS_COMMENT: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#comment");
/// <http://www.w3.org/2000/01/rdf-schema#member> property IRI.
pub const RDFS_MEMBER: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#member");
/// <http://www.w3.org/2000/01/rdf-schema#seeAlso> property IRI.
pub const RDFS_SEE_ALSO: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#seeAlso");
/// <http://www.w3.org/2000/01/rdf-schema#isDefinedBy> property IRI.
pub const RDFS_IS_DEFINED_BY: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#isDefinedBy");

/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#langString> datatype IRI.
pub const RDF_LANG_STRING: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML> datatype IRI.
pub const RDF_HTML: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral> datatype IRI.
pub const RDF_XML_LITERAL: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON> datatype IRI.
pub const RDF_JSON: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#Property> class IRI.
pub const RDF_PROPERTY: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag> class IRI.
pub const RDF_BAG: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq> class IRI.
pub const RDF_SEQ: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt> class IRI.
pub const RDF_ALT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#List> class IRI.
pub const RDF_LIST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#List");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement> class IRI.
pub const RDF_STATEMENT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement");

/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> property IRI.
pub const RDF_TYPE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#first> property IRI.
pub const RDF_FIRST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#rest> property IRI.
pub const RDF_REST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#subject> property IRI.
pub const RDF_SUBJECT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate> property IRI.
pub const RDF_PREDICATE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#object> property IRI.
pub const RDF_OBJECT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#object");
/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#value> property IRI.
pub const RDF_VALUE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#value");

/// <http://www.w3.org/1999/02/22-rdf-syntax-ns#nil> individual IRI.
pub const RDF_NIL: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");

/// <http://www.w3.org/2001/XMLSchema#string> datatype IRI.
pub const XSD_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#string");
