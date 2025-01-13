use crate::char_utils::extract_backtick_text;
use crate::checker::{check_rust_code, CompilerMessage};
use crate::formatter::format_rust_code;
use crate::outline_generator::fill_outline;
use crate::program_renderer::render_program;
use crate::program_state::ExecutionSkeleton;
use crate::protos::exercises::{
    ErrorExerciseGroup, ErrorMessage, ErrorMessageCode, Exercise, Exercises,
};
use crate::skeleton_generator::fill_skeleton;
use crate::variable::{AvailableVariables, OutlineStatement};
use protobuf::{Message as ProtobufMessage, MessageField, SpecialFields};
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
struct OutputProgram {
    compiler_messages: Vec<CompilerMessage>,
    human_messages: Vec<String>,
    skeleton: Vec<ExecutionSkeleton>,
    outline: Vec<OutlineStatement>,
    formatted_program: String,
    variable_names: Vec<String>,
    seed: u64,
}

pub fn generate_and_write_programs_to_file() -> std::io::Result<()> {
    let mut programs_by_error = group_programs_by_error();
    let exercises = convert_to_protobuf(&mut programs_by_error);

    let bytes = exercises.write_to_bytes()?;
    let mut file = File::create("exercises.pb.bin")?;
    file.write_all(&bytes)
}

fn convert_to_protobuf(
    programs_by_error: &mut HashMap<Vec<String>, Vec<OutputProgram>>,
) -> Exercises {
    let mut rng = StdRng::seed_from_u64(2048);

    programs_by_error.retain(|key, value| {
        !key.contains(&String::from("E0207")) && !key.contains(&String::from("E0308")) && !key.contains(&String::from("E0277")) && key.len() <= 1
    });

    for (_, output_programs) in programs_by_error.iter_mut() {
        // Modify values in-place
        output_programs.shuffle(&mut rng);
        output_programs.truncate(200);
    }

    let exercise_groups = programs_by_error
        .iter()
        .map(|(error_vec, output_programs)| {
            let exercises = output_programs
                .iter()
                .map(|output_program| Exercise {
                    special_fields: SpecialFields::default(),
                    formatted_program: output_program.formatted_program.clone(),
                    program_length: output_program.skeleton.len() as i32,
                    variable_names: output_program.variable_names.clone(),
                    human_errors: output_program.human_messages.clone(),
                    errors: output_program
                        .compiler_messages
                        .iter()
                        .map(|compiler_message| {
                            let message = compiler_message.message.clone();
                            let implicated_variable_names = extract_backtick_text(&message);
                            ErrorMessage {
                                special_fields: SpecialFields::default(),
                                message,
                                implicated_variable_names,
                                code: MessageField(Some(Box::new(
                                    compiler_message
                                        .clone()
                                        .code
                                        .map(|c| ErrorMessageCode {
                                            special_fields: SpecialFields::default(),
                                            code: c.code.clone(),
                                        })
                                        .unwrap(),
                                ))),
                            }
                        })
                        .collect(),
                })
                .collect();
            ErrorExerciseGroup {
                special_fields: SpecialFields::default(),
                error_codes: error_vec.clone(),
                exercises,
            }
        })
        .collect();

    Exercises {
        special_fields: SpecialFields::default(),
        exercise_groups,
    }
}

fn group_programs_by_error() -> HashMap<Vec<String>, Vec<OutputProgram>> {
    let results: Vec<(Vec<String>, OutputProgram)> = (0..8000)
        .into_par_iter()
        .filter_map(|i| {
            let seed = 42 * i;
            let mut rng = StdRng::seed_from_u64(seed);

            let (program, skeleton, outline, available_variables, errors, human_errors) =
                create_and_check(&mut rng);
            if errors.len() > 2 {
                return None;
            }
            // println!("{}", program);
            // println!("{:?}", errors);

            let mut error_codes: Vec<String> = errors
                .iter()
                .filter_map(|e| e.code.as_ref().map(|c| c.code.clone()))
                .collect();
            error_codes.dedup();
            error_codes.sort();

            Some((
                error_codes,
                OutputProgram {
                    compiler_messages: errors,
                    human_messages: human_errors,
                    skeleton,
                    outline,
                    formatted_program: program,
                    seed,
                    variable_names: available_variables.names(),
                },
            ))
        })
        .collect();

    let mut programs_by_error: HashMap<Vec<String>, Vec<OutputProgram>> = HashMap::new();
    for (error_codes, program) in results {
        programs_by_error
            .entry(error_codes)
            .or_default()
            .push(program);
    }

    let program_count_by_error = programs_by_error
        .iter()
        .map(|(error, programs)| {
            (
                error,
                programs.len(),
                programs
                    .iter()
                    .map(|p| p.skeleton.clone())
                    .collect::<Vec<_>>(),
                programs
                    .iter()
                    .map(|p| p.formatted_program.clone())
                    .collect::<Vec<_>>(),
            )
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

fn create_and_check(
    rng: &mut StdRng,
) -> (
    String,
    Vec<ExecutionSkeleton>,
    Vec<OutlineStatement>,
    AvailableVariables,
    Vec<CompilerMessage>,
    Vec<String>
) {
    let (skeleton, outline, available_variables, formatted_program) = create_program(rng);
    let (json_messages, human_messages) = check_rust_code(&*formatted_program).unwrap();
    (
        formatted_program,
        skeleton,
        outline,
        available_variables,
        json_messages,
        human_messages
    )
}

fn create_program(
    mut rng: &mut StdRng,
) -> (
    Vec<ExecutionSkeleton>,
    Vec<OutlineStatement>,
    AvailableVariables,
    String,
) {
    let skeleton = fill_skeleton(&mut rng);
    let (outline, available_variables) = fill_outline(&mut rng, &skeleton);
    let program = render_program(&outline, &mut rng);

    let formatted_program = match format_rust_code(&*program.join("\n")) {
        Ok(formatted_program) => formatted_program,
        Err(e) => {
            panic!("Error formatting program: {}", e);
        }
    };

    (skeleton, outline, available_variables, formatted_program)
}
