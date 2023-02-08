use std::collections::{HashSet, BinaryHeap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let to_sorted_elements = |x: &str|
        x.to_lowercase().chars().collect::<BinaryHeap<char>>().into_sorted_vec();
    possible_anagrams.iter().filter(|e| {
        to_sorted_elements(word) == to_sorted_elements(e) && word.to_lowercase() != e.to_lowercase()
    }).cloned().collect()
}
