use std::{collections::{HashMap, HashSet}};

fn is_anagram(a: &HashMap<char, u32>, b: &HashMap<char, u32>) -> bool {
    a.keys().all(|character| match b.get(character) {
        Some(count) if *count == *a.get(character).unwrap() => true,
        _ => false,
    })
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut anagrams = HashSet::new();

    // Maintain char frequency for the given word. We'll compare this with the char
    // frequency of each candidate words.
    let mut word_tally: HashMap<char, u32> = HashMap::new();

    // For each character in word, convert it to lower case. It's possible that
    // a character is made up of two or more chars; hence the tally on the char
    // frequency is made against each item in the Chars (chr_lower) iterator,
    // instead of the individual char in the word.
    for chr in word.chars() {

        // Avoid E0716: A temporary value is being dropped while a borrow is
        // still in active use.
        let str_lower = chr.to_lowercase().to_string();
        let chr_lower = str_lower.chars();

        for c in chr_lower {
            let count = word_tally
                .entry(c)
                .or_insert(0);
            *count += 1;
        }
    }

    println!("word_tally: {:?}", &word_tally);

    'outer: for candidate in possible_anagrams {

        if word.to_lowercase() == *candidate.to_lowercase() {
            continue 'outer;
        }

        let mut cand_tally : HashMap<char, u32> = HashMap::new();

        for chr in candidate.chars() {

            let str_lower = chr.to_lowercase().to_string();
            let chr_lower = str_lower.chars();

            for c in chr_lower {
                if !&word_tally.contains_key(&c) {
                    continue 'outer;
                }

                let count = cand_tally.entry(c).or_insert(0);
                *count += 1;
            }
        }

        println!("cand_tally: {:?}", &cand_tally);

        if &word_tally.len() == &cand_tally.len() && is_anagram(&word_tally, &cand_tally) {
            anagrams.insert(*candidate);
        }
    }

    anagrams
}