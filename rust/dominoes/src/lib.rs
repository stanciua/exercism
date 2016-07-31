extern crate permutohedron;
use permutohedron::Heap;

pub fn chain(input: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut cell_and_inv_cell_vec = input.iter()
                                         .map(|&(x, y)| ((x, y), (y, x)))
                                         .collect::<Vec<_>>();

    let heap = Heap::new(&mut cell_and_inv_cell_vec);
    let mut solution = Vec::new();
    for o in heap {
        let mut cur_cell = o[0].0;
        let first_cell = cur_cell;
        solution.push(cur_cell);
        for &(cell, inv_cell) in o.iter().skip(1) {
            if cur_cell.1 == cell.0 {
                cur_cell = cell;
                solution.push(cell);
            } else if cur_cell.1 == inv_cell.0 {
                cur_cell = inv_cell;
                solution.push(inv_cell);
            }
        }
        // if len of solution is the same as the len of input we need to check if
        // the first digit is the same as the last one
        if solution.len() == input.len() {
            if first_cell.0 == cur_cell.1 {
                // found the solution, just return and ignore the other solutions
                return Some(solution);
            }
        }
        solution.clear();
    }
    None
}
