use crate::checker::{check_rust_code, CompilerMessage, DiagnosticCode};
use crate::formatter::format_rust_code;
use crate::outline_generator::fill_outline;
use crate::program_generator::render_program;
use crate::skeleton_generator::fill_skeleton;
use crate::variable::OutlineStatement;
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::collections::{HashMap, HashSet};
use crate::program_state::ExecutionSkeleton;

mod checker;
mod formatter;
mod namer;
mod outline_generator;
mod program_generator;
mod program_state;
mod skeleton_generator;
mod variable;
mod weighted_choices;

#[derive(Debug)]
struct OutputProgram {
    compiler_messages: Vec<CompilerMessage>,
    skeleton: Vec<ExecutionSkeleton>,
    outline: Vec<OutlineStatement>,
    formatted_program: String,
}

fn main() {
    let mut programs_by_error: HashMap<Vec<String>, Vec<OutputProgram>> = HashMap::new();

    for i in 0..100 {
        let seed = 42 * i;
        // let seed = 1764;
        let mut rng = StdRng::seed_from_u64(seed);

        let (program, skeleton, outline, errors) = create_and_check(&mut rng);

        if errors.len() > 2 {
            continue;
        }

        if errors.is_empty() {
            programs_by_error
                .entry(vec![])
                .or_insert_with(Vec::new)
                .push(OutputProgram {
                    compiler_messages: errors,
                    skeleton,
                    outline,
                    formatted_program: program,
                });
            continue;
        }

        let error_codes = errors
            .iter()
            .filter_map(|e| {
                if let Some(_) = e.clone().code.map(|e| e.code) {
                    Some(e)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut codes = error_codes
            .iter()
            .flat_map(|e| e.code.clone())
            .map(|e| e.code)
            .collect::<Vec<_>>();
        codes.dedup();
        codes.sort();
        programs_by_error
            .entry(codes)
            .or_insert_with(Vec::new)
            .push(OutputProgram {
                compiler_messages: errors,
                skeleton,
                outline,
                formatted_program: program,
            });
    }

    let program_count_by_error = programs_by_error
        .iter()
        .map(|(error, programs)| {
            (error, programs.len(), programs.iter().map(|p| p.skeleton.clone()).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    program_count_by_error.iter().for_each(|pcbe| {
        println!("\n{:?} {:?}", pcbe.0, pcbe.1);
        pcbe.2.iter().for_each(|skeleton| {
            println!("{:?}", skeleton);
        });
    })
}

fn create_and_check(rng: &mut StdRng) -> (String, Vec<ExecutionSkeleton>, Vec<OutlineStatement>, Vec<CompilerMessage>) {
    let (skeleton, outline, formatted_program) = create_program(rng);
    let messages = check_rust_code(&*formatted_program).unwrap();
    (
        formatted_program,
        skeleton,
        outline,
        messages
            .iter()
            .filter_map(|message| match message.code {
                None => None,
                Some(_) => {
                    if (message.level == "error") {
                        Some(message.clone())
                    } else {
                        None
                    }
                }
            })
            .collect(),
    )
}

fn create_program(mut rng: &mut StdRng) -> (Vec<ExecutionSkeleton>, Vec<OutlineStatement>, String) {
    let skeleton = fill_skeleton(&mut rng);
    let outline = fill_outline(&mut rng, &skeleton);
    let program = render_program(&outline);

    let formatted_program = match format_rust_code(&*program.join("\n")) {
        Ok(formatted_program) => formatted_program,
        Err(e) => {
            panic!("Error formatting program: {}", e);
        }
    };

    (skeleton, outline, formatted_program)
}
