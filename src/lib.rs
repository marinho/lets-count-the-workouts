use constants::APP_TITLE;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod constants;
mod models;
mod pages;

// Top-Level pages
use crate::models::workout::Workout;
use crate::models::workout_set::WorkoutSet;
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (workouts, set_workouts) = signal::<Vec<Workout>>(Vec::new());

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text=APP_TITLE />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route
                    path=path!("/")
                    view=move || view! { <Home workouts=workouts set_workouts=set_workouts /> }
                />
            </Routes>
        </Router>
    }
}
