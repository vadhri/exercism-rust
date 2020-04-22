use std::collections::HashMap;
use std::time::SystemTime;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID {
    position: (u32, u32),
}


/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID {
    position: (u32, u32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackID {
     position: (u32, u32),
     cbid: u32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    input_sheet: HashMap<(u32, u32), T>,
    compute_sheet: HashMap<(u32, u32), (Vec<CellID>, Box<dyn Fn(&[T]) -> T>)>,
    callbacks: HashMap<(u32, u32), HashMap<u32, Box<dyn Fn(T) -> () + 'a>>>,
    last_entry_input: (u32, u32),
    last_entry_compute: (u32, u32)
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: std::fmt::Debug +  Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_sheet: HashMap::new(),
            compute_sheet: HashMap::new(),
            callbacks: HashMap::new(),
            last_entry_input: (0, 0),
            last_entry_compute: (0, 0)
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellID {
        let seed = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => 0,
        };

        if self.last_entry_input.0 == 0 {
            self.last_entry_input.0 = seed as u32;
        }

        let entry_position: (u32, u32) = (self.last_entry_input.0 + 1, self.last_entry_input.1 + 1);

        self.input_sheet.insert(entry_position, _initial.clone());
        self.last_entry_input = entry_position;

        InputCellID {
            position: entry_position
        }
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Copy +  'static +  Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellID],
        _compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let entry_position: (u32, u32) = (self.last_entry_compute.0 + 1, self.last_entry_compute.1 + 1);
        let mut output: Result<ComputeCellID, CellID> = Err(_dependencies[0]);

        for item in _dependencies {
            let position = match item {
                CellID::Compute(pos) => pos.position,
                CellID::Input(pos) => pos.position
            };

            match self.input_sheet.contains_key(&position) || self.compute_sheet.contains_key(&position) {
                true => {
                    self.last_entry_compute = entry_position;
                    self.compute_sheet.insert(entry_position,
                        (
                            _dependencies.to_vec(),
                            Box::new(_compute_func)
                        )
                    );

                    let compute_cell = ComputeCellID {
                        position: entry_position
                    };

                    output = Ok(compute_cell);
                },
                false => {
                    output = Err(*item);
                }
            }
        }
        output
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        let find_value = |id| {
            match id {
                CellID::Input(x) => {
                    match self.input_sheet.get(&x.position) {
                        Some(x) => Some(*x),
                        None => None
                    }
                },
                CellID::Compute(x) => {
                    match self.compute_sheet.get(&x.position) {
                        Some(x) => {
                            let (dependency_vector, compute_function) = x;
                            let mut values: Vec<T> = Vec::new();

                            for dependency in dependency_vector {
                                match dependency {
                                    CellID::Input(x) => {
                                        match self.input_sheet.contains_key(&x.position) {
                                            true => values.push(*self.input_sheet.get(&x.position).unwrap()),
                                            false => ()
                                        }
                                    },
                                    CellID::Compute(x) => {
                                        match self.value(CellID::Compute(*x)) {
                                            Some(re_computed) => values.push(re_computed),
                                            None => ()
                                        }
                                    }
                                }
                            }

                            Some(compute_function(&values))
                        }
                        None => None
                    }
                }
            }
        };

        find_value(id)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellID, _new_value: T) -> bool {
        let mut output = false;
        let mut find_dependencies = |_input: CellID| {
            let mut existing_values = HashMap::new();

            for (k, _) in self.compute_sheet.iter() {
                    let value = self.value(
                        CellID::Compute (ComputeCellID {
                            position: *k
                        })
                    ).unwrap();

                    existing_values.insert(k, value);
            }

            output = match self.input_sheet.get_mut(&_id.position) {
                Some(x) => {
                    *x = _new_value;
                    true
                },
                None => false
            };

            for (k, _) in self.compute_sheet.iter() {
                match self.callbacks.contains_key(&k) {
                    true =>  {
                        let callback_vector = self.callbacks.get(&k).unwrap();
                        for (_key, cb) in callback_vector.values().enumerate() {
                            let value = self.value(
                                CellID::Compute (ComputeCellID {
                                    position: *k
                                })
                            ).unwrap();

                            if value != *existing_values.get(&k).unwrap() {
                                (cb)(self.value(CellID::Compute (ComputeCellID {
                                    position: *k
                                })).unwrap())
                            }
                        }
                    },
                    false => ()
                }
            }
        };

        find_dependencies(CellID::Input(_id));
        output
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: Fn(T,) -> () +  FnMut(T) -> () + 'a>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        let mut cbid;
        match self.compute_sheet.contains_key(&_id.position) {
            true => {
                match self.callbacks.contains_key(&_id.position) {
                    false => {
                        let mut cb_vector: HashMap<u32, Box<dyn Fn(T) -> () + 'a>> = HashMap::new();
                        cb_vector.insert(0, Box::new(_callback));
                        self.callbacks.insert(_id.position, cb_vector);

                        cbid = 0;
                    },
                    true => {
                        let compute_cell_id = self.callbacks.get_mut(&_id.position).unwrap();
                        let max = compute_cell_id.keys().fold(0, |a, &b| a.max(b));

                        compute_cell_id.insert(max+1 as u32, Box::new(_callback));
                        cbid = compute_cell_id.keys().len() - 1;
                    }
                }

                Some(CallbackID {
                    position: _id.position,
                    cbid: cbid as u32
                })
            },
            false => None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let mut res: Result<(), RemoveCallbackError> =Ok(());

        match self.compute_sheet.contains_key(&cell.position) {
            true => {
                match self.callbacks.contains_key(&cell.position) {
                    true => {
                        let callback_list = self.callbacks.get_mut(&cell.position).unwrap();
                        match callback_list.contains_key(&callback.cbid) {
                            true => {
                                callback_list.remove(&callback.cbid);
                            },
                            false => {
                                res = Err(RemoveCallbackError::NonexistentCallback);
                            }
                        }

                    },
                    false => {
                        res = Err(RemoveCallbackError::NonexistentCallback);
                    }
                }
            },
            false => {
                res = Err(RemoveCallbackError::NonexistentCell);
            }
        }

        res
    }
}
