pub mod parser;
pub use parser::parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Thing {
    Group(Box<Group>),
    Garbage(usize),
}

impl Thing {
    pub fn ok<'a>(&'a self) -> Option<&'a Box<Group>> {
        use Thing::*;
        match *self {
            Group(ref box_group) => Some(box_group),
            Garbage(_) => None,
        }
    }

    pub fn garbage_chars(&self) -> usize {
        use Thing::*;
        match *self {
            Garbage(g) => g,
            Group(ref group) => group.garbage_chars(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    things: Vec<Thing>,
}

impl Group {
    pub fn new<S: AsRef<[Thing]>>(things: &S) -> Group {
        let things = things.as_ref();
        let mut things_vec = Vec::with_capacity(things.len());
        things_vec.extend(things.iter().cloned());
        Group { things: things_vec }
    }

    pub fn score(&self) -> usize {
        self.recursive_score(0)
    }

    fn recursive_score(&self, outer_score: usize) -> usize {
        let score = 1 + outer_score;
        score +
            self.things
                .iter()
                .filter_map(|thing| thing.ok())
                .map(|box_group| box_group.recursive_score(score))
                .sum::<usize>()
    }

    pub fn count(&self) -> usize {
        1 +
            self.things
                .iter()
                .filter_map(|thing| thing.ok())
                .map(|box_group| box_group.count())
                .sum::<usize>()
    }

    pub fn garbage_chars(&self) -> usize {
        self.things.iter().map(|thing| thing.garbage_chars()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_groups() -> Vec<&'static str> {
        vec![
            "{}",
            "{{{}}}",
            "{{},{}}",
            "{{{},{},{{}}}}",
            "{<{},{},{{}}>}",
            "{<a>,<a>,<a>,<a>}",
            "{{<a>},{<a>},{<a>},{<a>}}",
            "{{<!>},{<!>},{<!>},{<a>}}",
        ]
    }

    #[test]
    fn test_count() {
        let expected = vec![1, 3, 3, 6, 1, 1, 5, 2];
        for (g, e) in example_groups().iter().zip(expected.iter()) {
            println!("Expecting {e} groups in \"{g}\"...", g = g, e = e);
            let parsed = parse(g);
            println!("  Parsed as {:?}", parsed);
            match parsed {
                Ok((g, _)) => {
                    match g.ok() {
                        Some(box_group) => {
                            assert_eq!(box_group.count(), *e);
                        }
                        None => panic!("Parsed a group as garbage!"),
                    }
                }
                e => panic!(e),
            }
        }
    }

    #[test]
    fn test_score() {
        let expected = vec![1, 6, 5, 16, 1, 1, 9, 3];
        for (g, e) in example_groups().iter().zip(expected.iter()) {
            println!("Expecting \"{g}\" to score {e}...", g = g, e = e);
            let parsed = parse(g);
            println!("  Parsed as {:?}", parsed);
            match parsed {
                Ok((g, _)) => {
                    match g.ok() {
                        Some(box_group) => {
                            assert_eq!(box_group.score(), *e);
                        }
                        None => panic!("Parsed a group as garbage!"),
                    }
                }
                e => panic!(e),
            }
        }
    }
}
