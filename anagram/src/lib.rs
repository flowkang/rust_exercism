use std::collections::{HashSet, BinaryHeap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.into_iter().filter(|e| {
        word.to_lowercase().chars().collect::<BinaryHeap<char>>().into_sorted_vec() ==
            e.to_lowercase().chars().collect::<BinaryHeap<char>>().into_sorted_vec() &&
            word.to_lowercase() != e.to_lowercase()
    }).cloned().collect()
}
