use leptos::prelude::*;

/// Fixed overlay with "stone" texture (noise) across the whole site.
#[component]
pub fn NoiseOverlay() -> impl IntoView {
    view! { <div class="dungeon-noise" aria-hidden="true"></div> }
}

/// Ornate divider with a central diamond.
#[component]
pub fn DividerOrnate() -> impl IntoView {
    view! {
        <div class="divider-ornate my-10" aria-hidden="true">
            <span class="gem"></span>
        </div>
    }
}

/// Ornate double-line frame with golden corners.
#[component]
pub fn OrnateFrame(children: Children) -> impl IntoView {
    view! {
        <div class="ornate-frame rounded-xl p-6 md:p-10">
            {children()}
        </div>
    }
}

/// Playing card back (burgundy damask), used for the deck.
#[component]
pub fn CardBack(
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let extra = format!("card-back {class}");
    view! { <div class=extra aria-hidden="true"></div> }
}

/// Empty board slot (armory / room) with optional label.
#[component]
pub fn CardSlot(
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let extra = format!("card-slot {class}");
    view! {
        <div class=extra>
            {label.map(|l| view! { <span class="slot-label">{l}</span> })}
        </div>
    }
}

/// Health bar with dynamic fill (0.0 - 100.0 percent).
#[component]
pub fn HpBar(percent: Signal<f64>) -> impl IntoView {
    view! {
        <div
            class="hp-bar"
            role="progressbar"
            aria-valuenow=move || percent.get().round() as i32
            aria-valuemin=0
            aria-valuemax=100
        >
            <div class="hp-fill" style=move || format!("width: {}%;", percent.get().clamp(0.0, 100.0))></div>
        </div>
    }
}

/// Badge / chip for skills and tags.
#[component]
pub fn Badge(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let extra = format!("badge {class}");
    view! { <span class=extra>{children()}</span> }
}
