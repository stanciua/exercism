extern crate itertools;

use std::collections::{HashSet, HashMap};
use itertools::Itertools;


pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let puzzle_split = puzzle.split("==").map(|s| s.trim()).collect::<Vec<_>>();
    if puzzle_split.len() != 2 {
        return None;
    }
    let result = puzzle_split[1];
    let left_tkns = puzzle_split[0]
        .split('+')
        .map(|s| s.trim())
        .collect::<Vec<_>>();


    let all_chars = left_tkns
        .iter()
        .flat_map(|s| s.chars())
        .chain(puzzle_split[1].chars())
        .unique()
        .collect::<String>();

    let combinations = (0..10)
        .into_iter()
        .cartesian_product(all_chars.chars())
        .into_iter()
        .combinations(3)
        .filter(|v| {
                    v.iter().map(|&(d, _)| d).collect::<HashSet<_>>().len() == v.len() &&
                    v.iter().map(|&(_, c)| c).collect::<HashSet<_>>().len() == v.len()
                })
        .map(|v| v.iter().map(|&(x, y)| (y, x as u8)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for combination in combinations {
        println!("{:?}", combination);
        if is_combination_alphametic(left_tkns.as_slice(), result, &combination) {
            return Some(combination 
                            .iter()  
                            .map(|&(x, y)| (x, y))                          
                            .collect::<HashMap<_, _>>());
        }
    }
    None
}

fn is_combination_alphametic(left: &[&str], right: &str, combination: &[(char, u8)]) -> bool {
    let result = str_to_num(right, combination);

    let mut num = 0; 
    for token in left {        
        num += str_to_num(*token, combination);
    }

    if result == 100 {
        println!("result: {:?}", result);
    }
    
    if num == 100 {
        println!("num: {:?}", num);
    }
    
    result == num
}
fn str_to_num(val: &str, lnums_slice: &[(char, u8)]) -> u32 {
    let mut num = 1;
    let mut val_slice = val;
    for letter in val.chars() {
        let digit = lnums_slice
                         .iter()
                         .map(|&(x, y)| (x, y))
                         .collect::<HashMap<_, _>>()[&letter];
        num += 10u32.pow(val_slice.len() as u32 - 1) * digit as u32;
        val_slice = &val_slice[1..];
    }
    
    num



}