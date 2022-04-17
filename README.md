# dioxus-helmet

Inspired by react-helmet, this small [Dioxus](https://github.com/DioxusLabs/dioxus) component allows you to place elements in the **head** of your code.

## Configuration

Add the package as a dependency to your `Cargo.toml`.

### Web:
```rust
dioxus-helmet = { git = "https://github.com/saicu/dioxus-helmet" }
```

### ~~Desktop:~~ (doesn't work yet)
```rust
dioxus-helmet = { git = "https://github.com/saicu/dioxus-helmet", default-features = false, features = ["desktop"] }
```

## Usage

Import it in your code 
```rust
use dioxus_helmet::Helmet;
```

Then you can just use it anywhere in your components like this:

```rust
    cx.render(rsx! {
        div {
            Helmet {
                link { rel: "stylesheet", href: "/style.css" }
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
            },
            p { "Hello, world!" }
        }
    })

```

Any children passed to the helmet component will be placed in the `<head></head>` of your document.

## License

This project is dual licensed under the MIT license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in dioxus-helmet by you, shall be licensed as MIT, without any additional terms or conditions.
