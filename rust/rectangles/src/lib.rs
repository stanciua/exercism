extern crate itertools;
use std::collections::HashSet;
use itertools::Itertools;

type Point = (i32, i32);

pub fn count(lines: &[&str]) -> i32 {
    if lines.is_empty() {
        return 0;
    }
    let rectangle_corners = lines.iter().enumerate().fold(HashSet::new(),
                                                          |acc, (outer_idx, val)| {
        val.chars()
            .enumerate()
            .filter(|&(_, ch)| ch == '+')
            .fold(acc, |mut acc, (inner_idx, _)| {
                acc.insert((outer_idx as i32, inner_idx as i32));
                acc
            })
    });

    // get the the list of total points from the diagram, then grab all the
    // right corners and feed it to a function that will calculate the left
    // corner
    let rectangles = (0i32..lines.len() as i32)
        .cartesian_product(0i32..lines[0].len() as i32)
        .filter(|&(x, y)| x != 0 && y != 0)
        .map(|corner| get_list_of_rects_from_right_corner(corner))
        .fold(HashSet::<(Point, Point)>::new(), |mut acc, val| {
            acc.extend(val.iter());
            acc
        });


    count_rectangles(&rectangle_corners, &rectangles, lines)
}

// gets the list of rectangles, (up_left, right_down) pairs starting from down right corner
fn get_list_of_rects_from_right_corner(point: Point) -> HashSet<(Point, Point)> {
    let mut v = HashSet::new();
    let mut left_corner = (point.0 - 1, point.1 - 1);
    // populates up left and down right pair of corners
    while left_corner.0 >= 0 {
        while left_corner.1 >= 0 {
            v.insert((left_corner, point));
            left_corner = (left_corner.0, left_corner.1 - 1);
        }
        left_corner = (left_corner.0 - 1, point.1 - 1);
    }

    v

}

// this function validates if the rectangle has been formed using valid ASCII
// characters
//   - horizontal: + | -
//   - vertical:   + | |
fn does_rectangle_contains_valid_chars(up_left: Point, right_down: Point, lines: &[&str]) -> bool {
    lines.iter()
            .skip(up_left.0 as usize)
            .take(1)
            .next()
            .unwrap()[up_left.1 as usize..right_down.1 as usize + 1]
        .chars()
        .all(|y| y == '+' || y == '-') &&
    lines.iter()
            .skip(right_down.0 as usize)
            .take(1)
            .next()
            .unwrap()[up_left.1 as usize..right_down.1 as usize + 1]
        .chars()
        .all(|y| y == '+' || y == '-') &&
    lines.iter()
        .skip(up_left.0 as usize)
        .take(right_down.0 as usize - up_left.0 as usize + 1)
        .map(|str| str.chars().skip(up_left.1 as usize).take(1).next().unwrap())
        .all(|c| c == '+' || c == '|') &&
    lines.iter()
        .skip(up_left.0 as usize)
        .take(right_down.0 as usize - up_left.0 as usize + 1)
        .map(|str| str.chars().skip(right_down.1 as usize).take(1).next().unwrap())
        .all(|c| c == '+' || c == '|')

}

fn count_rectangles(rectangle_points: &HashSet<Point>,
                    from: &HashSet<(Point, Point)>,
                    lines: &[&str])
                    -> i32 {
    let mut count = 0;
    for &(up_left_corner, right_down_corner) in from {
        if rectangle_points.contains(&up_left_corner) &&
           rectangle_points.contains(&right_down_corner) &&
           rectangle_points.contains(&(right_down_corner.0, up_left_corner.1)) &&
           rectangle_points.contains(&(up_left_corner.0, right_down_corner.1)) &&
           does_rectangle_contains_valid_chars(up_left_corner, right_down_corner, lines) {
            count += 1;
        }
    }
    count
}
