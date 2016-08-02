extern crate itertools;
use std::collections::HashSet;
use itertools::Itertools;

pub fn count(lines: &[&str]) -> usize {
    let rectangle_corners = lines.iter().enumerate().fold(HashSet::new(),
                                                          |acc, (outer_idx, val)| {
        val.chars().enumerate().filter(|&(_, ch)| ch == '+').fold(acc, |mut acc, (inner_idx, _)| {
            acc.insert((outer_idx, inner_idx));
            acc
        })
    });

    let (no_rows, no_cols) = (lines.len(), lines[0].len());


    let points = (0..no_rows).cartesian_product(0..no_cols).collect::<Vec<_>>();

    let right_corners = points.iter().filter(|&&(x, y)| x != 0 && y != 0).collect::<Vec<_>>();

    0
}

fn get_list_of_rects_from_point(point: (u32, u32), board_size: (u32, u32)) -> Vec<((u32, u32), (u32, u32)) {

}
