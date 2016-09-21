extern crate rand;

use rand::Rng;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet};

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = u64;
pub type CallbackID = u64;

enum Cell<'a, T: Debug + Copy + PartialEq> {
    Input(T),
    Compute(Vec<CellID>, Box<Fn(&[T]) -> T>, HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>),
}

pub struct Reactor<'a, T: Debug + Copy + PartialEq> {
    cells: Vec<Cell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: 'a + Debug + Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::default() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::Input(initial));

        (self.cells.len() - 1) as CellID
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(&mut self,
                                                      dependencies: &[CellID],
                                                      compute_func: F)
                                                      -> Result<CellID, ()> {
        if !dependencies.iter().all(|v| (*v as usize) < self.cells.len()) {
            return Err(());
        }

        self.cells.push(Cell::Compute(dependencies.to_vec(),
                                      Box::new(compute_func),
                                      HashMap::default()));

        Ok((self.cells.len() - 1) as CellID)

    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        if let Some(val) = self.cells.get(id as usize) {
            match *val {
                Cell::Input(v) => return Some(v),
                Cell::Compute(ref deps, ref f, _) => {
                    return Some(f(deps.iter()
                        .map(|cid| self.value(*cid).unwrap())
                        .collect::<Vec<_>>()
                        .as_slice()));
                }
            }
        }
        None
    }

    fn get_dependencies_for_cell_id(&self,
                                    id: CellID,
                                    impacted_computed_cells: &mut HashSet<CellID>) {
        for (idx, cell) in self.cells.iter().enumerate() {
            if let Cell::Compute(ref deps, _, _) = *cell {
                if deps.contains(&id) {
                    impacted_computed_cells.insert(idx as CellID);
                    self.get_dependencies_for_cell_id(idx as CellID, impacted_computed_cells);
                }
            }
        }

    }
    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        let mut impacted_computed_cells = HashSet::new();
        self.get_dependencies_for_cell_id(id, &mut impacted_computed_cells);

        // callbacks need to be triggered only if the value has been changed
        let computed_cells_old_values = impacted_computed_cells.iter()
            .map(|idx| (idx, self.value(*idx as CellID).unwrap()))
            .collect::<Vec<_>>();


        match self.cells.get_mut(id as usize) {
            Some(&mut Cell::Input(ref mut val)) => {
                *val = new_value;
            }
            _ => return Err(()),
        }

        let computed_cells_new_values = impacted_computed_cells.iter()
            .map(|idx| (idx, self.value(*idx as CellID).unwrap()))
            .collect::<Vec<_>>();



        let changed_computed_cells = computed_cells_old_values.iter()
            .zip(computed_cells_new_values.iter())
            .filter(|&(&(_, old_val), &(_, new_val))| old_val != new_val)
            .map(|(&(_, _), &(idx, val))| (idx, val))
            .collect::<Vec<_>>();

        for (idx, val) in changed_computed_cells {
            if let Some(&mut Cell::Compute(_, _, ref mut callbacks)) = self.cells
                .get_mut(*idx as usize) {
                callbacks.iter_mut().map(|(_, f)| f(val)).all(|_| true);
            }
        }

        Ok(())

    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self,
                                                id: CellID,
                                                callback: F)
                                                -> Result<CallbackID, ()> {
        if (id as usize) > self.cells.len() {
            return Err(());
        }

        let mut rng = rand::thread_rng();
        let mut clbkid = rng.gen::<CallbackID>();
        if let Cell::Compute(_, _, ref mut callbacks) = self.cells[id as usize] {
            while callbacks.contains_key(&clbkid) {
                clbkid = rng.gen::<CallbackID>();
            }
            callbacks.insert(clbkid, Box::new(callback));
        }

        Ok(clbkid)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        if (cell as usize) > self.cells.len() {
            return Err(());
        }

        match self.cells[cell as usize] {
            Cell::Compute(_, _, ref mut callbacks) => {
                if !callbacks.contains_key(&callback) {
                    return Err(());

                }
                let _ = callbacks.remove(&callback);
            }
            Cell::Input(_) => return Err(()),
        }

        Ok(())
    }
}
