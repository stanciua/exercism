use std::collections::HashMap;

pub fn annotate(board: &[&str]) -> Vec<String> {
    let board_size = (board[0].len(), board.len());
    let mut board_map = board[..].into_iter().enumerate().fold(HashMap::new(), |acc, (y, row)| {
        row.chars().enumerate().fold(acc, |mut acc, (x, ch)| {
            acc.insert((x, y), ch);
            acc
        })
    });

    let search_pos: Vec<(usize, usize)> = board_map.iter()
        .filter(|&(_, &val)| val != '*')
        .map(|(&(x, y), _)| (x, y))
        .collect();


    // for each position get the neighbors and count the mines in them
    for (x, y) in search_pos {
        let count = cell_neighbors((x as isize, y as isize),
                                   (board_size.0 as isize, board_size.1 as isize))
            .into_iter()
            .fold(0, |mut acc, (x1, y1)| {
                if board_map[&(x1 as usize, y1 as usize)] == '*' {
                    acc += 1
                }
                acc
            });


        // update the board map if count is non-zero
        if count != 0 {
            if let Some(val) = board_map.get_mut(&(x, y)) {
                *val = format!("{}", count).chars().take(1).next().unwrap();
            }
        }
    }

    (0..board_size.1).fold(Vec::new(), |mut acc, val| {
        acc.push((0..board_size.0).fold(String::new(), |mut acci, vali| {
            acci.push(board_map[&(vali, val)]);
            acci
        }));
        acc
    })
}

fn cell_neighbors(cell: (isize, isize), board_size: (isize, isize)) -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    // N
    if cell.1 - 1 >= 0 {
        v.push((cell.0, cell.1 - 1));
    }
    // S
    if cell.1 + 1 < board_size.1 {
        v.push((cell.0, cell.1 + 1));
    }
    // V
    if cell.0 - 1 >= 0 {
        v.push((cell.0 - 1, cell.1));
    }
    // E
    if cell.0 + 1 < board_size.0 {
        v.push((cell.0 + 1, cell.1));
    }

    // diagonals are next

    // NV
    if cell.1 - 1 >= 0 && cell.0 - 1 >= 0 {
        v.push((cell.0 - 1, cell.1 - 1));
    }
    // SV
    if cell.1 + 1 < board_size.1 && cell.0 - 1 >= 0 {
        v.push((cell.0 - 1, cell.1 + 1));
    }
    // SE
    if cell.0 + 1 < board_size.0 && cell.1 + 1 < board_size.1 {
        v.push((cell.0 + 1, cell.1 + 1));
    }
    // NE
    if cell.0 + 1 < board_size.0 && cell.1 - 1 >= 0 {
        v.push((cell.0 + 1, cell.1 - 1));
    }
    v
}
