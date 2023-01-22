// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    for x in magazine {
        magazine_map.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }

    for x in note {
        let matched = magazine_map.get(x);
        match matched {
            None => { return false; }
            Some(count) => {
                if *count > 0 {
                    magazine_map.insert(x, count - 1);
                } else {
                    return false;
                }
            }
        }
    }

    true
}
