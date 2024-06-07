//! ## General
//! Inspired by react-dioxus-helmet, this small [Dioxus](https://crates.io/crates/dioxus) component allows you to place elements in the **head** of your code.
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
//! #[component]
//! fn HeadElements(path: String) -> Element {
//!     rsx! {
//!         Helmet {
//!             link { rel: "icon", href: "{path}"}
//!             title { "Helmet" }
//!             style {
//!                 r"
//!                       body {{
//!                           font-size: 22px;
//!                           margin: 0;
//!                           color: white;
//!                           text-align: center;
//!                       }}
//!                   "
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Reach your dynamic values down as owned properties (eg `String` and **not** `&'a str`).
//!
//! Also make sure that there are **no states** in your component where you use Helmet.
//!
//! Any children passed to the `Helmet` component will then be placed in the `<head></head>` of your document.
//!
//! They will be visible while the component is rendered. Duplicates **won't** get appended multiple times.
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use lazy_static::lazy_static;
use rustc_hash::FxHasher;
use std::{
    hash::{Hash, Hasher},
    sync::Mutex,
};

lazy_static! {
    static ref INIT_CACHE: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

#[derive(Props, Clone, PartialEq)]
pub struct HelmetProps {
    children: Element,
}

pub fn Helmet(props: HelmetProps) -> Element {
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(head) = document.head() {
                    if let Some(element_maps) = extract_element_maps(&props.children) {
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
    });

    None
}

#[derive(Debug, Hash)]
struct ElementMap<'a> {
    tag: &'a str,
    attributes: Vec<(&'a str, String)>,
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
            let _ = new_element.set_attribute("data-dioxus-helmet-id", &hash.to_string());

            if let Some(inner_html) = self.inner_html {
                new_element.set_inner_html(inner_html);
            }

            Some(new_element)
        } else {
            None
        }
    }
}

fn extract_element_maps(children: &Element) -> Option<Vec<ElementMap>> {
    if let Some(vnode) = &children {
        let elements: Vec<ElementMap> = vnode
            .template
            .get()
            .roots
            .iter()
            .flat_map(|root: &TemplateNode| {
                if let TemplateNode::Element {
                    tag,
                    attrs,
                    children,
                    ..
                } = root
                {
                    let mut attributes: Vec<(&str, String)> = vnode
                        .dynamic_attrs
                        .iter()
                        .flat_map(|attrs| {
                            attrs
                                .iter()
                                .filter_map(|attr| match &attr.value {
                                    AttributeValue::None => None,
                                    AttributeValue::Listener(_) => None,
                                    AttributeValue::Any(_) => None,
                                    AttributeValue::Bool(value) => {
                                        Some((attr.name, value.to_string()))
                                    }
                                    AttributeValue::Float(value) => {
                                        Some((attr.name, value.to_string()))
                                    }
                                    AttributeValue::Int(value) => {
                                        Some((attr.name, value.to_string()))
                                    }
                                    AttributeValue::Text(value) => Some((attr.name, value.clone())),
                                })
                                .collect::<Vec<(&str, String)>>()
                        })
                        .collect::<Vec<(&str, String)>>();

                    let mut template_attributes: Vec<(&str, String)> = attrs
                        .iter()
                        .flat_map(|attribute| {
                            if let TemplateAttribute::Static { name, value, .. } = attribute {
                                Some((*name, value.to_string()))
                            } else {
                                None
                            }
                        })
                        .collect();

                    attributes.append(&mut template_attributes);

                    let inner_html = match children.first() {
                        Some(TemplateNode::Text { text }) => Some(*text),
                        Some(TemplateNode::Element { children, .. }) if children.len() == 1 => {
                            if let Some(TemplateNode::Text { text }) = children.first() {
                                Some(*text)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    Some(ElementMap {
                        tag,
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
