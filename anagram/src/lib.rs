use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");

    let mut  anagrams = HashSet::new();

    for value in possible_anagrams.iter(){
        if is_anagram(word, *value){
            anagrams.insert(*value);
        }
    }

    anagrams

}

fn is_anagram(word: &str, given_word: &str) -> bool{

    if word == given_word || word.to_ascii_lowercase() == given_word.to_ascii_lowercase(){
        return false;
    }

    let word = word.to_ascii_lowercase();
    let given_word = given_word.to_ascii_lowercase();

    let mut  w: Vec<char> = word.chars().collect();
    let mut gw: Vec<char> = given_word.chars().collect();
    w.sort();
    gw.sort();


    if w == gw{
        println!("Anagram {:?}", gw);
        return true;
    }

    false
}
