use leptos::{prelude::*, text_prop::TextProp};
use leptos_bulma::elements::BProgress;

#[component]
pub fn BasicProgress() -> impl IntoView {
    view! {
        <BProgress max=100 value=25>
            "25%"
        </BProgress>
    }
}
