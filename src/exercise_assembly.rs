use crate::checker::{check_rust_code, CompilerMessage};
use crate::formatter::format_rust_code;
use crate::outline_generator::fill_outline;
use crate::program_renderer::render_program;
use crate::program_state::ExecutionSkeleton;
use crate::skeleton_generator::fill_skeleton;
use crate::variable::OutlineStatement;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;
use protobuf::{Message as ProtobufMessage, MessageField, SpecialFields};
use crate::protos::exercises::{ErrorExerciseGroup, ErrorMessage, ErrorMessageCode, ErrorMessageSpan, Exercise, Exercises};

#[derive(Debug, Clone)]
struct OutputProgram {
    compiler_messages: Vec<CompilerMessage>,
    skeleton: Vec<ExecutionSkeleton>,
    outline: Vec<OutlineStatement>,
    formatted_program: String,
    seed: u64
}


pub fn generate_and_write_programs() -> std::io::Result<()> {
    let mut programs_by_error = group_programs_by_error();
    let exercises = convert_to_protobuf(&mut programs_by_error);

    let bytes = exercises.write_to_bytes()?;
    let mut file = File::create("exercises.pb.bin")?;
    file.write_all(&bytes)
}

fn convert_to_protobuf(programs_by_error: &mut HashMap<Vec<String>, Vec<OutputProgram>>) -> Exercises {
    let mut rng = StdRng::seed_from_u64(2048);

    programs_by_error
        .retain(|key, value| {
            !key.contains(&String::from("E207")) && !key.contains(&String::from("E308"))
        });

    for (_, output_programs) in programs_by_error.iter_mut() {
        // Modify values in-place
        output_programs.shuffle(&mut rng);
        output_programs.truncate(200);
    }

    let exercise_groups = programs_by_error.iter().map(|(error_vec, output_programs)| {
        let exercises = output_programs.iter().map(|output_program| {
            Exercise {
                special_fields: SpecialFields::default(),
                formatted_program: output_program.formatted_program.clone(),
                errors: output_program.compiler_messages.iter().map(|compiler_message| {
                    ErrorMessage {
                        special_fields: SpecialFields::default(),
                        message: compiler_message.message.clone(),
                        code: MessageField(Some(Box::new(
                            compiler_message.clone().code.map(|c| {
                                ErrorMessageCode {
                                    special_fields: SpecialFields::default(),
                                    code: c.code.clone(),
                                    explanation: c.explanation.unwrap()
                                }
                            }).unwrap()
                        ))),
                        spans: compiler_message.spans.iter().map(|span| ErrorMessageSpan {
                            special_fields: SpecialFields::default(),
                            line_start: span.line_start,
                            line_end: span.line_end,
                            column_start: span.column_start,
                            column_end: span.column_end
                        }).collect()
                    }
                }).collect()
            }
        }).collect();
        ErrorExerciseGroup {
            special_fields: SpecialFields::default(),
            error_codes: error_vec.clone(),
            exercises
        }
    }).collect();

    Exercises {
        special_fields: SpecialFields::default(),
        exercise_groups
    }
}

fn group_programs_by_error() -> HashMap<Vec<String>, Vec<OutputProgram>> {
    let results: Vec<(Vec<String>, OutputProgram)> = (0..10000)
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
            // if pcbe.0.len() > 0 && pcbe.0.first().unwrap() == "E0503" {
            //     pcbe.3.iter().for_each(|program| {
            //         println!("{}", program);
            //     });
            // }
        });
    });

    programs_by_error
}

fn create_and_check(rng: &mut StdRng) -> (String, Vec<ExecutionSkeleton>, Vec<OutlineStatement>, Vec<crate::checker::CompilerMessage>) {
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

