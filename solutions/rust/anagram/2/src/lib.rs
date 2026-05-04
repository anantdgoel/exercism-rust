use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower = word.to_lowercase();
    let counts = char_counts(&lower);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let cand_lower = candidate.to_lowercase();
            cand_lower != lower && char_counts(&cand_lower) == counts
        })
        .copied()
        .collect()
}

fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}
