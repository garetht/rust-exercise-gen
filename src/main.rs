use crate::checker::{check_rust_code, CompilerMessage};
use crate::formatter::format_rust_code;
use crate::outline_generator::fill_outline;
use crate::program_renderer::render_program;
use crate::program_state::ExecutionSkeleton;
use crate::skeleton_generator::fill_skeleton;
use crate::variable::OutlineStatement;
use rand::prelude::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;

mod checker;
mod formatter;
mod namer;
mod outline_generator;
mod program_renderer;
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
    seed: u64
}

fn main() {
    let results: Vec<(Vec<String>, OutputProgram)> = (0..500)
        .into_par_iter()
        .filter_map(|i| {
            let seed = 42 * i;
            let mut rng = StdRng::seed_from_u64(seed);

            let (program, skeleton, outline, errors) = create_and_check(&mut rng);
            if errors.len() > 2 {
                return None;
            }

            let mut error_codes: Vec<String> = errors
                .iter()
                .filter_map(|e| e.code.as_ref().map(|c| c.code.clone()))
                .collect();
            error_codes.dedup();
            error_codes.sort();

            Some((error_codes, OutputProgram {
                compiler_messages: errors,
                skeleton,
                outline,
                formatted_program: program,
                seed
            }))
        })
        .collect();

    let mut programs_by_error: HashMap<Vec<String>, Vec<OutputProgram>> = HashMap::new();
    for (error_codes, program) in results {
        programs_by_error.entry(error_codes).or_default().push(program);
    }

    // for i in 0..100 {
    //     let seed = 42 * i;
    //     // let seed = 336;
    //     let mut rng = StdRng::seed_from_u64(seed);
    //
    //     let (program, skeleton, outline, errors) = create_and_check(&mut rng);
    //     if errors.len() > 2 {
    //         continue;
    //     }
    //
    //     if errors.is_empty() {
    //         programs_by_error
    //             .entry(vec![])
    //             .or_insert_with(Vec::new)
    //             .push(OutputProgram {
    //                 compiler_messages: errors,
    //                 skeleton,
    //                 outline,
    //                 formatted_program: program,
    //                 seed
    //             });
    //         continue;
    //     }
    //
    //     let error_codes = errors
    //         .iter()
    //         .filter_map(|e| {
    //             if let Some(_) = e.clone().code.map(|e| e.code) {
    //                 Some(e)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //
    //     let mut codes = error_codes
    //         .iter()
    //         .flat_map(|e| e.code.clone())
    //         .map(|e| e.code)
    //         .collect::<Vec<_>>();
    //     codes.dedup();
    //     codes.sort();
    //     programs_by_error
    //         .entry(codes)
    //         .or_insert_with(Vec::new)
    //         .push(OutputProgram {
    //             compiler_messages: errors,
    //             skeleton,
    //             outline,
    //             formatted_program: program,
    //             seed
    //         });
    // }

    // programs_by_error.iter().for_each(|p| {
    //     println!("\n{:?} {:?}", p.0, p.1.len());
    //     p.1.iter().for_each(|op| {
    //         println!("{}\n {}", op.seed, op.formatted_program);
    //     });
    // })

    let program_count_by_error = programs_by_error
        .iter()
        .map(|(error, programs)| {
            (error, programs.len(), programs.iter().map(|p| p.skeleton.clone()).collect::<Vec<_>>(), programs.iter().map(|p| p.formatted_program.clone()).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    program_count_by_error.iter().for_each(|pcbe| {
        println!("\n{:?} {:?}", pcbe.0, pcbe.1);
        pcbe.2.iter().for_each(|skeleton| {
            println!("{:?}", skeleton);
            // if pcbe.0.len() > 0 && pcbe.0.first().unwrap() == "E0507" {
            //     pcbe.3.iter().for_each(|program| {
            //         println!("{}", program);
            //     });
            // }
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
                    if message.level == "error" {
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
