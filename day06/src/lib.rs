use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type Banks = Vec<usize>;

fn first_index_of_max_val(banks: &Banks) -> Option<usize> {
    banks.iter().max().map(|max_size| {
        // unwrap should work because I'm very confident that the list does contain
        // its max value
        banks.iter().position(|size| size == max_size).unwrap()
    })
}

pub fn redistribute(banks: &mut Banks) {
    if banks.is_empty() {
        return;
    }
    // unwrap is safe because we know there's at least one item
    let first_matching_index = first_index_of_max_val(banks).unwrap();
    let max_size = banks[first_matching_index];
    banks[first_matching_index] = 0;
    for r_index in 0..max_size {
        let index = (1 + first_matching_index + r_index) % banks.len();
        banks[index] += 1;
    }
}

fn hash_of(banks: &Banks) -> u64 {
    let mut hasher = DefaultHasher::new();
    banks.hash(&mut hasher);
    hasher.finish()
}

/// Compute the length of the redistributions cycle
///
/// Returns a 2-tuple whose elements are
///
/// ```text
/// (
///   num cycles before a previously-encountered state occurs,
///   length of detected loop
/// )
/// ```
pub fn redistributions_cycle_len(banks: &Banks) -> (usize, usize) {
    let mut cycle_len = 0;
    let mut banks = banks.clone();
    let mut state_hashes = HashMap::new();

    while !state_hashes.contains_key(&hash_of(&banks)) {
        state_hashes.insert(hash_of(&banks), cycle_len);
        cycle_len += 1;
        redistribute(&mut banks);
    }

    (
        cycle_len,
        cycle_len - state_hashes.get(&hash_of(&banks)).unwrap(),
    )
}
