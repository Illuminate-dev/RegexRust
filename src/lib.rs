mod parser;
use parser::{any_char, map, match_literal, one_or_more, pred, Parser, ParserResult};

fn literal_pattern<'a>(input: &'a str) -> ParserResult<'a, impl Parser<'a, char>> {
    let parser = pred(any_char, |c| c.is_alphanumeric());

    match parser.parse(input) {
        Ok((input, _)) => {
            let parser = parser;
            Ok((input, parser))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {}

    #[test]
    fn test_literal() {
        let parser = literal_pattern("hello");

        let parser = match parser {
            Ok((_, parser)) => parser,
            Err(_) => panic!("Failed to parse literal"),
        };

        let result = parser.parse("hello world");

        assert_eq!(result, Ok(("ello world", 'h')));
    }
}
