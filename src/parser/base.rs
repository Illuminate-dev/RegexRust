use super::types::*;

pub fn match_literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(x) if x == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

pub fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        p1.parse(input)
            .and_then(|(input2, out1)| p2.parse(input2).map(|(rest, out2)| (rest, (out1, out2))))
    }
}

pub fn or<'a, P1, P2, R>(parser1: P1, parser2: P2) -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| parser1.parse(input).or_else(|_| parser2.parse(input))
}

pub fn zero_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    range(parser, 0..)
}

pub fn one_or_more<'a, P, R>(parser: P) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
{
    range(parser, 1..)
}

pub fn range<'a, P, R, Range>(parser: P, r: Range) -> impl Parser<'a, Vec<R>>
where
    P: Parser<'a, R>,
    Range: std::ops::RangeBounds<usize>,
{
    move |mut input| {
        let mut out = Vec::new();
        while let Ok((next_input, value)) = parser.parse(input) {
            input = next_input;
            out.push(value);
        }
        if r.contains(&out.len()) {
            Ok((input, out))
        } else {
            Err(input)
        }
    }
}

pub fn any_char(input: &str) -> ParserResult<char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        None => Err(input),
    }
}

pub fn pred<'a, P, F, Output>(parser: P, predicate: F) -> impl Parser<'a, Output>
where
    P: Parser<'a, Output>,
    F: Fn(&Output) -> bool,
{
    move |input| {
        parser.parse(input).and_then(|(next_input, value)| {
            if predicate(&value) {
                Ok((next_input, value))
            } else {
                Err(input)
            }
        })
    }
}

pub fn optional<'a, P, R>(parser: P) -> impl Parser<'a, Option<R>>
where
    P: Parser<'a, R>,
{
    move |input| match parser.parse(input) {
        Ok((next_input, value)) => Ok((next_input, Some(value))),
        Err(_err) => Ok((input, None)),
    }
}

pub fn map<'a, P, R, F, R2>(parser: P, f: F) -> impl Parser<'a, R2>
where
    P: Parser<'a, R>,
    F: Fn(R) -> R2
{
    move |input|
        parser.parse(input)
            .map(|(next_input, value)| (next_input, f(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_literal() {
        let parser = match_literal("Hola");
        assert_eq!(parser.parse("Hola"), Ok(("", ())));
        assert_eq!(parser.parse("Hello"), Err("Hello"));
        assert_eq!(parser.parse("Hola!"), Ok(("!", ())));
    }

    #[test]
    fn test_pair() {
        let parser = pair(match_literal("Hello "), match_literal("world!"));
        assert_eq!(parser.parse("Hello world!"), Ok(("", ((), ()))));
        assert_eq!(parser.parse("Hello world! end"), Ok((" end", ((), ()))));
        assert_eq!(parser.parse("Hello world"), Err("world"));
    }

    #[test]
    fn test_or() {
        let parser = or(match_literal("hello"), match_literal("world"));

        assert_eq!(parser.parse("hello"), Ok(("", ())));
        assert_eq!(parser.parse("world"), Ok(("", ())));
        assert_eq!(parser.parse("world!"), Ok(("!", ())));
        assert_eq!(parser.parse("he"), Err("he"));
    }

    #[test]
    fn test_zero_or_more() {
        let parser = zero_or_more(match_literal("a"));
        assert_eq!(parser.parse("aaaaa"), Ok(("", vec![(), (), (), (), ()])));
        assert_eq!(parser.parse("b"), Ok(("b", vec![])));
    }

    #[test]
    fn test_one_or_more() {
        let parser = one_or_more(match_literal("a"));
        assert_eq!(parser.parse("aaaaa"), Ok(("", vec![(), (), (), (), ()])));
        assert_eq!(parser.parse("b"), Err("b"));
    }

    #[test]
    fn test_range() {
        let parser = range(match_literal("a"), 2..);
        assert_eq!(parser.parse("aaa"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("baa"), Err("baa"));
        let parser = range(match_literal("a"), 2..=3);
        assert_eq!(parser.parse("aaa"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("aaaa"), Err(""));
        let parser = range(match_literal("a"), 2..3);
        assert_eq!(parser.parse("aaa"), Err(""));
    }

    #[test]
    fn test_any_char() {
        let parser = any_char;
        assert_eq!(parser.parse("a"), Ok(("", 'a')));
        assert_eq!(parser.parse("abc"), Ok(("bc", 'a')));
        assert_eq!(parser.parse(""), Err(""));
    }

    #[test]
    fn test_pred() {
        let parser = pred(any_char, |c| *c == 'a');
        assert_eq!(parser.parse("a"), Ok(("", 'a')));
        assert_eq!(parser.parse("b"), Err("b"));
    }

    #[test]
    fn test_optional() {
        let parser = optional(match_literal("a"));
        assert_eq!(parser.parse("a"), Ok(("", Some(()))));
        assert_eq!(parser.parse("b"), Ok(("b", None)));
    }

    #[test]
    fn test_map() {
        let parser = map(match_literal("a"), |_| 1);
        assert_eq!(parser.parse("a"), Ok(("", 1)));
        assert_eq!(parser.parse("b"), Err("b"));
    }
}

