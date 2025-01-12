use crate::variable::{MemoryType};

/**
 * A summary of the items we want to exist in the program
 */
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ExecutionSkeleton {
    Init(MemoryType),
    Move,
    Borrow,
    Read(bool),
    Write, // i.e. a push to an array, or assigning something?
}
