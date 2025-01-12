use crate::variable::{MemoryType, Position, VariableDeclaration};

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

pub struct ProgramState {
    variables_in_scope: Vec<VariableDeclaration>,
    function_calls_made: u8,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            variables_in_scope: Vec::new(),
            function_calls_made: 0,
        }
    }

    pub fn add_variable(&mut self, variable: VariableDeclaration) {
        self.variables_in_scope.push(variable);
    }

    pub fn get_variables_in_scope(&self) -> &Vec<VariableDeclaration> {
        &self.variables_in_scope
    }

    pub fn get_function_calls_made(&self) -> u8 {
        self.function_calls_made
    }

    pub fn increment_function_calls_made(&mut self) {
        self.function_calls_made += 1;
    }
}
