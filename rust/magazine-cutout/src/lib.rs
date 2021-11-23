// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    let mut magazine_words = HashMap::new();
    let mut note_words = HashMap::new();

    for word in magazine {
        let count = magazine_words.entry(word).or_insert(0);
        *count += 1;
    }
    
    for word in note {
        let count = note_words.entry(word).or_insert(0);
        *count += 1;
    }
    
    for (n_word, n_count) in note_words {
        let is_enough = match magazine_words.get(n_word) {
            Some(m_count) => *m_count >= n_count,
            None => false,
        };
        if !is_enough {
            return false;
        }
    }
    
    true
}