use std::collections::HashMap;

pub fn anagrams_for<'a, 'b>(keywd: &'a str, inputs: &'b [&str]) -> Vec<&'b str> {
    let keywd_upper = keywd.to_uppercase();
    let keywd_map = chars_count_in_word(&keywd_upper);
    let mut v = Vec::new();


    for input in inputs {
        let input_upper = input.to_uppercase();
        let input_map = chars_count_in_word(&input_upper);
        if input_map == keywd_map && keywd_upper != input_upper {
            v.push(*input);
        }
    }
    v
}

fn chars_count_in_word(word: &String) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut acc, val| {
        {
            let val = acc.entry(val).or_insert(0);
            *val += 1;
        }
        acc
    })
}
