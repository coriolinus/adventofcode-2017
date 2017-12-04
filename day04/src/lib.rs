extern crate counter;
use counter::Counter;

use std::hash::Hash;

pub fn contains_only_unique<S, T>(phrase: &S) -> bool
where
    S: AsRef<[T]>,
    T: Hash + Eq,
{
    Counter::init(phrase.as_ref()).map.values().all(|&count| {
        count == 1
    })
}

pub fn count_unique<OuterSlice, InnerSlice, T>(list: &OuterSlice) -> usize
where
    OuterSlice: AsRef<[InnerSlice]>,
    InnerSlice: AsRef<[T]>,
    T: Hash + Eq,
{
    list.as_ref().iter().filter(contains_only_unique).count()
}

pub fn contains_anagram<S, T>(phrase: &S) -> bool
where
    S: AsRef<[T]>,
    T: AsRef<str>,
{
    let counts = phrase
        .as_ref()
        .iter()
        .map(|word| Counter::init(word.as_ref().chars()).map)
        .collect::<Vec<_>>();
    for (outer_idx, outer) in counts.iter().enumerate() {
        for (inner_idx, inner) in counts.iter().enumerate() {
            if outer_idx != inner_idx && outer == inner {
                return true;
            }
        }
    }
    false
}

pub fn is_valid<Phrase, T>(phrase: &Phrase) -> bool
where
    Phrase: AsRef<[T]>,
    T: Hash + Eq + AsRef<str>,
{
    contains_only_unique(phrase) && !contains_anagram(phrase)
}

pub fn count_valid<OuterSlice, InnerSlice, T>(list: &OuterSlice) -> usize
where
    OuterSlice: AsRef<[InnerSlice]>,
    InnerSlice: AsRef<[T]>,
    T: Hash + Eq + AsRef<str>,
{
    list.as_ref().iter().filter(is_valid).count()
}
