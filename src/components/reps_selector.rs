use crate::constants::{REPETITIONS_MAXIMUM, REPETITIONS_MINIMUM, REPETITIONS_STEP};
use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn RepsSelector() -> impl IntoView {
    let (value, set_value) = signal(REPETITIONS_MINIMUM);

    view! {
        <span class="count-selector">
            <span>{move || format!(" {} reps", value.get())}</span>

            <input
                type="range"
                min=REPETITIONS_MINIMUM
                max=REPETITIONS_MAXIMUM
                step=REPETITIONS_STEP
                value=move || value.get()
                on:input=move |event| {
                    let v = event_target_value(&event).parse::<u8>().unwrap_or(REPETITIONS_MINIMUM);
                    set_value.set(v);
                }
            />
        </span>
    }
}
