use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn DeleteWorkoutSetButton() -> impl IntoView {
    // index: usize, set_workouts: WriteSignal<Vec<Workout>>

    view! {
        <button>
            // on:click=move |_| {
            // set_workouts
            // .update(|workouts| {
            // workouts.remove(index);
            // });
            // }
            <i class="fas fa-square-minus"></i>
        </button>
    }
}
