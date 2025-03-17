use crate::models::workout_set::WorkoutSet;

#[derive(Clone)]
pub struct Workout {
    pub name: String,
    pub sets: Vec<WorkoutSet>,
}

impl Workout {
    pub fn new(name: String) -> Self {
        Self { name, sets: vec![] }
    }

    pub fn new_with_sets(name: String, sets: Vec<WorkoutSet>) -> Self {
        Self { name, sets }
    }
}
