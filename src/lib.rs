//! ## General
//! Inspired by react-helmet, this small [Dioxus](https://crates.io/crates/dioxus) component allows you to place elements in the **head** of your code.
//! ## Configuration
//! Add the package as a dependency to your `Cargo.toml`.
//! ### Web:
//! ```no_run
//! dioxus-helmet = "0.1.1"
//! ```
//! ### ~~Desktop:~~ (doesn't work yet)
//! ```
//! dioxus-helmet = { version = "0.1.1", default-features = false, features = ["desktop"] }
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
    #[cfg(feature = "web")]
    let eval = dioxus::web::use_eval(&cx);

    #[cfg(feature = "desktop")]
    let eval = dioxus::desktop::use_eval(&cx);

    if let Some(VNode::Fragment(fragment)) = &cx.props.children {
        fragment.children.iter().for_each(|child| {
            if let VNode::Element(element) = child {
                let tag = element.tag;
                let attributes: String = element
                    .attributes
                    .iter()
                    .map(|attribute| {
                        let name = attribute.name;
                        let value = attribute.value;
                        format!("el.setAttribute('{name}', '{value}');")
                    })
                    .collect();
                let children = &*element.children;

                let inner_text = match children.first() {
                    Some(VNode::Text(text)) => {
                        let text = text.text;
                        format!("el.innerText = '{text}'")
                    }
                    Some(VNode::Fragment(fragment)) if fragment.children.len() == 1 => fragment
                        .children
                        .first()
                        .and_then(|child| {
                            if let VNode::Text(text) = child {
                                let text = text.text.replace("}\n", "} ").replace('\n', "");
                                Some(format!("el.innerHTML = '{text}'"))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default(),
                    _ => "".to_owned(),
                };

                eval(format!(
                    r#"
                        let el = document.createElement('{tag}')
                        {attributes}
                        {inner_text}
                        document.head.appendChild(el)
                    "#
                ));
            }
        });
    }

    None
}
