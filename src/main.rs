use crate::exercise_assembly::{generate_and_write_programs_to_file};

mod checker;
mod formatter;
mod namer;
mod outline_generator;
mod program_renderer;
mod program_state;
mod skeleton_generator;
mod variable;
mod weighted_choices;
mod exercise_assembly;
mod protos;
mod char_utils;

fn main() {
    generate_and_write_programs_to_file();
}
