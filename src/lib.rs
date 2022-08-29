//! ## General
//! Inspired by react-helmet, this small [Dioxus](https://crates.io/crates/dioxus) component allows you to place elements in the **head** of your code.
//!
//! ## Configuration
//! Add the package as a dependency to your `Cargo.toml`.
//! ```no_run
//! cargo add dioxus-helmet
//! ```
//!
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

use lazy_static::lazy_static;
use std::sync::Mutex;

use dioxus::prelude::*;

#[derive(Props)]
pub struct HelmetProps<'a> {
    children: Element<'a>,
}

lazy_static! {
    static ref INIT_CACHE: Mutex<Vec<ElementMap>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementMap {
    tag: String,
    attributes: Vec<(String, String)>,
    inner_html: Option<String>,
}

impl ElementMap {
    fn try_into_element(&self, document: &web_sys::Document) -> Option<web_sys::Element> {
        if let Ok(new_element) = document.create_element(&self.tag) {
            self.attributes.iter().for_each(|(name, value)| {
                let _ = new_element.set_attribute(name, value);
            });

            if let Some(inner_html) = &self.inner_html {
                new_element.set_inner_html(inner_html);
            }

            return Some(new_element);
        }
        None
    }
}

#[allow(non_snake_case)]
pub fn Helmet<'a>(cx: Scope<'a, HelmetProps<'a>>) -> Element {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                if let Some(element_maps) = extract_element_maps(&cx.props.children) {
                    if let Ok(mut init_cache) = INIT_CACHE.try_lock() {
                        element_maps.iter().for_each(|element_map| {
                            if !init_cache.contains(element_map) {
                                init_cache.push(element_map.clone());

                                if let Some(new_element) = element_map.try_into_element(&document) {
                                    let _ = head.append_child(&new_element);
                                }
                            }
                        });
                    }
                }
            }
        }
    }

    None
}

fn extract_element_maps(children: &Element) -> Option<Vec<ElementMap>> {
    if let Some(VNode::Fragment(fragment)) = &children {
        let elements = fragment
            .children
            .iter()
            .flat_map(|child| {
                if let VNode::Element(element) = child {
                    let attributes = element
                        .attributes
                        .iter()
                        .map(|attribute| (attribute.name.to_owned(), attribute.value.to_owned()))
                        .collect();

                    let inner_html = match element.children.first() {
                        Some(VNode::Text(vtext)) => Some(vtext.text.to_owned()),
                        Some(VNode::Fragment(fragment)) if fragment.children.len() == 1 => {
                            if let Some(VNode::Text(vtext)) = fragment.children.first() {
                                Some(vtext.text.replace("}\n", "} ").replace('\n', ""))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    return Some(ElementMap {
                        tag: element.tag.to_owned(),
                        attributes,
                        inner_html,
                    });
                }

                None
            })
            .collect();

        return Some(elements);
    }

    None
}
