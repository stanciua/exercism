extern crate itertools;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

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

    let max_lgth = data.iter().map(|s| s.len()).max().unwrap();

    let data = normalize_tokens(data.as_mut_slice(), max_lgth);
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
        .cloned()
        .collect::<String>();

    let mut m = HashMap::new();

    m.entry('_').or_insert(0);
    if find_solution(
        data.as_slice(),
        &mut m,
        0,
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

fn find_solution(
    data: &[&str],
    m: &mut HashMap<char, u8>,
    carry: u8,
    sum: u8,
    curr_row: usize,
    max_rows: usize,
    curr_col: i8,
    leading_zero_letters: &str,
) -> bool {
    if curr_col == -1 {
        return carry == 0;
    }

    if curr_row != max_rows - 1 {
        let letter = data[curr_row].chars().nth(curr_col as usize).unwrap();

        if m.get(&letter).is_some() {
            let v: u8;
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
                .difference(&m.iter()
                    .filter(|&(&x, _)| x != '_')
                    .map(|(_, &y)| y)
                    .collect::<HashSet<_>>())
                .cloned()
                .collect::<Vec<_>>();
            avail_digits.sort();

            for d in avail_digits {
                if d == 0 {
                    if leading_zero_letters.chars().any(|c| c == letter) {
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
        let local_carry: u8;
        // we are evaluating the SUM now
        let letter = data[curr_row].chars().nth(curr_col as usize).unwrap();
        if m.get(&letter).is_some() {
            let v = m[&letter];
            // if char is assigned and matches the sum value
            let mut calc_val = sum + carry;
            local_carry = calc_val / 10;
            calc_val %= 10;
            if v == calc_val {
                if find_solution(
                    data,
                    m,
                    local_carry,
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
            let mut calc_val = sum + carry;
            local_carry = calc_val / 10;
            calc_val %= 10;

            if calc_val == 0 {
                if leading_zero_letters.chars().any(|c| c == letter) {
                    return false;
                }
            }
            if m.iter().any(|(&c, &d)| d == calc_val && c != '_') {
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
                    local_carry,
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
            padded_tkn.push_str(tkn);
            output.push(padded_tkn);
        } else {
            output.push(tkn.to_string());
        }
    }
    output
}
