use crate::constants::REPETITIONS_MINIMUM;
use crate::constants::WEIGHT_MINIMUM;
use crate::Workout;
use crate::WorkoutSet;
use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn AddWorkoutButton(set_workouts: WriteSignal<Vec<Workout>>) -> impl IntoView {
    view! {
        <button
            class="add-workout"
            on:click=move |_| {
                set_workouts
                    .update(|workouts| {
                        workouts
                            .push(
                                Workout::new_with_sets(
                                    "New Workout".to_string(),
                                    vec![WorkoutSet::new(REPETITIONS_MINIMUM, WEIGHT_MINIMUM)],
                                ),
                            );
                    });
            }
        >
            "+ Workout"
        </button>
    }
}
