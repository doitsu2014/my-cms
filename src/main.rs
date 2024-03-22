#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// You can also collect google fonts
const ROBOTO_FONT: &str = manganis::mg!(font().families(["Roboto"]));
const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(app)
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    rsx! { AppHeader {} }
}

fn AppHeader() -> Element {
    const RESIZED_PNG_ASSET: manganis::ImageAsset =
        manganis::mg!(image("public/images/Doitsu-Tech-Logo-Square.png").size(68, 68));
    rsx! {
        header { class: "flex p-4",
            a { href: "/",
                img {
                    src: RESIZED_PNG_ASSET,
                    referrerpolicy: "no-referrer",
                    alt: "hero",
                    class: "rounded-lg"
                }
            }
            a { p {
                class: "flex px-2 my-auto", "Home 1"
            }
        }
        }
    }
}
