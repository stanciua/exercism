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
        Puzzle {
            data: data,
            all_chars: chars,
            char_dig: HashMap::new(),
            leading_zeroes: leading_zeroes,
        }
    }

    pub fn assign_letter(&mut self, letter: char, digit: u8) -> bool {
        println!("assing letter: {:?}, digit: {:?}", letter, digit);
        if self.leading_zeroes
            .iter()
            .find(|&&c| c == letter && digit == 0)
            .is_some() ||
            self.char_dig
                .iter()
                .find(|&(&l, &d)| digit == d && l != '_')
                .is_some()
        {
            return false;
        }

        // println!("assing letter: {:?}, digit: {:?}", letter, digit);
        self.char_dig.insert(letter, digit);
        true
    }

    pub fn unassign_letter(&mut self, letter: char, digit: u8) {
        println!("unassign letter: {:?}", letter);
        let mut found = false;
        match self.char_dig.get(&letter) {
            Some(&v) => {
                if v == digit {
                    found = true;
                }
            }
            None => (),
        }

        if found {
            // println!("unassign letter: {:?}", letter);
            self.char_dig.remove(&letter);
        }
    }


    pub fn find_solution(
        &mut self,
        add_data: &[&str],
        assign_letters: &str,
        result: &mut Vec<bool>,
        mut carry: i32,
    ) -> bool {
        // println!("add_data: {:?}", add_data);
        // println!("assign_letters: {:?}", assign_letters);
        if add_data.len() == 0 {
            return result.iter().all(|&x| x == true);
        }

        if add_data[0] == "__I" && assign_letters.len() == 0 {
            println!("boom!");
            println!("puzzle: {:?}", *self);
        }
        if self.char_dig.len() == add_data[0].chars().collect::<HashSet<_>>().len() {
            // println!("puzzle: {:?}", *self);
            // println!("assign_letters: {:?}", assign_letters);
            // println!("add_data: {:?}", add_data);
            // we can compute the now and compare it to the result
            let mut left = add_data[0][..add_data[0].len() - 1].chars().fold(
                0,
                |mut acc, v| {
                    acc += self.char_dig[&v];
                    acc
                },
            );


            // println!("left is: {:?}", left);
            // add the carry if non-zero
            left += carry as u8;

            if left >= 10 {
                carry = 1;
                left -= 10;
            } else {
                carry = 0
            }


            if self.char_dig[&add_data[0].chars().last().unwrap()] == left {
                result.push(true);
            } else {
                result.push(false);
            }

            // clear the letter to digit mapping
            self.char_dig.clear();
            // println!("cleard self.char_dig!!!");
            if add_data[1..].len() == 0 {
                self.find_solution(&add_data[1..], "", result, carry);
            } else {
                self.find_solution(&add_data[1..], add_data[1..][0], result, carry);
            }

        } else {
            for d in 0..10 {
                println!("digit: {:?}", d);
                println!("puzzle: {:?}", *self);
                println!("assign_letters: {:?}", assign_letters);
                println!("add_data: {:?}", add_data);
                if self.assign_letter(assign_letters.chars().take(1).next().unwrap(), d) {
                    if self.find_solution(add_data, &assign_letters[1..], result, carry) {
                        return true;
                    }
                    self.unassign_letter(assign_letters.chars().take(1).next().unwrap(), d);
                }
            }
            return false;
        }

        false
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
    let mut puzzle = Puzzle::new(
        left_and_right_data.as_slice(),
        &all_chars,
        &leading_zero_letters,
    );

    if puzzle.find_solution(puzzle.data, &puzzle.data[0], &mut vec![], 0) {
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
