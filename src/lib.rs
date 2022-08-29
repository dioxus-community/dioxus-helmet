//! ## General
//! Inspired by react-helmet, this small [Dioxus](https://crates.io/crates/dioxus) component allows you to place elements in the **head** of your code.
//! ## Configuration
//! Add the package as a dependency to your `Cargo.toml`.
//! ```no_run
//! dioxus-helmet = "0.1.3"
//! ```
//! ## Usage
//! Import it in your code:
//! ```
//! use dioxus_helmet::Helmet;
//! ```
//! Then just use it anywhere in your components like this:
//! ```
//! cx.render(rsx! {
//!     div {
//!         Helmet {
//!             link { rel: "stylesheet", href: "/style.css" }
//!             title { "Helmet" }
//!             style {
//!                 [r#"
//!                     body {
//!                         color: blue;
//!                     }
//!                     a {
//!                         color: red;
//!                     }
//!                 "#]
//!             }
//!         },
//!         p { "Hello, world!" }
//!     }
//! })
//! ```
//! Any children passed to the helmet component will be placed in the `<head></head>` of your document.

use dioxus::prelude::*;

#[derive(Props)]
pub struct HelmetProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Helmet<'a>(cx: Scope<'a, HelmetProps<'a>>) -> Element {
    let initialized = use_state(&cx, || false);

    if !*initialized.get() {
        initialized.set(true);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(head) = document.head() {
                    if let Some(new_children) = extract_elements(&document, &cx.props.children) {
                        new_children.iter().for_each(|child| {
                            let _ = head.append_child(child);
                        });
                    }
                }
            }
        }
    }

    None
}

fn extract_elements<'a>(
    document: &web_sys::Document,
    children: &Element<'a>,
) -> Option<Vec<web_sys::Element>> {
    if let Some(VNode::Fragment(fragment)) = &children {
        let elements = fragment
            .children
            .iter()
            .flat_map(|child| {
                if let VNode::Element(element) = child {
                    if let Ok(new_element) = document.create_element(element.tag) {
                        element.attributes.iter().for_each(|attribute| {
                            let name = attribute.name;
                            let value = attribute.value;
                            let _ = new_element.set_attribute(name, value);
                        });

                        match element.children.first() {
                            Some(VNode::Text(text)) => {
                                new_element.set_text_content(Some(text.text));
                            }
                            Some(VNode::Fragment(fragment)) if fragment.children.len() == 1 => {
                                if let Some(VNode::Text(text)) = fragment.children.first() {
                                    let inner = text.text.replace("}\n", "} ").replace('\n', "");

                                    new_element.set_inner_html(&inner);
                                };
                            }
                            _ => {}
                        };

                        return Some(new_element);
                    }
                }
                None
            })
            .collect();

        return Some(elements);
    }

    None
}
