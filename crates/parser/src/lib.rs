// use num::FromPrimitive;
// use num_derive::{FromPrimitive, ToPrimitive};
use rowan::{GreenNodeBuilder, Language};
use std::iter::Peekable;

use types::{FooLang, SyntaxKind, SyntaxNode};
struct Parser<'a, I: Iterator<Item = (SyntaxKind, &'a str)>> {
    builder: GreenNodeBuilder<'static>,
    iter: Peekable<I>,
}

impl<'a, I: Iterator<Item = (SyntaxKind, &'a str)>> Parser<'a, I> {
    fn peek(&mut self) -> Option<SyntaxKind> {
        while self
            .iter
            .peek()
            .map(|&(t, _)| t == SyntaxKind::Whitespace)
            .unwrap_or(false)
        {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }

    fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string);
        }
    }

    pub fn parse(mut self) -> SyntaxNode {
        self.builder.start_node(SyntaxKind::Root.into());

        for (syntax_kind, s) in self.iter {
            self.builder.token(FooLang::kind_to_raw(syntax_kind), s);
        }
        self.builder.finish_node();
        SyntaxNode::new_root(self.builder.finish())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let ast = Parser {
            builder: GreenNodeBuilder::new(),
            iter: vec![
                // 1 + 2 * 3 + 4
                (SyntaxKind::Number, "5".into()),
                (SyntaxKind::Plus, "+".into()),
                (SyntaxKind::Number, "2".into()),
            ]
            .into_iter()
            .peekable(),
        }
        .parse();
        println!("{:#?}", ast);
    }
}
