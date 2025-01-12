use crate::program_state::ExecutionSkeleton;
use crate::variable::MemoryType::{Heap, Stack};
use crate::weighted_choices::WeightedChoices;
use rand::prelude::StdRng;
use rand::Rng;

pub struct SkeletonGenerationMethods;

impl SkeletonGenerationMethods {
    /**
        For 0507s and 0505s
    **/
    pub fn get_borrow_moves(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };
        history.push(ExecutionSkeleton::Init(memory_type));
        history.push(ExecutionSkeleton::Borrow);
        history.push(ExecutionSkeleton::Move);
        if rng.gen_bool(0.2) {
            history.push(ExecutionSkeleton::Move);
        }
        if rng.gen_bool(0.5) {
            history.push(ExecutionSkeleton::Read(true));
        }
        history
    }

    pub fn get_standard(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        loop {
            if history.is_empty() {
                let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };

                history.push(ExecutionSkeleton::Init(memory_type));
            }

            // also try for smaller programs
            let history_length = history.iter().count();
            if history_length > 2 {
                let random = rng.gen::<f64>();
                if random < 0.6 * (history_length - 2) as f64 {
                    break;
                }
            }

            let mut choices = WeightedChoices::new();
            choices.add(ExecutionSkeleton::Borrow, 0.175);
            choices.add(ExecutionSkeleton::Move, 0.175);

            let choice = choices.choose(rng);
            history.push(choice.unwrap());
        }

        if rng.gen_bool(0.3) {
            history.push(ExecutionSkeleton::Read(true));
        }

        history
    }

}

pub fn fill_skeleton(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
    if rng.gen_bool(0.5) {
        SkeletonGenerationMethods::get_borrow_moves(rng)
    } else {
        SkeletonGenerationMethods::get_standard(rng)
    }
}
