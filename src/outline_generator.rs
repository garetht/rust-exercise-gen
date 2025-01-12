use crate::program_state::ExecutionSkeleton;
use crate::variable::{rand_borrow, rand_initialize_variable, rand_move, rand_read, AvailableVariables, OutlineStatement};
use rand::prelude::StdRng;

pub fn fill_outline(rng: &mut StdRng, execution_skeletion: &Vec<ExecutionSkeleton>) -> Vec<OutlineStatement> {
    let mut program_outline: Vec<OutlineStatement> = vec![];
    for instruction in execution_skeletion {
        // match all variable assignments and extract them
        let available_variables = calculate_available_variables(&program_outline);

        match instruction {
            // any assignments past the first may not respect the stack/heap distinction
            ExecutionSkeleton::Init(memory_type, mutability) => {
                let assignment = rand_initialize_variable(
                    rng,
                    available_variables,
                    memory_type,
                    mutability
                );
                program_outline.push(OutlineStatement::VariableDeclaration(assignment.clone()));
            }
            ExecutionSkeleton::Move => program_outline.push(
                rand_move(rng, &available_variables)
            ),
            ExecutionSkeleton::Borrow(mutability) => {
                program_outline.push(rand_borrow(rng, &available_variables, mutability));
            }
            ExecutionSkeleton::Read(read_all) => {
                program_outline.push(rand_read(rng, &available_variables, *read_all));
            }
            ExecutionSkeleton::Write => {}
        }
    }
    program_outline
}

fn calculate_available_variables(program_outline: &Vec<OutlineStatement>) -> AvailableVariables {
    let variables = program_outline
        .iter()
        .filter_map(|statement| match statement {
            OutlineStatement::VariableDeclaration(declaration) => Some(declaration),
            _ => None,
        })
        .cloned()
        .collect();

    AvailableVariables::new(variables)
}
