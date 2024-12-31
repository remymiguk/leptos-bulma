use leptos::{prelude::*, text_prop::TextProp};

#[component]
pub fn BTextarea(
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let mut b_textarea = view! {
        <textarea class=format!("textarea {}", class.unwrap_or_default()) id=id name=name placeholder=placeholder prop:value=value/>
    };

    for (attr_name, attr_value) in attributes {
        b_textarea = b_textarea.attr(attr_name, attr_value);
    }

    b_textarea
}
