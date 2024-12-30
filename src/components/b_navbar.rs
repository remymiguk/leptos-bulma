use leptos::html::{Div, A};
use leptos::{prelude::*, text_prop::TextProp};

#[component]
pub fn BNavbar(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <nav class=format!("navbar {}", class.get())>{children()}</nav> }
}

#[component]
pub fn BNavbarBrand(children: Children) -> impl IntoView {
    view! { <nav class="navbar-brand">{children()}</nav> }
}

#[component]
pub fn BNavbarBurger(is_active: RwSignal<bool>) -> impl IntoView {
    let burger_ref = create_node_ref::<A>();

    let _ = leptos_use::on_click_outside(burger_ref, move |_| {
        is_active.set(false);
    });

    view! {
        <a
            node_ref=burger_ref
            class="navbar-burger"
            class:is-active=is_active
            on:click=move |_| { is_active.update(|v| *v = !*v) }
        >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    }
}

#[component]
pub fn BNavbarDivider() -> impl IntoView {
    view! { <hr class="navbar-divider"/> }
}

#[component]
pub fn BNavbarEnd(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("navbar-end {}", class.get())>{children()}</div> }
}

#[component]
pub fn BNavbarItem(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] href: Option<TextProp>,
    #[prop(optional, into)] target: Option<TextProp>,
    #[prop(optional, into)] title: Option<TextProp>,
) -> impl IntoView {
    view! {
        <a class=format!("navbar-item {}", class.get()) href=href target=target title=title>
            {children()}
        </a>
    }
}

#[component]
pub fn BNavbarItemDropdown<F, IV>(
    children: Children,
    #[prop(optional, into)] dropdown_class: TextProp,
    #[prop(optional, into)] href: Option<TextProp>,
    #[prop(optional, into)] is_hoverable: MaybeSignal<bool>,
    trigger: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let node_ref = create_node_ref::<Div>();
    let is_active = create_rw_signal(false);

    let _ = leptos_use::on_click_outside(node_ref, move |_| {
        is_active.set(false);
    });

    let toggle_dropdown = move |_| {
        if is_hoverable.get() {
            is_active.set(false);
        } else {
            is_active.update(|value| *value = !*value);
        }
    };

    view! {
        <div
            node_ref=node_ref
            class="navbar-item has-dropdown"
            class:is-active=is_active
            class:is-hoverable=is_hoverable
            on:click=toggle_dropdown
        >
            <a class="navbar-link" href=href>
                {trigger()}
            </a>

            <div class=format!("navbar-dropdown {}", dropdown_class.get())>{children()}</div>
        </div>
    }
}

#[component]
pub fn BNavbarMenu(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] is_active: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <div
            class=format!("navbar-menu {}", class.get())
            class:is-active=move || is_active.map(|v| v.get()) == Some(true)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn BNavbarStart(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("navbar-start {}", class.get())>{children()}</div> }
}
