use crate::components::{
    add_workout_button::AddWorkoutButton, delete_workout_button::DeleteWorkoutButton,
    workout_set_view::WorkoutSetView,
};
use crate::constants::APP_TITLE;
use leptos::prelude::*;

use crate::models::workout::Workout;

/// Default Home Page
#[component]
pub fn Home(
    workouts: ReadSignal<Vec<Workout>>,
    set_workouts: WriteSignal<Vec<Workout>>,
) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div class="container">
                <header>
                    <h1>{APP_TITLE}</h1>
                    <AddWorkoutButton set_workouts=set_workouts />
                    <form id="date-selector">"Default: current date"</form>
                </header>

                <main>
                    <ul class="workouts">
                        {move || {
                            workouts
                                .get()
                                .iter()
                                .enumerate()
                                .map(|(index, workout)| {
                                    let (workout_signal, workout_signal_set) = signal(
                                        workout.clone(),
                                    );
                                    view! {
                                        <li>
                                            <span class="name">{workout.name.clone()}</span>
                                            <WorkoutSetView
                                                workout_signal=workout_signal
                                                set_workouts=set_workouts
                                                workout_index=index
                                            />
                                            <DeleteWorkoutButton
                                                index=index
                                                set_workouts=set_workouts
                                            />
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </ul>
                </main>
            </div>
        </ErrorBoundary>
    }
}
