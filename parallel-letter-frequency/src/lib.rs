use std::{collections::HashMap};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    /*
    (
        "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{worker_count} workers"),
        }
    );*/
}


fn parallel_frequency(letter: &str)-> HashMap<char, usize>{

let mut frequencies = HashMap::new();
letter.chars().for_each(|c| {
    if frequencies.contains_key(&c){
       let value = *frequencies.get(&c).expect("Could not get the value");
       frequencies.insert(c, value + 1);
    }else{
        frequencies.insert(c, 1);
    }
});

frequencies
}
