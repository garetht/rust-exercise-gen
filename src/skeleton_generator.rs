use crate::program_state::Mutability::{Defer, Immutable, Mutable};
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
        history.push(ExecutionSkeleton::Init(memory_type, Defer));
        history.push(ExecutionSkeleton::Borrow(Defer));
        history.push(ExecutionSkeleton::Move);
        if rng.gen_bool(0.2) {
            history.push(ExecutionSkeleton::Move);
        }
        if rng.gen_bool(0.5) {
            history.push(ExecutionSkeleton::Read(true));
        }
        history
    }

    pub fn get_borrow_move_borrows(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };
        history.push(ExecutionSkeleton::Init(memory_type, Defer));
        history.push(ExecutionSkeleton::Borrow(Defer));
        history.push(ExecutionSkeleton::Move);
        history.push(ExecutionSkeleton::Borrow(Defer));
        history
    }

    pub fn get503s(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };
        history.push(ExecutionSkeleton::Init(memory_type, Mutable));
        history.push(ExecutionSkeleton::Borrow(Mutable));
        history.push(ExecutionSkeleton::Move);
        history.push(ExecutionSkeleton::Borrow(Immutable));
        history
    }

    pub fn get499s(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };
        history.push(ExecutionSkeleton::Init(memory_type, Mutable));
        history.push(ExecutionSkeleton::Borrow(Mutable));
        history.push(ExecutionSkeleton::Borrow(Mutable));
        history
    }

    pub fn get_standard(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
        let mut history: Vec<ExecutionSkeleton> = vec![];
        loop {
            if history.is_empty() {
                let memory_type = if rng.gen_bool(0.5) { Heap } else { Stack };

                history.push(ExecutionSkeleton::Init(memory_type, Defer));
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
            choices.add(ExecutionSkeleton::Borrow(Defer), 0.175);
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
    let mut weighted_choices: WeightedChoices<fn(&mut StdRng) -> Vec<ExecutionSkeleton>> = WeightedChoices::new();
    weighted_choices.add(SkeletonGenerationMethods::get_borrow_moves, 0.25);
    weighted_choices.add(SkeletonGenerationMethods::get_borrow_move_borrows, 0.1);
    weighted_choices.add(SkeletonGenerationMethods::get503s, 0.1);
    weighted_choices.add(SkeletonGenerationMethods::get499s, 0.05);
    weighted_choices.add(SkeletonGenerationMethods::get_standard, 0.5);

    let function = weighted_choices.choose(rng).unwrap();
    function(rng)
}
