use std::{cell::Cell, rc::Rc};

use crate::use_route;
use leptos::*;

/// Displays the child route nested in a parent route, allowing you to control exactly where
/// that child route is displayed. Renders nothing if there is no nested child.
#[component]
pub fn Outlet(cx: Scope) -> impl IntoView {
    let route = use_route(cx);
    let is_showing = Rc::new(Cell::new(None));
    let (outlet, set_outlet) = create_signal(cx, None);
    create_effect(cx, move |_| {
        match (route.child(), &is_showing.get()) {
            (None, _) => {
                set_outlet.set(None);
            }
            (Some(child), Some(is_showing_val)) if child.id() == *is_showing_val => {
                // do nothing: we don't need to rerender the component, because it's the same
            }
            (Some(child), _) => {
                is_showing.set(Some(child.id()));
                provide_context(child.cx(), child.clone());
                set_outlet.set(Some(child.outlet().into_view(cx)))
            }
        }
    });

    move || outlet.get()
}
