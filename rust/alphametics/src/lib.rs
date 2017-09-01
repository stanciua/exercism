extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::fmt;
use itertools::Itertools;

struct Puzzle<'a> {
    data: &'a [&'a str],
    all_chars: &'a str,
    char_dig: HashMap<char, u8>,
    leading_zeroes: &'a [char],
}

impl<'a> fmt::Debug for Puzzle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "data: {:?}, all_chars: {:?}, char_dig: {:?}, leading_zeroes: {:?}",
            self.data,
            self.all_chars,
            self.char_dig,
            self.leading_zeroes
        )
    }
}

impl<'a> Puzzle<'a> {
    pub fn new(data: &'a [&'a str], chars: &'a str, leading_zeroes: &'a [char]) -> Puzzle<'a> {
        let mut puzzle = Puzzle {
            data: data,
            all_chars: chars,
            char_dig: HashMap::new(),
            leading_zeroes: leading_zeroes,
        };

        puzzle.char_dig.entry('_').or_insert(0);
        puzzle
    }

    pub fn assign_letter(&mut self, letter: char, digit: u8) -> bool {
        println!("try assing letter: {:?} to digit: {:?}", letter, digit);
        if letter == '_' {
            return true;
        }
        if self.leading_zeroes
            .iter()
            .find(|&&c| c == letter && digit == 0)
            .is_some() || self.char_dig.iter().find(|&(_, &d)| digit == d).is_some()
        {
            return false;
        }

        println!("assing letter: {:?} to digit: {:?}", letter, digit);
        self.char_dig.insert(letter, digit);
        true
    }

    pub fn unassign_letter(&mut self, letter: char, digit: u8) {
        println!("try unassing letter: {:?} to digit: {:?}", letter, digit);
        // don't remove _ from the map, it should always remain in there
        if letter == '_' {
            return;
        }
        let mut found = false;
        match self.char_dig.get(&letter) {
            Some(&v) => {
                if v == digit {
                    found = true;
                }
            }
            None => (),
        }

        println!("unassing letter: {:?} to digit: {:?}", letter, digit);
        if found {
            self.char_dig.remove(&letter);
        }
    }


    pub fn find_solution(
        &mut self,
        data: &[&str],
        assign_letters: &str,
        result: &mut Vec<bool>,
        mut carry: i32,
    ) -> bool {
        if data.len() == 0 {
            return result.iter().all(|&x| x == true);
        }

        let length = assign_letters.len();
        if length < data.len() * data[0].len() && (length % data[0].len()) == 0 {
            let p = (data.len() * data[0].len() - assign_letters.len()) / data[0].len() - 1;
            println!("p: {:?}", p);
            // we can compute the now and compare it to the result
            let mut left = data[p][..data[0].len() - 1].chars().fold(0, |mut acc, v| {
                acc += self.char_dig[&v];
                acc
            });

            // add the carry if non-zero
            left += carry as u8;

            if left >= 10 {
                carry = 1;
                left -= 10;
            } else {
                carry = 0
            }

            if self.char_dig[&data[p].chars().last().unwrap()] == left {
            } else {
                result.push(false);
            }
        }

        for d in 0..10 {
            println!("PUZZLE: {:?}", *self);
            println!("ASSIGN_LETTERS: {:?}", assign_letters);
            println!("DATA: {:?}", data);
            result.push(true);
            if self.assign_letter(assign_letters.chars().take(1).next().unwrap(), d) {
                if self.find_solution(data, &assign_letters[1..], result, carry) {
                    return true;
                }
            }
            self.unassign_letter(assign_letters.chars().take(1).next().unwrap(), d);
        }
        return false;
    }
}
fn normalize_tokens(tkns: &mut [String], max_len: usize) {
    for tkn in tkns.iter_mut() {
        if tkn.len() < max_len {
            let mut padded_tkn = "_".repeat(max_len - tkn.len());
            padded_tkn.extend(tkn.chars());
            *tkn = padded_tkn;
        }
    }
}

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

    let mut leading_zero_letters = left_tkns
        .iter()
        .map(|s| s.chars().take(1).next().unwrap())
        .unique()
        .collect::<Vec<_>>();
    leading_zero_letters.push(result.chars().take(1).next().unwrap());

    let leading_zero_letters = leading_zero_letters
        .into_iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&x| x)
        .collect::<Vec<_>>();

    let all_chars = left_tkns
        .iter()
        .flat_map(|s| s.chars())
        .chain(puzzle_split[1].chars())
        .unique()
        .collect::<String>();

    let mut left_tkns = left_tkns.iter().map(|x| x.to_string()).collect::<Vec<_>>();


    normalize_tokens(left_tkns.as_mut_slice(), result.len());

    let left_and_right_data = group_add_letter_togheter(&left_tkns)
        .into_iter()
        .zip(result.chars())
        .map(|(mut s, c)| {
            s.push(c);
            s
        })
        .collect::<Vec<_>>();

    left_tkns.push(result.to_string());
    let left_and_right_data = left_and_right_data
        .iter()
        .rev()
        .map(|x| x.as_str())
        .collect::<Vec<_>>();

    let assign_letters = left_and_right_data
        .iter()
        .flat_map(|s| s.chars())
        .collect::<String>();
    let mut puzzle = Puzzle::new(
        left_and_right_data.as_slice(),
        &all_chars,
        &leading_zero_letters,
    );

    if puzzle.find_solution(puzzle.data, &assign_letters, &mut vec![], 0) {
        return Some(puzzle.char_dig);
    }

    None
}

fn group_add_letter_togheter<'a>(left_tkns: &'a [String]) -> Vec<String> {
    let mut output = Vec::new();
    for i in 0..left_tkns[0].len() {
        let letters = left_tkns.iter().fold(String::new(), |mut acc, tkn| {
            if let Some(l) = tkn.chars().skip(i).take(1).next() {
                acc.push(l);
            }
            acc
        });

        output.push(letters);
    }
    output
}
