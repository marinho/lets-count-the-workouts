use crate::constants::{WEIGHT_MAXIMUM, WEIGHT_MINIMUM, WEIGHT_STEP};
use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn WeightSelector() -> impl IntoView {
    let (value, set_value) = signal(WEIGHT_MINIMUM);

    view! {
        <span class="weight-selector">
            <span>{move || format!(" {:.2} kg", value.get() as f32 / 1000.0)}</span>

            <input
                type="range"
                min=WEIGHT_MINIMUM
                max=WEIGHT_MAXIMUM
                step=WEIGHT_STEP
                value=move || value.get()
                on:input=move |event| {
                    let v = event_target_value(&event).parse::<u16>().unwrap_or(WEIGHT_MINIMUM);
                    set_value.set(v);
                }
            />
        </span>
    }
}
