use super::{Group, Thing};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    ParseGroupError(&'a str),
    ParseGarbageError(&'a str),
}

pub type ParseResult<'a> = Result<(Thing, &'a str), ParseError<'a>>;

pub fn parse<'a>(input: &'a str) -> ParseResult<'a> {
    parse_group(input)
}

fn parse_group<'a>(mut input: &'a str) -> ParseResult<'a> {
    // confirm that we're starting a group
    if input.chars().next() != Some('{') {
        return Err(ParseError::ParseGroupError(input));
    }
    input = &input[1..];

    let mut things = Vec::new();

    loop {
        input = match input.chars().next() {
            Some(',') => &input[1..],
            Some('{') => {
                let (group, input) = parse_group(input)?;
                things.push(group);
                input
            }
            Some('<') => {
                let (garbage, input) = parse_garbage(input)?;
                things.push(garbage);
                input
            }
            Some('}') => return Ok((Thing::Group(Box::new(Group::new(&things))), &input[1..])),
            _ => return Err(ParseError::ParseGroupError(input)),
        }
    }
}

fn parse_garbage<'a>(mut input: &'a str) -> ParseResult<'a> {
    // confirm that we're starting a garbage section
    if input.chars().next() != Some('<') {
        return Err(ParseError::ParseGarbageError(input));
    }
    input = &input[1..];

    let mut garbage_chars = 0;
    let mut escaped = false;
    for (idx, ch) in input.chars().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '!' => escaped = true,
            '>' => return Ok((Thing::Garbage(garbage_chars), &input[(idx + 1)..])),
            _ => garbage_chars += 1,
        }
    }

    Err(ParseError::ParseGarbageError(&input[input.len()..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_garbage() {
        let garbage = vec![
            "<>",
            "<random characters>",
            "<<<<>",
            "<{!>}>",
            "<!!>",
            "<!!!>>",
            "<{o\"i!a,<{i<a>",
        ];
        for g in garbage {
            match parse_garbage(g) {
                Ok((Thing::Garbage(_), "")) => {}
                _ => {
                    panic!("Failed to completely parse '{}' as garbage", g);
                }
            }
        }
    }

    #[test]
    fn test_parse_group() {
        let groups = vec![
            "{}",
            "{{{}}}",
            "{{},{}}",
            "{{{},{},{{}}}}",
            "{<{},{},{{}}>}",
            "{<a>,<a>,<a>,<a>}",
            "{{<a>},{<a>},{<a>},{<a>}}",
            "{{<!>},{<!>},{<!>},{<a>}}",
        ];
        for g in groups {
            match parse_group(g) {
                Ok((g, remaining)) => {
                    assert!(g.ok().is_some());
                    assert_eq!(remaining, "");
                }
                e => panic!(e),
            }
        }
    }

    #[test]
    fn test_garbage_chars() {
        let cases = vec![
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ];
        for (g, e) in cases {
            assert_eq!(
                parse_garbage(g)
                    .expect("failed to parse garbage")
                    .0
                    .garbage_chars(),
                e
            );
        }
    }
}
