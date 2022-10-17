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
//!
//! Then use it as a component like this:
//!
//! ```rust
//! #[inline_props]
//! fn HeadElements(cx: Scope, path: String) -> Element {
//!     cx.render(rsx! {
//!         Helmet {
//!             link { rel: "icon", href: "{path}"}
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
//!         }
//!     })
//! }
//! ```
//!
//! Reach your dynamic values down as owned properties (eg `String` and **not** `&'a str`).
//!
//! Also make sure that there are **no states** in your component where you use Helmet.
//!
//! Any children passed to the helmet component will then be placed in the `<head></head>` of your document.
//!
//! They will be visible while the component is rendered. Duplicates **won't** get appended multiple times.

use dioxus::prelude::*;
use lazy_static::lazy_static;
use rustc_hash::FxHasher;
use std::{
    hash::{Hash, Hasher},
    sync::Mutex,
};

lazy_static! {
    static ref INIT_CACHE: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

#[derive(Props)]
pub struct HelmetProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Helmet<'a>(cx: Scope<'a, HelmetProps<'a>>) -> Element {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                if let Some(element_maps) = extract_element_maps(&cx.props.children) {
                    if let Ok(mut init_cache) = INIT_CACHE.try_lock() {
                        element_maps.iter().for_each(|element_map| {
                            let mut hasher = FxHasher::default();
                            element_map.hash(&mut hasher);
                            let hash = hasher.finish();

                            if !init_cache.contains(&hash) {
                                init_cache.push(hash);

                                if let Some(new_element) =
                                    element_map.try_into_element(&document, &hash)
                                {
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

impl Drop for HelmetProps<'_> {
    fn drop(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element_maps) = extract_element_maps(&self.children) {
                    if let Ok(mut init_cache) = INIT_CACHE.try_lock() {
                        element_maps.iter().for_each(|element_map| {
                            let mut hasher = FxHasher::default();
                            element_map.hash(&mut hasher);
                            let hash = hasher.finish();

                            if let Some(index) = init_cache.iter().position(|&c| c == hash) {
                                init_cache.remove(index);
                            }

                            if let Ok(children) =
                                document.query_selector_all(&format!("[data-helmet-id='{hash}']"))
                            {
                                if let Ok(Some(children_iter)) = js_sys::try_iter(&children) {
                                    children_iter.for_each(|child| {
                                        if let Ok(child) = child {
                                            let el = web_sys::Element::from(child);
                                            el.remove();
                                        };
                                    });
                                }
                            }
                        });
                    }
                }
            }
        }
    }
}

#[derive(Debug, Hash)]
struct ElementMap<'a> {
    tag: &'a str,
    attributes: Vec<(&'a str, &'a str)>,
    inner_html: Option<&'a str>,
}

impl<'a> ElementMap<'a> {
    fn try_into_element(
        &self,
        document: &web_sys::Document,
        hash: &u64,
    ) -> Option<web_sys::Element> {
        if let Ok(new_element) = document.create_element(self.tag) {
            self.attributes.iter().for_each(|(name, value)| {
                let _ = new_element.set_attribute(name, value);
            });
            let _ = new_element.set_attribute("data-helmet-id", &hash.to_string());

            if let Some(inner_html) = self.inner_html {
                new_element.set_inner_html(inner_html);
            }

            Some(new_element)
        } else {
            None
        }
    }
}

fn extract_element_maps<'a>(children: &'a Element) -> Option<Vec<ElementMap<'a>>> {
    if let Some(VNode::Fragment(fragment)) = &children {
        let elements = fragment
            .children
            .iter()
            .flat_map(|child| {
                if let VNode::Element(element) = child {
                    let attributes = element
                        .attributes
                        .iter()
                        .map(|attribute| {
                            (attribute.attribute.name, attribute.value.as_text().unwrap())
                        })
                        .collect();

                    let inner_html = match element.children.first() {
                        Some(VNode::Text(vtext)) => Some(vtext.text),
                        Some(VNode::Fragment(fragment)) if fragment.children.len() == 1 => {
                            if let Some(VNode::Text(vtext)) = fragment.children.first() {
                                Some(vtext.text)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    Some(ElementMap {
                        tag: element.tag,
                        attributes,
                        inner_html,
                    })
                } else {
                    None
                }
            })
            .collect();

        Some(elements)
    } else {
        None
    }
}
