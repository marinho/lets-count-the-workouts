#[derive(Clone)]
pub struct WorkoutSet {
    pub reps: u8,
    pub weight: u16,
}

impl WorkoutSet {
    pub fn new(reps: u8, weight: u16) -> Self {
        Self { reps, weight }
    }
}
