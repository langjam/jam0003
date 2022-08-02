///! BAM! virtual machine.
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    compiler::{Flow, Unit},
    Value,
};
use lazy_static::lazy_static;

/// A reference-counted, mutable stream in memory.
pub struct SharedStream {
    data: Rc<RefCell<Flow>>,
    // TODO: make this Optional
    // by adding a `Hold` instr to create it only when necessary:
    // Make a flow peekable, invloves adding a cache for it.
    // This is only observable through `Peek` flows. Everyone
    // else who holds the handle will consume from the stream normally.
    // Hold(Handle)
    cache: VecDeque<Value>,
}

/// The execution context of one machine.
pub struct Task {
    /// Locally-reachable streams, indexed by symbols.
    locals: Vec<SharedStream>,
}

/// Execution factory.
pub struct Factory {
    /// Call stack.
    stack: Vec<Task>,
}
