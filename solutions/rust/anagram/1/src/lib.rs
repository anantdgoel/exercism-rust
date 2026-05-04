use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let lower = word.to_lowercase();
    let mut charset: HashMap<char, usize> = HashMap::new();
    for c in lower.chars() {
        *charset.entry(c).or_default() += 1;
    }

    let mut anagrams = HashSet::new();
    for candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if candidate_lower == lower {
            continue;
        }
        let mut candidate_charset = HashMap::new();
        for c in candidate_lower.chars() {
            *candidate_charset.entry(c).or_default() += 1;
        }
        if candidate_charset == charset {
            anagrams.insert(*candidate);
        };
    }

    anagrams
}
