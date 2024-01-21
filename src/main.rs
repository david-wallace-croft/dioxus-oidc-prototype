use dioxus::prelude::*;

fn main() {
  dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
  cx.render(rsx! {
    div {
      "Hello, World!"
    }
  })
}
