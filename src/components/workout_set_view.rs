use crate::{
    components::{
        copy_workout_set_button::CopyWorkoutSetButton,
        delete_workout_set_button::DeleteWorkoutSetButton, reps_selector::RepsSelector,
        weight_selector::WeightSelector,
    },
    models::workout::Workout,
};

use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn WorkoutSetView(
    workout_signal: ReadSignal<Workout>,
    set_workouts: WriteSignal<Vec<Workout>>,
    workout_index: usize,
) -> impl IntoView {
    view! {
        <div class="workout-set">
            <span class="inputs">
                <RepsSelector />
                <WeightSelector />
            </span>

            <span class="buttons">
                <CopyWorkoutSetButton />
                <DeleteWorkoutSetButton />
            </span>
        </div>
    }
}
