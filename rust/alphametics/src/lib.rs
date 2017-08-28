extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::fmt;
use itertools::Itertools;

struct Puzzle<'a> {
    left_to_right_data: &'a Vec<(String, char)>,
    all_chars: &'a str,
    char_dig: &'a mut Vec<(char, u8)>,
    leading_zeroes: &'a [char],
}

impl<'a> fmt::Debug for Puzzle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "left_to_right_data: {:?}, all_chars: {:?}, char_dig: {:?}, leading_zeroes: {:?}",
            self.left_to_right_data,
            self.all_chars,
            self.char_dig,
            self.leading_zeroes
        )
    }
}

impl<'a> Puzzle<'a> {
    pub fn new(
        left_to_right_data: &'a Vec<(String, char)>,
        chars: &'a str,
        char_dig: &'a mut Vec<(char, u8)>,
        leading_zeroes: &'a [char],
    ) -> Puzzle<'a> {
        Puzzle {
            left_to_right_data: left_to_right_data,
            all_chars: chars,
            char_dig: char_dig,
            leading_zeroes: leading_zeroes,
        }
    }

    pub fn assign_letter(&mut self, letter: char, digit: u8) -> bool {
        if self.leading_zeroes
            .iter()
            .find(|&&c| c == letter && digit == 0)
            .is_some() || self.char_dig.iter().find(|&&(_, d)| digit == d).is_some()
        {
            return false;
        }

        self.char_dig.push((letter, digit));

        true
    }

    pub fn unassign_letter(&mut self, letter: char, digit: u8) {
        if let Some(idx) = self.char_dig
            .iter()
            .position(|&(x, y)| x == letter && digit == y)
        {
            self.char_dig.remove(idx);
        }
    }

    pub fn find_solution(&mut self, carry: i32) -> bool {
        println!("puzzle: {:?}", *self);
        // if letter_to_assign.len() == 0 {
        //     return false;
        // }

        // for d in 0..10 {
        //     if self.assign_letter(letter_to_assign.chars().take(1).next().unwrap(), d) {
        //         if self.find_solution(0) {
        //             return true;
        //         }
        //         self.unassign_letter(letter_to_assign.chars().take(1).next().unwrap(), d);
        //     }
        // }
        // return false;
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

    let mut char_dig = Vec::new();
    let mut left_letters = &group_add_letter_togheter(&left_tkns);

    let mut puzzle = Puzzle::new(
        &left_letters
            .iter()
            .zip(result.chars())
            .collect::<Vec<(_, _)>>(),
        &all_chars,
        &mut char_dig,
        &leading_zero_letters,
    );

    if puzzle.find_solution(0) {
        return Some(
            puzzle
                .char_dig
                .iter()
                .map(|&(x, y)| (x, y))
                .collect::<HashMap<_, _>>(),
        );
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
