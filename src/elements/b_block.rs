use leptos::{prelude::*, text_prop::TextProp};

#[component]
pub fn BBlock(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("block {}", class.get())>{children()}</div> }
}
