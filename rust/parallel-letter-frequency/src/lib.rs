extern crate rayon;

use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(text: &[&str], _num_threads: usize) -> HashMap<char, usize> {
    let letter_count = text.par_iter()
        .map(|line| {
            line.chars().filter(|ch| ch.is_alphabetic()).fold(HashMap::new(), |mut acc, val| {
                {
                    let count = acc.entry(val.to_lowercase().next().unwrap()).or_insert(0);
                    *count += 1;
                }
                acc
            })
        })
        .reduce_with(|acc, x| {
            x.iter().fold(acc, |mut acc, (key, val)| {
                {
                    let new_val = acc.entry(*key).or_insert(0);
                    *new_val += *val;
                }
                acc
            })
        });

    if let Some(rslt) = letter_count {
        println!("{:?}", rslt);
        rslt
    } else {
        HashMap::new()
    }


}
