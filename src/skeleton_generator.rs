use crate::program_state::ExecutionSkeleton;
use crate::variable::MemoryType::{Heap, Stack};
use crate::weighted_choices::WeightedChoices;
use rand::prelude::StdRng;
use rand::Rng;

pub fn fill_skeleton(rng: &mut StdRng) -> Vec<ExecutionSkeleton> {
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
        if history
            .iter()
            .any(|state| matches!(state, ExecutionSkeleton::Init(Heap)))
        {
            choices.add(ExecutionSkeleton::Borrow, 0.175);
            choices.add(ExecutionSkeleton::Move, 0.175);
        } else {
            choices.add(ExecutionSkeleton::Borrow, 0.35);
        }

        // if history
        //     .iter()
        //     .filter(|state| matches!(state, ExecutionSkeleton::Init(_)))
        //     .count()
        //     < 2
        // {
        //     choices.add(ExecutionSkeleton::Init(Heap), 0.1);
        //     choices.add(ExecutionSkeleton::Init(Stack), 0.1);
        //     choices.add(ExecutionSkeleton::Read, 0.1);
        // } else {
        // }
        choices.add(ExecutionSkeleton::Read, 0.01);

        let choice = choices.choose(rng);
        history.push(choice.unwrap());
    }

    history
}
