# dioxus-helmet

<a href="https://crates.io/crates/dioxus-helmet">
  <img src="https://img.shields.io/crates/v/dioxus-helmet.svg?style=flat-square"
  alt="Crates.io version" />
</a>

## General

Inspired by react-helmet, this small [Dioxus](https://github.com/DioxusLabs/dioxus) component allows you to place elements in the **head** of your code.

## Configuration

Add the package as a dependency to your `Cargo.toml`.

```
cargo add dioxus-helmet
```

## Usage

Import it in your code:
```rust
use dioxus_helmet::Helmet;
```

Then use it as a component like this:

```rust
#[component]
fn HeadElements(path: String) -> Element {
    rsx! {
        Helmet {
            link { rel: "icon", href: "{path}"}
            title { "Helmet" }
            style {
                [r#"
                    body {
                        color: blue;
                    }
                    a {
                        color: red;
                    }
                "#]
            }
        }
    }
}
```

Any children passed to the helmet component will then be placed in the `<head></head>` of your document.

**IMPORTANT**: The nodes inside the `Helmet` component are not reactive, so they won't be updated
when the value of them changes. So it's better to use static values inside the `Helmet`
component instead of signals.

They will be visible while the component is rendered. Duplicates **won't** get appended multiple times.

## License

This project is licensed under the [MIT license](https://github.com/saicu/dioxus-helmet/blob/main/LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in dioxus-helmet by you, shall be licensed as MIT, without any additional terms or conditions.
