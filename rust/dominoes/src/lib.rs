extern crate permutohedron;
use permutohedron::Heap;

pub fn chain(input: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut input_as_vec = input.iter().cloned().collect::<Vec<_>>();
    let input_permutations = Heap::new(&mut input_as_vec);
    let mut solution = Vec::new();
    for permutation in input_permutations {
        let mut cur_cell = permutation[0];
        solution.push(cur_cell);
        for &(x, y) in permutation.iter().skip(1) {
            if cur_cell.1 == x {
                cur_cell = (x, y);
                solution.push(cur_cell);
            } else if cur_cell.1 == y {
                cur_cell = (y, x);
                solution.push(cur_cell);
            }
        }
        // if len of solution is the same as the len of input we need to check if
        // the first digit is the same as the last one
        if solution.len() == input.len() {
            if solution[0].0 == cur_cell.1 {
                // found the solution, just return and ignore the other solutions
                return Some(solution);
            }
        }
        solution.clear();
    }
    None
}
