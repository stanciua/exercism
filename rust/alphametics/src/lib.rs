extern crate itertools;

use std::collections::{HashSet, HashMap};
use std::fmt;
use itertools::Itertools;

struct Puzzle<'a> {
    all_tkns: Vec<String>,
    all_chars: &'a str,
    char_dig: &'a mut Vec<(char, u8)>,
    leading_zeroes: &'a [char],
}

impl<'a> fmt::Debug for Puzzle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "all_tkns: {:?}, all_chars: {:?}, char_dig: {:?}, leading_zeroes: {:?}",
            self.all_tkns,
            self.all_chars,
            self.char_dig,
            self.leading_zeroes
        )
    }
}

impl<'a> Puzzle<'a> {
    pub fn new(
        all_tkns: Vec<String>,
        chars: &'a str,
        char_dig: &'a mut Vec<(char, u8)>,
        leading_zeroes: &'a [char],
    ) -> Puzzle<'a> {
        Puzzle {
            all_tkns: all_tkns,
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

    pub fn puzzle_solved(&self) -> bool {
        return self.is_combination_alphametic();
    }

    pub fn unassign_letter(&mut self, letter: char, digit: u8) {
        if let Some(idx) = self.char_dig.iter().position(
            |&(x, y)| x == letter && digit == y,
        )
        {
            self.char_dig.remove(idx);
        }
    }
    pub fn find_solution(&mut self, letter_to_assign: &str, carry: i32) -> bool {
        if letter_to_assign.len() == 0 {
            return self.puzzle_solved();
        }

        for d in 0..10 {
            if self.assign_letter(letter_to_assign.chars().take(1).next().unwrap(), d) {
                if self.find_solution(&letter_to_assign[1..], 0) {
                    return true;
                }
                self.unassign_letter(letter_to_assign.chars().take(1).next().unwrap(), d);
            }
        }
        return false;
    }

    fn is_combination_alphametic(&self) -> bool {
        let result = self.str_to_num(self.all_tkns.last().unwrap(), self.char_dig);
        if result.is_none() {
            return false;
        }

        let mut num = 0;
        for token in &self.all_tkns[..self.all_tkns.len() - 1] {
            let rslt = self.str_to_num(&*token, self.char_dig);
            if rslt.is_none() {
                return false;
            }
            num += rslt.unwrap();
        }

        result.unwrap() == num
    }

    fn str_to_num(&self, val: &str, lnums_slice: &[(char, u8)]) -> Option<u32> {
        let mut num = 0;
        let mut val_slice = val;
        let letter_to_digit_map = lnums_slice.iter().cloned().collect::<HashMap<_, _>>();

        if let Some(l) = val.chars().take(1).next() {
            if l != '_' && letter_to_digit_map[&l] == 0 {
                return None;
            }
        }
        for letter in val.chars() {
            let mut digit = 0;
            if letter == '_' {
                digit = 0;
            } else {
                digit = letter_to_digit_map[&letter];
            }
            num += 10u32.pow(val_slice.len() as u32 - 1) * digit as u32;
            val_slice = &val_slice[1..];
        }

        Some(num)
    }
}

fn normalize_tokens(tkns: &mut [String]) {
    let mut tkn_max_lgth = tkns[0].len();
    if let Some(max) = tkns.iter_mut().max_by(|x, y| x.len().cmp(&y.len())) {
        tkn_max_lgth = max.len();
    }
    for tkn in tkns.iter_mut() {
        if tkn.len() < tkn_max_lgth {
            let mut padded_tkn = "_".repeat(tkn_max_lgth - tkn.len());
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

    let mut all_tkns = left_tkns.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    all_tkns.push(result.to_string());

    normalize_tokens(all_tkns.as_mut_slice());

    let mut char_dig = Vec::new();
    let mut puzzle = Puzzle::new(all_tkns, &all_chars, &mut char_dig, &leading_zero_letters);

    if puzzle.find_solution(puzzle.all_chars, 0) {
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
