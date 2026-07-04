use core::fmt;
use std::{borrow::Borrow, cmp::Ordering};

use into_owned_trait::IntoOwned;
use langtag::LangTag;

use crate::RdfDisplay;

use super::{CowLiteral, Literal, LiteralTypeRef};

/// RDF Literal reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LiteralRef<'a> {
    /// Literal value.
    pub value: &'a str,

    /// Literal type.
    pub r#type: LiteralTypeRef<'a>,
}

impl<'a> LiteralRef<'a> {
    pub fn new(value: &'a str, type_: impl Into<LiteralTypeRef<'a>>) -> Self {
        Self {
            value,
            r#type: type_.into(),
        }
    }

    pub fn as_type(&self) -> LiteralTypeRef<'a> {
        self.r#type
    }

    pub fn as_type_mut(&mut self) -> &mut LiteralTypeRef<'a> {
        &mut self.r#type
    }

    pub fn into_type(self) -> LiteralTypeRef<'a> {
        self.r#type
    }

    pub fn as_value(&self) -> &'a str {
        self.value
    }

    pub fn into_value(self) -> &'a str {
        self.value
    }

    pub fn into_parts(self) -> (&'a str, LiteralTypeRef<'a>) {
        (self.value, self.r#type)
    }

    pub fn as_str(&self) -> &'a str {
        self.value
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.value.as_ref()
    }

    pub fn is_lang_string(&self) -> bool {
        self.r#type.is_lang_string()
    }

    pub fn lang_tag(&self) -> Option<&'a LangTag> {
        self.r#type.lang_tag()
    }

    pub fn into_cow(self) -> CowLiteral<'a> {
        CowLiteral::new(self.value, self.r#type)
    }
}

impl LiteralRef<'_> {
    pub fn to_owned(self) -> Literal {
        Literal::new(self.value.to_owned(), self.r#type.into_owned())
    }

    pub fn cloned(self) -> Literal {
        self.to_owned()
    }
}

impl<'a> IntoOwned for LiteralRef<'a> {
    type Owned = Literal;

    fn into_owned(self) -> Self::Owned {
        Literal::new(self.value, self.r#type.into_owned())
    }
}

impl From<LiteralRef<'_>> for Literal {
    fn from(value: LiteralRef<'_>) -> Self {
        value.to_owned()
    }
}

impl<'a> PartialEq<LiteralRef<'a>> for Literal {
    fn eq(&self, other: &LiteralRef<'a>) -> bool {
        self.r#type == other.r#type && self.value == other.value
    }
}

impl PartialEq<Literal> for LiteralRef<'_> {
    fn eq(&self, other: &Literal) -> bool {
        self.r#type == other.r#type && self.value == other.value
    }
}

impl equivalent::Equivalent<Literal> for LiteralRef<'_> {
    fn equivalent(&self, key: &Literal) -> bool {
        self == key
    }
}

impl<'a> PartialOrd<LiteralRef<'a>> for Literal {
    fn partial_cmp(&self, other: &LiteralRef<'a>) -> Option<Ordering> {
        Some(
            self.value
                .as_str()
                .partial_cmp(other.value)?
                .then(self.r#type.partial_cmp(&other.r#type)?),
        )
    }
}

impl PartialOrd<Literal> for LiteralRef<'_> {
    fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
        Some(
            self.value
                .partial_cmp(other.value.as_str())?
                .then(self.r#type.partial_cmp(&other.r#type)?),
        )
    }
}

impl Borrow<str> for LiteralRef<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for LiteralRef<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LiteralRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.rdf_fmt(f)?;
        if self.r#type.is_xsd_string() {
            Ok(())
        } else {
            self.r#type.rdf_fmt(f)
        }
    }
}

impl RdfDisplay for LiteralRef<'_> {
    fn rdf_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.rdf_fmt(f)?;
        if self.r#type.is_xsd_string() {
            Ok(())
        } else {
            self.r#type.rdf_fmt(f)
        }
    }
}
