use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = Default::default();

    for possible_anagram in possible_anagrams {
        // Converting to lowercase means that this will be case insensitive whatever the input be
        let mut possible_anagram_chars: Vec<char> = possible_anagram.to_lowercase().chars().collect();
        let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();

        // Before rearrange the chars prevents detecting same word as is own anagram
        if word_chars.eq(&possible_anagram_chars) {
            break;
        }

        // Sorting in order to just compare if the two strings are equals
        word_chars.sort_unstable();
        possible_anagram_chars.sort_unstable();


        if word_chars.eq(&possible_anagram_chars) {
            result.insert(possible_anagram);
        }
    }

    result
}
