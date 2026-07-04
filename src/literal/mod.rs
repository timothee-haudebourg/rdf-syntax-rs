use crate::RdfDisplay;
use langtag::LangTag;
use std::fmt;

mod cow;
mod r#ref;
mod r#type;

pub use cow::*;
pub use r#ref::*;
pub use r#type::*;

/// RDF Literal.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal {
    /// Literal value.
    pub value: String,

    /// Literal type.
    pub r#type: LiteralType,
}

impl Literal {
    pub fn new(value: impl Into<String>, type_: impl Into<LiteralType>) -> Self {
        Self {
            value: value.into(),
            r#type: type_.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    pub fn is_lang_string(&self) -> bool {
        self.r#type.is_lang_string()
    }

    pub fn lang_tag(&self) -> Option<&LangTag> {
        self.r#type.lang_tag()
    }

    pub fn as_ref(&self) -> LiteralRef<'_> {
        LiteralRef::new(&self.value, self.r#type.as_ref())
    }

    pub fn as_cow(&self) -> CowLiteral<'_> {
        CowLiteral::new(&self.value, self.r#type.as_cow())
    }

    pub fn into_cow(self) -> CowLiteral<'static> {
        CowLiteral::new(self.value, self.r#type.into_cow())
    }
}

impl AsRef<str> for Literal {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Literal {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.rdf_fmt(f)?;
        if self.r#type.is_xsd_string() {
            Ok(())
        } else {
            self.r#type.rdf_fmt(f)
        }
    }
}

impl RdfDisplay for Literal {
    fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.rdf_fmt(f)?;
        if self.r#type.is_xsd_string() {
            Ok(())
        } else {
            self.r#type.rdf_fmt(f)
        }
    }
}
