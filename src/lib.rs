// use num::FromPrimitive;
// use num_derive::{FromPrimitive, ToPrimitive};
// use rowan::{GreenNodeBuilder, Language, NodeOrToken};
// use std::iter::Peekable;

// #[repr(u16)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
// pub enum SyntaxKind {
//     Whitespace = 0,
//     FnKw,
//     LetKw,
//     Ident,
//     Number,
//     Plus,
//     Minus,
//     Star,
//     Slash,
//     Equals,
//     LBrace,
//     RBrace,
//     Root,
// }

// impl From<SyntaxKind> for rowan::SyntaxKind {
//     fn from(kind: SyntaxKind) -> Self {
//         Self(kind as u16)
//     }
// }
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum BarLang {}

// impl rowan::Language for BarLang {
//     type Kind = SyntaxKind;
//     fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
//         debug_assert!(raw.0 <= SyntaxKind::Root as u16);
//         FromPrimitive::from_u16(raw.0).unwrap()
//     }

//     fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
//         kind.into()
//     }
// }

// type SyntaxNode = rowan::SyntaxNode<BarLang>;
// #[allow(unused)]
// type SyntaxToken = rowan::SyntaxToken<BarLang>;
// #[allow(unused)]
// type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

// struct Parser<'a, I: Iterator<Item = (SyntaxKind, &'a str)>> {
//     builder: GreenNodeBuilder<'static>,
//     iter: Peekable<I>,
// }

// impl<'a, I: Iterator<Item = (SyntaxKind, &'a str)>> Parser<'a, I> {
//     fn peek(&mut self) -> Option<SyntaxKind> {
//         while self
//             .iter
//             .peek()
//             .map(|&(t, _)| t == SyntaxKind::Whitespace)
//             .unwrap_or(false)
//         {
//             self.bump();
//         }
//         self.iter.peek().map(|&(t, _)| t)
//     }

//     fn bump(&mut self) {
//         if let Some((token, string)) = self.iter.next() {
//             self.builder.token(token.into(), string);
//         }
//     }

//     fn parse(mut self) -> SyntaxNode {
//         self.builder.start_node(SyntaxKind::Root.into());

//         for (syntax_kind, s) in self.iter {
//             self.builder.token(BarLang::kind_to_raw(syntax_kind), s);
//         }
//         self.builder.finish_node();
//         SyntaxNode::new_root(self.builder.finish())
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         use super::*;
//         let ast = Parser {
//             builder: GreenNodeBuilder::new(),
//             iter: vec![
//                 // 1 + 2 * 3 + 4
//                 (SyntaxKind::Number, "5".into()),
//                 (SyntaxKind::Plus, "+".into()),
//                 (SyntaxKind::Number, "2".into()),
//             ]
//             .into_iter()
//             .peekable(),
//         }
//         .parse();
//         println!("{:#?}", ast);
//     }
// }

// // use std::mem::transmute
