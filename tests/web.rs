// NOTE:
// This doesn't work, the javascript used to create an element isn't working when accessing in rust. 
// eval("console.log(document.head.innerHTML)") gives the expected result, while
// head.inner_html() doesnt show any changes from the js. 

// #![cfg(target_arch = "wasm32")]

// use dioxus::prelude::*;
// use dioxus::web::use_eval;
// use wasm_bindgen_test::*;
// use dioxus_helmet::Helmet;

// wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// fn it_works() {
//     fn main() {

//         dioxus::web::launch(APP);
//     }

//     static APP: Component = |cx| {
//         let eval = use_eval(&cx);
//         eval("let el = document.createElement('title'); el.innerText = 'test'; document.head.appendChild(el); console.log(document.head.innerHTML);");

//         cx.render(rsx! {
//             div {
//                 h1 { "Hi" }
//             }
//         })
//     };

    

//     main();

//     let head = gloo_utils::head();
//     assert_eq!(head.inner_html(), "");
// }
