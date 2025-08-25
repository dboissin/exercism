use std::collections::{HashMap, HashSet};

type InternalId = usize;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(InternalId);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(InternalId);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(InternalId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl From<InternalId> for ComputeCellId {
    fn from(value: InternalId) -> Self {
        ComputeCellId(value)
    }
}

impl From<InternalId> for InputCellId {
    fn from(value: InternalId) -> Self {
        InputCellId(value)
    }
}

impl Into<InternalId> for CellId {
    fn into(self) -> InternalId {
        match self {
            CellId::Compute(value) => value.0,
            CellId::Input(value) => value.0,
        }
    }
}

impl Into<InternalId> for InputCellId {
    fn into(self) -> InternalId {
        self.0
    }
}

impl Into<InternalId> for ComputeCellId {
    fn into(self) -> InternalId {
        self.0
    }
}

struct Node<'a, T> {
    value: T,
    compute_func: Option<Box<dyn Fn(&[T]) -> T>>,
    callbacks: HashMap<InternalId, Box<dyn FnMut(T) + 'a>>,
    dependencies: Option<Vec<CellId>>,
    dependents: Vec<InternalId>
}

impl<'a, T: Copy + PartialEq> Node<'a, T> {

    fn new(value: T) -> Node<'a, T> {
        Node { value: value, compute_func: None, callbacks: HashMap::new(), dependencies: None, dependents: Vec::new() }
    }

    fn new_compute(value: T, compute_func: Box<dyn Fn(&[T]) -> T>, dependencies: Vec<CellId>) -> Node<'a, T> {
        Node { value: value, compute_func: Some(compute_func), callbacks: HashMap::new(), dependencies: Some(dependencies), dependents: Vec::new() }
    }
    
    fn add_dependent(&mut self, id: InternalId) {
        self.dependents.push(id);
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    nodes: HashMap<InternalId, Node<'a, T>>,
    seq_id: InternalId
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor { nodes: HashMap::new(), seq_id: 0 }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = self.next_id();
        self.nodes.insert(id, Node::new(initial));
        InputCellId::from(id)
    }

    fn next_id(&mut self) -> InternalId {
        let id = self.seq_id;
        self.seq_id += 1;
        id
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
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = self.next_id();
        let deps = dependencies.iter().map(|dep| *dep).collect();
        let res = compute_func(self.get_cells(Some(id), &deps)?.as_slice());
        let node = Node::new_compute(res, Box::new(compute_func), deps);
        self.nodes.insert(id, node);
        Ok(ComputeCellId(id))
    }

    fn get_cells(&mut self, id: Option<InternalId>, dependencies: &Vec<CellId>) -> Result<Vec<T>, CellId> {
        let mut args = Vec::with_capacity(dependencies.len());
        for cell_id in dependencies {
            if let Some(node) = self.nodes.get_mut(&(*cell_id).into()) {
                args.push(node.value);
                if id.is_some() {
                    node.add_dependent(id.unwrap());
                }
            } else {
                return Err(*cell_id)
            }
        }
        Ok(args)
    }

    fn compute(&mut self, node_id: InternalId, values: &[T]) -> Option<Vec<InternalId>> {
        if let Some(node) = self.nodes.get_mut(&node_id) && node.compute_func.is_some() {
            let old_value = node.value;
            node.value = node.compute_func.as_ref().unwrap()(values);
            if old_value != node.value {
                if !node.callbacks.is_empty() {
                    for callback in node.callbacks.values_mut() {
                        callback(node.value);
                    }
                }
                if !node.dependents.is_empty() {
                    return  Some(node.dependents.clone());
                }
            }
        }
        None
    }

    fn update_compute(&mut self, node_id: InternalId) -> Result<Vec<InternalId>, CellId> {
        if let Some(node) = self.nodes.get(&node_id) {
            let dependencies = node.dependencies.as_ref().unwrap().clone();
            let dependencies_values = self.get_cells(None, &dependencies)?;
            if let Some(node_ids) = self.compute(node_id, dependencies_values.as_slice()) {
                return Ok(node_ids)
            }
        }
        Ok(Vec::new())
    }

    fn update_computes(&mut self, node_id: InternalId) {
        let mut dependents:HashSet<InternalId> = HashSet::new();
        dependents.extend(self.nodes.get(&node_id).map(|node| &node.dependents).unwrap());
        while !dependents.is_empty() {
            let mut childs_deps:HashSet<InternalId> = HashSet::new();
            for dependent_id in dependents {
                if let Ok(node_ids) = self.update_compute(dependent_id) && !node_ids.is_empty() {
                    childs_deps.extend(node_ids);
                }
            }
            dependents = childs_deps;
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.nodes.get(&id.into()).map(|node| node.value)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let internal_id = id.into();
        if let Some(node) = self.nodes.get_mut(&internal_id) {
            node.value = new_value;
            self.update_computes(internal_id);
            true
        } else {
            false
        }
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
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let callback_id = self.next_id();
        if let Some(node) = self.nodes.get_mut(&id.into()) {
            node.callbacks.insert(callback_id, Box::new(callback));
            return Some(CallbackId(callback_id))
        }
        None
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(node) = self.nodes.get_mut(&cell.into()) {
            if let Some(_) = node.callbacks.remove(&callback.0) {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }

}
