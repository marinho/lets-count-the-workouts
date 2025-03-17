use crate::Workout;
use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn DeleteWorkoutButton(index: usize, set_workouts: WriteSignal<Vec<Workout>>) -> impl IntoView {
    view! {
        <button
            class="delete-workout"
            on:click=move |_| {
                set_workouts
                    .update(|workouts| {
                        workouts.remove(index);
                    });
            }
        >
            <i class="fas fa-trash"></i>
        </button>
    }
}
