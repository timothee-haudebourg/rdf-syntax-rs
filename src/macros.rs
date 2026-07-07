#[macro_export]
#[doc(hidden)]
macro_rules! unexpected_token {
    () => {};
}

/// Creates a gRDF triple.
#[macro_export]
macro_rules! grdf_triple {
	// Parse a triple.
	{
		@from ($($acc:tt)*) $id:ident $($rest:tt)*
	} => {
		$crate::grdf_triple!(@from ($($acc)* ($crate::Term::from($id)),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) < $iri:literal > $($rest:tt)*
	} => {
		$crate::grdf_triple!(@from ($($acc)* $crate::Term::iri($crate::iri!($iri).to_owned()),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) _ : $id:literal $($rest:tt)*
	} => {
		$crate::grdf_triple!(@from ($($acc)* $crate::Term::BlankId($crate::BlankIdBuf::from_suffix($id).unwrap()),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal ^^ $ty:literal $($rest:tt)*
	} => {
		$crate::grdf_triple!(@from ($($acc)* $crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::iri!($ty).to_owned()
			)
		)),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal $($rest:tt)*
	} => {
		$crate::grdf_triple!(@from ($($acc)* $crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::XSD_STRING.to_owned()
			)
		)),) $($rest)*)
	};
	{
		@from ($($acc:tt)*)
	} => {
		<$crate::rdf_types::Triple::<$crate::Term>>::new($($acc)*)
	};
	{
		@from ($($acc:tt)*) $t:tt $($rest:tt)*
	} => {
		$crate::unexpected_token!($t)
	};
	// Main rule
	{
		$($t:tt)*
	} => {
		$crate::grdf_triple!(@from () $($t)*)
	};
}

/// Creates an array of triples.
#[macro_export]
macro_rules! grdf_triples {
	// Tokenize the triples.
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $i:ident $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* $i] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] < $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* <] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] > $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* >] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] _ $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* _] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] : $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* :] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] ^ $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* ^] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $l:literal $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)*] [$($current)* $l] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] . $($rest:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [$($acc)* ( $($current)* )] [] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] []
	} => {
		$crate::grdf_triples!(@triples_from [] $($acc)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $t:tt $($rest:tt)*
	} => {
		$crate::unexpected_token!($t)
	};
	// Parse a tokenized triple list.
	{
		@triples_from [$($acc:tt)*] ($($triple:tt)*) $($rest:tt)*
	} => {
		$crate::grdf_triples!(@triples_from [$($acc)* $crate::grdf_triple!($($triple)*),] $($rest)*)
	};
	{
		@triples_from [$($acc:tt)*]
	} => {
		[$($acc)*]
	};

	// Main rule.
	{
		$($t:tt)*
	} => {
		$crate::grdf_triples!(@tokenize [] [] $($t)*)
	};
}

/// Creates a gRDF quad.
#[macro_export]
macro_rules! grdf_quad {
	// Parse a quad.
	{
		@from ($($acc:tt)*) $id:ident $($rest:tt)*
	} => {
		$crate::grdf_quad!(@from ($($acc)* ($crate::Term::from($id)),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) < $iri:literal > $($rest:tt)*
	} => {
		$crate::grdf_quad!(@from ($($acc)* $crate::Term::iri($crate::iri!($iri).to_owned()),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) _ : $id:literal $($rest:tt)*
	} => {
		$crate::grdf_quad!(@from ($($acc)* $crate::Term::BlankId($crate::BlankIdBuf::from_suffix($id).unwrap()),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal ^^ $ty:literal $($rest:tt)*
	} => {
		$crate::grdf_quad!(@from ($($acc)* $crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::iri!($ty).to_owned()
			)
		)),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal $($rest:tt)*
	} => {
		$crate::grdf_quad!(@from ($($acc)* $crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::XSD_STRING.to_owned()
			)
		)),) $($rest)*)
	};
	{
		@from ($s:expr, $p:expr, $o:expr,)
	} => {
		<$crate::rdf_types::Quad::<$crate::Term>>::new($s, $p, $o, None)
	};
	{
		@from ($s:expr, $p:expr, $o:expr, $g:expr,)
	} => {
		<$crate::rdf_types::Quad::<$crate::Term>>::new($s, $p, $o, Some($g))
	};
	{
		@from ($($acc:tt)*) $t:tt $($rest:tt)*
	} => {
		$crate::unexpected_token!($t)
	};
	// Main rule
	{
		$($t:tt)*
	} => {
		$crate::grdf_quad!(@from () $($t)*)
	};
}

/// Creates a gRDF quad pattern.
///
/// The type of the returned value is `Quad<Pattern<Term, _>>`.
///
/// This is similar to [`grdf_quad`], but with the addition of variables, which
/// can be introduced using the syntax `?ident`. The variable `ident` must be
/// an identifier to some Rust variable holding the pattern variable's value.
#[macro_export]
macro_rules! grdf_quad_pattern {
	// Parse a quad pattern.
	{
		@from ($($acc:tt)*) ? $id:ident $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Var($id),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $id:ident $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Ground($id),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) < $iri:literal > $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Ground($crate::Term::iri($crate::iri!($iri).to_owned())),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) _ : $id:literal $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Ground($crate::Term::BlankId($crate::BlankIdBuf::from_suffix($id).unwrap())),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal ^^ $ty:literal $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Ground($crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::iri!($ty).to_owned()
			)
		))),) $($rest)*)
	};
	{
		@from ($($acc:tt)*) $value:literal $($rest:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from ($($acc)* $crate::rdf_types::pattern::Pattern::Ground($crate::Term::literal($crate::Literal::new(
			$value.to_owned(),
			$crate::LiteralType::Any(
				$crate::XSD_STRING.to_owned()
			)
		))),) $($rest)*)
	};
	{
		@from ($s:expr, $p:expr, $o:expr,)
	} => {
		<$crate::Quad<$crate::rdf_types::pattern::Pattern<$crate::Term, _>>>::new($s, $p, $o, None)
	};
	{
		@from ($s:expr, $p:expr, $o:expr, $g:expr,)
	} => {
		<$crate::Quad<$crate::rdf_types::pattern::Pattern<$crate::Term, _>>>::new($s, $p, $o, Some($g))
	};
	{
		@from ($($acc:tt)*) $t:tt $($rest:tt)*
	} => {
		$crate::unexpected_token!($t)
	};
	// Main rule
	{
		$($t:tt)*
	} => {
		$crate::grdf_quad_pattern!(@from () $($t)*)
	};
}

/// Creates an array of quads.
#[macro_export]
macro_rules! grdf_quads {
	// Tokenize the quads.
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $i:ident $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* $i] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] < $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* <] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] > $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* >] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] _ $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* _] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] : $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* :] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] ^ $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* ^] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $l:literal $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)*] [$($current)* $l] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] . $($rest:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [$($acc)* ( $($current)* )] [] $($rest)*)
	};
	{
		@tokenize [$($acc:tt)*] []
	} => {
		$crate::grdf_quads!(@quads_from [] $($acc)*)
	};
	{
		@tokenize [$($acc:tt)*] [$($current:tt)*] $t:tt $($rest:tt)*
	} => {
		$crate::unexpected_token!($t)
	};
	// Parse a tokenized triple list.
	{
		@quads_from [$($acc:tt)*] ($($triple:tt)*) $($rest:tt)*
	} => {
		$crate::grdf_quads!(@quads_from [$($acc)* $crate::grdf_quad!($($triple)*),] $($rest)*)
	};
	{
		@quads_from [$($acc:tt)*]
	} => {
		[$($acc)*]
	};

	// Main rule.
	{
		$($t:tt)*
	} => {
		$crate::grdf_quads!(@tokenize [] [] $($t)*)
	};
}

#[cfg(test)]
mod tests {
    use crate::rdf_types::pattern::Pattern;
    use iref::iri;

    #[test]
    fn grdf_triple_macro() {
        let _ = grdf_triple! [
            _:"foo" <"https://example.org/#foo"> "foo"
        ];
    }

    #[test]
    fn grdf_triples_macro() {
        let term = iri!("https://example.org/#iri").to_owned();
        let _ = grdf_triples! [
            _:"foo" <"https://example.org/#foo"> "foo" .
            <"https://example.org/#bar"> _:"bar" "bar"^^"https://example.org/#datatype" .
            <"https://example.org/#baz"> term "value" .
        ];
    }

    #[test]
    fn grdf_quad_macro() {
        let _ = grdf_quad! [
            _:"foo" <"https://example.org/#foo"> "foo"
        ];
    }

    #[test]
    fn grdf_quads_macro() {
        let term = iri!("https://example.org/#iri").to_owned();
        let _ = grdf_quads! [
            _:"foo" <"https://example.org/#foo"> "foo" .
            <"https://example.org/#bar"> _:"bar" "bar"^^"https://example.org/#datatype" .
            <"https://example.org/#baz"> term "value" <"https://example.org/#graph"> .
        ];
    }

    #[test]
    fn grdf_quad_pattern_macro_var() {
        let s = 1;
        let g = 2;
        let pattern = grdf_quad_pattern! [
            ?s <"https://example.org/#foo"> "foo"^^"https://example.org/#datatype" ?g
        ];

        assert!(matches!(pattern.0, Pattern::Var(1)));
        assert!(matches!(pattern.1, Pattern::Ground(_)));
        assert!(matches!(pattern.2, Pattern::Ground(_)));
        assert!(matches!(pattern.3, Some(Pattern::Var(2))));
    }

    #[test]
    fn grdf_quad_pattern_macro_ground() {
        let term = crate::Term::iri(iri!("https://example.org/#iri").to_owned());
        let pattern: crate::Quad<Pattern<crate::Term, i32>> = grdf_quad_pattern! [
            _:"foo" term "value"
        ];

        assert!(matches!(
            pattern.0,
            Pattern::Ground(crate::Term::BlankId(_))
        ));
        assert!(matches!(pattern.1, Pattern::Ground(_)));
        assert!(matches!(pattern.2, Pattern::Ground(_)));
        assert!(pattern.3.is_none());
    }
}
