pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(&str, usize)> = phrase
        .split_whitespace()
        .map(|word| {
            let position = word.chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            (word, position)
        })
        .collect();
    
    words.sort_by_key(|&(_, pos)| pos);
    
    words.iter()
        .map(|(word, _)| word.chars().filter(|c| !c.is_ascii_digit()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}