extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::fmt;
use itertools::Itertools;
use std::cmp;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let puzzle_split = puzzle.split("==").map(|s| s.trim()).collect::<Vec<_>>();
    if puzzle_split.len() != 2 {
        return None;
    }
    let result = puzzle_split[1];
    let mut data = puzzle_split[0]
        .split('+')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    data.push(result);

    let data = normalize_tokens(data.as_mut_slice(), result.len());
    let data = data.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    println!("data: {:?}", data);
    let mut leading_zero_letters = data.iter()
        .map(|s| s.chars().take(1).next().unwrap())
        .unique()
        .collect::<Vec<_>>();

    leading_zero_letters.push(result.chars().take(1).next().unwrap());

    let leading_zero_letters = leading_zero_letters
        .into_iter()
        .collect::<HashSet<_>>()
        .iter()
        .map(|&x| x)
        .collect::<String>();

    let mut m = HashMap::new();
    m.entry('_').or_insert(0);
    if find_solution(
        data.as_slice(),
        &mut m,
        &mut 0,
        0,
        0,
        data.len(),
        (data[0].len() - 1) as i8,
        leading_zero_letters.as_str(),
    )
    {
        m.remove(&'_');
        return Some(m);
    }

    None
}

fn assign_letter(m: &mut HashMap<char, u8>, leading_zeroes: &str, letter: char, digit: u8) -> bool {
    if leading_zeroes
        .chars()
        .find(|&c| c == letter && digit == 0)
        .is_some() || m.iter().find(|&(_, &d)| digit == d).is_some()
    {
        return false;
    }

    m.insert(letter, digit);

    return true;
}

pub fn unassign_letter(m: &mut HashMap<char, u8>, letter: char, digit: u8) {
    if let Some(idx) = m.iter().position(|(&x, &y)| x == letter && digit == y) {
        m.remove(&letter);
    }
}

fn find_solution(
    data: &[&str],
    m: &mut HashMap<char, u8>,
    carry: &mut u8,
    sum: u8,
    curr_row: usize,
    max_rows: usize,
    curr_col: i8,
    leading_zero_letters: &str,
) -> bool {
    // println!("M: {:?}", m);
    // println!("CARRY: {:?}", *carry);
    // println!("SUM: {:?}", sum);
    // println!("CURR_ROW: {:?}", curr_row);
    // println!("CURR_COL: {:?}", curr_col);
    // // If we are on the last iterm of data, we are in the SUM, this means that
    // // this has the leftmost digit of the terms
    // // If we try to assign a char in one of the addends
    // let mut mm = HashMap::new();
    // mm.entry('_').or_insert(0);
    // mm.entry('I').or_insert(1);
    // mm.entry('B').or_insert(9);
    // mm.entry('L').or_insert(0);
    // if *m == mm {
    //     println!("hello");
    // }
    if curr_col == -1 {
        if *carry == 0 {
            return true;
        } else {
            return false;
        }
    }

    if curr_row != max_rows - 1 {
        let mut letter = data[curr_row]
            .chars()
            .skip(curr_col as usize)
            .next()
            .unwrap();

        if m.contains_key(&letter) {
            let mut v = 0;
            {
                v = m[&letter];
            }
            if find_solution(
                data,
                m,
                carry,
                sum + v,
                curr_row + 1,
                max_rows,
                curr_col,
                leading_zero_letters,
            )
            {
                return true;
            }
        } else {
            // letter is not already assigned, iterate over available digits
            let mut avail_digits = (0..10u8)
                .into_iter()
                .collect::<HashSet<_>>()
                .difference(&m.values().cloned().collect::<HashSet<_>>())
                .cloned()
                .collect::<Vec<_>>();
            avail_digits.sort();

            for d in avail_digits {
                if d == 0 {
                    if let Some(_) = leading_zero_letters.chars().find(|&c| c == letter) {
                        continue;
                    }
                }
                m.insert(letter, d);
                if find_solution(
                    data,
                    m,
                    carry,
                    sum + d,
                    curr_row + 1,
                    max_rows,
                    curr_col,
                    leading_zero_letters,
                )
                {
                    return true;
                } else {
                    m.remove(&letter);
                }
            }
            return false;

        }

    } else if curr_row == max_rows - 1 {

        // we are evaluating the SUM now
        let mut letter = data[curr_row]
            .chars()
            .skip(curr_col as usize)
            .next()
            .unwrap();
        if m.contains_key(&letter) {
            let mut v = 0;
            {
                v = m[&letter];
            }
            // if char is assigned and matches the sum value
            let mut calc_val = sum + *carry;
            if calc_val >= 10 {
                *carry = 1;
                calc_val -= 10;
            } else {
                *carry = 0;
            }

            if v == calc_val {
                if find_solution(
                    data,
                    m,
                    carry,
                    0,
                    0,
                    max_rows,
                    curr_col - 1,
                    leading_zero_letters,
                )
                {
                    return true;
                }
            } else {
                // char is assigned and doesn't match the sum value
                return false;
            }

        } else {
            // char is not assigned and correct digit is already in use
            let mut calc_val = sum + *carry;
            if calc_val >= 10 {
                *carry = 1;
                calc_val -= 10;
            } else {
                *carry = 0;
            }
            let letter = data[curr_row]
                .chars()
                .skip(curr_col as usize)
                .next()
                .unwrap();
            if let Some(_) = m.iter().find(|&(&c, &d)| d == calc_val && c != '_') {
                return false;
            } else {
                // if calc_val >= 10 {
                //     println!("M: {:?}", m);
                //     println!("CARRY: {:?}", *carry);
                //     println!("SUM: {:?}", sum);
                //     println!("CURR_ROW: {:?}", curr_row);
                //     println!("CURR_COL: {:?}", curr_col);
                //     panic!("boom");
                // }
                m.insert(letter, calc_val);

                if find_solution(
                    data,
                    m,
                    carry,
                    0,
                    0,
                    max_rows,
                    curr_col - 1,
                    leading_zero_letters,
                )
                {
                    return true;
                } else {
                    m.remove(&letter);
                }
            }
        }
    }
    false
}

fn normalize_tokens(tkns: &mut [&str], max_len: usize) -> Vec<String> {
    let mut output = Vec::new();

    for tkn in tkns.iter_mut() {
        if tkn.len() < max_len {
            let mut padded_tkn = "_".repeat(max_len - tkn.len());
            padded_tkn.extend(tkn.chars());
            output.push(padded_tkn);
        } else {
            output.push(tkn.to_string());
        }
    }
    output
}
