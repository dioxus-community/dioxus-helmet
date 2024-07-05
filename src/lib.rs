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
use dioxus_core::AttributeValue;
use lazy_static::lazy_static;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

lazy_static! {
    static ref INIT_CACHE: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

#[allow(non_snake_case)]
#[component]
pub fn Helmet(children: Element) -> Element {
    use_hook_with_cleanup(move || {
        let document = web_sys::window()?.document()?;
        let head = document.head()?;
        let element_maps = extract_element_maps(&children)?;
        let mut init_cache = INIT_CACHE.try_lock().ok()?;

        element_maps.iter().for_each(|element_map| {
            let mut hasher = FxHasher::default();
            element_map.hash(&mut hasher);
            let hash = hasher.finish();

            if init_cache.contains(&hash) { return; }
            init_cache.push(hash);

            if let Some(new_element) = element_map.try_into_element(&document, &hash) {
                let _ = head.append_child(&new_element);
            }
        });

        Some(element_maps)
    },
    move |element_maps| {
        let Some(element_maps) = element_maps else { return; };
        let Some(window) = web_sys::window() else { return; };
        let Some(document) = window.document() else { return; };
        let Ok(mut init_cache) = INIT_CACHE.try_lock() else { return; };

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
    });

    None
}

#[derive(Debug, Hash, Clone)]
struct ElementMap {
    tag: &'static str,
    attributes: Vec<(&'static str, String)>,
    inner_html: Option<&'static str>,
}

impl ElementMap {
    fn try_into_element(
        &self,
        document: &web_sys::Document,
        hash: &u64,
    ) -> Option<web_sys::Element> {
        let new_element = document.create_element(self.tag).ok()?;

        self.attributes.iter().try_for_each(|(name, value)| {
            new_element.set_attribute(name, value)
        }).ok()?;
        new_element.set_attribute("data-helmet-id", &hash.to_string()).ok()?;

        if let Some(inner_html) = self.inner_html {
            new_element.set_inner_html(inner_html);
        }

        Some(new_element)
    }
}

fn extract_element_maps(children: &Element) -> Option<Vec<ElementMap>> {
    use AttributeValue as AV;
    use TemplateAttribute as TA;
    use TemplateNode as TN;

    let vnode = children.as_ref()?;
    let template = vnode.template.get();

    let elements = template.roots.iter()
        .filter_map(|root| {
            let TN::Element { tag, attrs, children, .. } = root else {
                return None;
            };

            let mut attributes = vec![];
            attrs.iter()
                .for_each(|attr| match attr {
                    TA::Static { name, value, .. } => attributes.push((*name, value.to_string())),
                    TA::Dynamic { id } => vnode.dynamic_attrs[*id].iter().for_each(|attr| {
                        match &attr.value {
                            AV::Bool(v) => attributes.push((attr.name, v.to_string())),
                            AV::Float(v) => attributes.push((attr.name, v.to_string())),
                            AV::Int(v) => attributes.push((attr.name, v.to_string())),
                            AV::Text(v) => attributes.push((attr.name, v.to_string())),
                            AV::None | AV::Listener(_) | AV::Any(_) => {}
                        }
                    })
                });

            let inner_html = match children.first() {
                Some(TN::Text { text }) => Some(*text),
                Some(TN::Element { children, .. }) if children.len() == 1 => {
                    match children.first() {
                        Some(TN::Text { text }) => Some(*text),
                        _ => None,
                    }
                }
                _ => None,
            };

            Some(ElementMap {
                tag,
                attributes,
                inner_html
            })
        })
        .collect();

    Some(elements)
}
