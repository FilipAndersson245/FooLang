use num::FromPrimitive;
// use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use std::convert::From;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum SyntaxToken<'a> {
    Whitespace(&'a str),
    FnKw(&'a str),
    LetKw(&'a str),
    Ident(&'a str),
    Number(&'a str),
    Plus(&'a str),
    Minus(&'a str),
    Star(&'a str),
    Slash(&'a str),
    Equals(&'a str),
    LBrace(&'a str),
    RBrace(&'a str),
    Root(&'a str),
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace = 0,
    FnKw,
    LetKw,
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LBrace,
    RBrace,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FooLang {}

impl rowan::Language for FooLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        debug_assert!(
            raw.0 <= SyntaxKind::Root as u16,
            "Failed to convert from raw to `SyntaxKind`"
        );
        FromPrimitive::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<FooLang>;

#[allow(unused)]
type SyntaxElement<'a> = rowan::NodeOrToken<SyntaxNode, SyntaxToken<'a>>;

impl From<SyntaxToken<'_>> for SyntaxKind {
    fn from(token: SyntaxToken) -> Self {
        match token {
            SyntaxToken::Whitespace(_) => SyntaxKind::Whitespace,
            SyntaxToken::FnKw(_) => SyntaxKind::FnKw,
            SyntaxToken::LetKw(_) => SyntaxKind::LetKw,
            SyntaxToken::Ident(_) => SyntaxKind::Ident,
            SyntaxToken::Number(_) => SyntaxKind::Number,
            SyntaxToken::Plus(_) => SyntaxKind::Plus,
            SyntaxToken::Minus(_) => SyntaxKind::Minus,
            SyntaxToken::Star(_) => SyntaxKind::Star,
            SyntaxToken::Slash(_) => SyntaxKind::Slash,
            SyntaxToken::Equals(_) => SyntaxKind::Equals,
            SyntaxToken::LBrace(_) => SyntaxKind::LBrace,
            SyntaxToken::RBrace(_) => SyntaxKind::RBrace,
            SyntaxToken::Root(_) => SyntaxKind::Root,
        }
    }
}
