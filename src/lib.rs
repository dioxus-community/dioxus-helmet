use dioxus::prelude::*;

#[derive(Props)]
pub struct HelmetProps<'a> {
    children: Element<'a>,
}

/// Any children passed to the helmet component will be moved into the **head** area of your document.
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

                let inner_text = if children.len() == 1 {
                    match children[0] {
                        VNode::Text(text) => {
                            let text = text.text;
                            format!("el.innerText = '{text}'")
                        }
                        VNode::Fragment(fragment) if fragment.children.len() == 1 => {
                            if let VNode::Text(text) = fragment.children[0] {
                                let text = text.text.replace("}\n", "} ").replace('\n', "");
                                format!("el.innerHTML = '{text}'")
                            } else {
                                "".to_owned()
                            }
                        }
                        _ => "".to_owned(),
                    }
                } else {
                    "".to_owned()
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
