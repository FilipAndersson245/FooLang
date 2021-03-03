use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::map,
    multi::many0,
    IResult,
};
use nom::{character::complete::space1, combinator::value};

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

fn is_space(s: &str) -> IResult<&str, SyntaxToken> {
    map(space1, |space: &str| SyntaxToken::Whitespace(space))(s)
}

#[allow(unused)]
fn is_fnkw(s: &str) -> IResult<&str, SyntaxToken> {
    value(SyntaxToken::LetKw("fn"), tag("fn"))(s)
}

#[allow(unused)]
fn is_letkw(s: &str) -> IResult<&str, SyntaxToken> {
    value(SyntaxToken::LetKw("let"), tag("let"))(s)
}

// TODO Handle digits at start.
#[allow(unused)]
fn is_ident(s: &str) -> IResult<&str, SyntaxToken> {
    map(take_while1(char::is_alphanumeric), |text: &str| {
        SyntaxToken::Ident(text)
    })(s)
}

fn is_math_op(s: &str) -> IResult<&str, SyntaxToken> {
    alt((
        value(SyntaxToken::Equals("="), tag("=")),
        value(SyntaxToken::Plus("+"), tag("+")),
        value(SyntaxToken::Minus("-"), tag("-")),
        value(SyntaxToken::Star("*"), tag("*")),
        value(SyntaxToken::Slash("/"), tag("/")),
    ))(s)
}

fn is_braces(s: &str) -> IResult<&str, SyntaxToken> {
    alt((
        value(SyntaxToken::LBrace("{"), tag("{")),
        value(SyntaxToken::RBrace("}"), tag("}")),
    ))(s)
}

fn is_number(s: &str) -> IResult<&str, SyntaxToken> {
    map(digit1, |text: &str| SyntaxToken::Number(text))(s)
}

pub fn tokenize(s: &str) -> Result<Vec<SyntaxToken>> {
    let (_, tokens) = many0(alt((is_space, is_number, is_math_op, is_braces)))(s)
        .map_err(|err| anyhow!("{}", err.to_string()))?;
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_space() {
        let (rem, space) = is_space(" ").unwrap();
        assert_eq!(rem, "");
        assert_eq!(space, SyntaxToken::Whitespace(" "));

        let (rem, space) = is_space("  a").unwrap();
        assert_eq!(rem, "a");
        assert_eq!(space, SyntaxToken::Whitespace("  "));
    }

    #[test]
    fn test_is_number() {
        let (rem, number) = is_number("5").unwrap();
        assert_eq!(rem, "");
        assert_eq!(number, SyntaxToken::Number("5"));

        let (rem, number) = is_number("747").unwrap();
        assert_eq!(rem, "");
        assert_eq!(number, SyntaxToken::Number("747"));

        let (rem, number) = is_number("815a").unwrap();
        assert_eq!(rem, "a");
        assert_eq!(number, SyntaxToken::Number("815"));
    }

    #[test]
    fn test_math_op() {
        let (rem, op) = is_math_op("+").unwrap();
        assert_eq!(rem, "");
        assert_eq!(op, SyntaxToken::Plus("+"));

        let (rem, op) = is_math_op("+5").unwrap();
        assert_eq!(rem, "5");
        assert_eq!(op, SyntaxToken::Plus("+"));

        let (rem, op) = is_math_op("-5").unwrap();
        assert_eq!(rem, "5");
        assert_eq!(op, SyntaxToken::Minus("-"));

        let (rem, op) = is_math_op("-").unwrap();
        assert_eq!(rem, "");
        assert_eq!(op, SyntaxToken::Minus("-"));

        let (rem, op) = is_math_op("*").unwrap();
        assert_eq!(rem, "");
        assert_eq!(op, SyntaxToken::Star("*"));

        let (rem, op) = is_math_op("*7").unwrap();
        assert_eq!(rem, "7");
        assert_eq!(op, SyntaxToken::Star("*"));

        let (rem, op) = is_math_op("/").unwrap();
        assert_eq!(rem, "");
        assert_eq!(op, SyntaxToken::Slash("/"));

        let (rem, op) = is_math_op("/4").unwrap();
        assert_eq!(rem, "4");
        assert_eq!(op, SyntaxToken::Slash("/"));

        let (rem, op) = is_math_op("=").unwrap();
        assert_eq!(rem, "");
        assert_eq!(op, SyntaxToken::Equals("="));

        let (rem, op) = is_math_op("=4").unwrap();
        assert_eq!(rem, "4");
        assert_eq!(op, SyntaxToken::Equals("="));
    }
    #[test]
    fn test_tokenize() {
        let tokens = tokenize("5+24").unwrap();
        assert_eq!(
            tokens,
            vec![
                SyntaxToken::Number("5"),
                SyntaxToken::Plus("+"),
                SyntaxToken::Number("24")
            ]
        )
    }
}
