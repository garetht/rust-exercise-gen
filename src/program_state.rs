use rand::Rng;
use rand::rngs::StdRng;
use crate::variable::{MemoryType};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mutability {
    Mutable,
    Immutable,
    Defer
}

impl Mutability {
    pub fn is_mutable(&self, rng: &mut StdRng) -> bool {
        match self {
            Mutability::Mutable => true,
            Mutability::Immutable => false,
            Mutability::Defer => rng.gen_bool(0.5)
        }
    }
}

/**
 * A summary of the items we want to exist in the program
 */
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ExecutionSkeleton {
    Init(MemoryType, Mutability),
    Move,
    Borrow(Mutability),
    Read(bool),
    Write, // i.e. a push to an array, or assigning something?
}
