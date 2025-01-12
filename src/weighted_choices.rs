use rand::Rng;
use std::collections::BTreeMap;
use std::hash::Hash;
use rand::prelude::StdRng;

pub struct WeightedChoices<T> {
    choices: BTreeMap<T, f64>,
}

impl<T: Clone + Eq + Hash + Ord> WeightedChoices<T> {
    pub fn exponential_decreasing(max: u8) -> WeightedChoices<u8> {
        let mut choices = WeightedChoices::new();

        let mut weight = 1.0;
        for i in 1..(max + 1) {
            let value = i;
            let weight = weight / 2.0;
            choices.add(value, weight);
        }
        choices
    }

    pub fn new() -> Self {
        WeightedChoices {
            choices: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, value: T, weight: f64) {
        self.choices.insert(value, weight);
    }

    fn weighted_choices(&self) -> BTreeMap<T, f64> {
        let scaling_factor = 1.0 / self.choices.values().sum::<f64>();
        self.choices
            .iter()
            .map(|(key, val)| (key.clone(), val * scaling_factor))
            .collect()
    }

    pub fn choose(&self, rng: &mut StdRng) -> Option<T> {
        let random = rng.gen::<f64>();

        let mut cumulative = 0.0;
        for (value, weight) in &self.weighted_choices() {
            cumulative += weight;
            if random <= cumulative {
                return Some(value.clone());
            }
        }
        None
    }
}
