///! BAM! virtual machine.
use std::{cell::RefCell, rc::Rc};

use crate::compiler::{Flow, Unit};
use lazy_static::lazy_static;

/// A reference-counted, mutable stream in memory.
pub struct SharedStream(Rc<RefCell<Flow>>);

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

