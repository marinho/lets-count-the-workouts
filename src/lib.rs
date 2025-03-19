use constants::APP_TITLE;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use web_sys::window;

// Modules
mod components;
mod constants;
mod models;
mod pages;

// Top-Level pages
use crate::models::workout::Workout;
use crate::models::workout_set::WorkoutSet;
use crate::pages::home::Home;

fn get_base_path() -> String {
    let window = window().unwrap();
    let pathname = window
        .location()
        .pathname()
        .expect("pathname should be available");

    let base_path = pathname
        .split('/')
        .filter(|segment| !segment.is_empty())
        .next()
        .map(|segment| format!("/{}", segment))
        .unwrap_or_else(|| "/".to_string());

    return base_path;
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (workouts, set_workouts) = signal::<Vec<Workout>>(Vec::new());

    let base_path = get_base_path();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text=APP_TITLE />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router base=base_path>
            <Routes fallback=|| view! { NotFound }>
                <Route
                    path=path!("/")
                    view=move || view! { <Home workouts=workouts set_workouts=set_workouts /> }
                />
            </Routes>
        </Router>
    }
}
