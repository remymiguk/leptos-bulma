use leptos::*;
use leptos_bulma::elements::BBox;

#[component]
pub fn BasicBox() -> impl IntoView {
    view! { <BBox class="has-text-centered m-4">"Hello, World!"</BBox> }
}
