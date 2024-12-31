use leptos::{prelude::*, text_prop::TextProp};

use super::{BControl, BField, BHelp, BLabel};

#[component]
pub fn BTextareaField(
    #[prop(optional, into)] error: MaybeProp<Option<String>>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: MaybeProp<String>,
) -> impl IntoView {
    let error_text = create_rw_signal(None);

    create_effect(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let textarea_class = move || {
        if has_error() {
            "textarea is-danger"
        } else {
            "textarea"
        }
    };

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BControl class="is-expanded">
                <textarea class=textarea_class id=id name=name placeholder=placeholder prop:value=value></textarea>
            </BControl>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}
