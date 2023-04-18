mod parser;
use parser::{any_char, map, match_literal, one_or_more, pred, Parser, ParserResult};

fn literal_pattern<'a, P>(input: &'a str) -> ParserResult<'a, P>
where
    P: Parser<'a, String>,
{
    let mut out = String::new();
    let mut chars = input.chars();

    while let Some(char) = chars.next() {
        match char {
            x if x.is_alphanumeric() => out.push(x),
            _ => break,
        }
    }

    if out.len() == 0 {
        return Err(input);
    }

    Ok((&input[out.len()..], map(match_literal(&out), |_| out)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {}

    #[test]
    fn test_literal() {}
}
