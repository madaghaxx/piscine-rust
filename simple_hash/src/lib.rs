use std::collections::HashMap;
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut res: HashMap<&'a str, usize> = Default::default();
    for i in 0..words.len(){
        *res.entry(words[i]).or_insert(0) += 1;
    }
    res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
