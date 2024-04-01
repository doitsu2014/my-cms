#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    #[redirect("/:..segments", |segments: Vec<String>| Route::Blogs {})]
    Blogs {},
    #[route("/blogs/:id")]
    Blog { id: i32 },
    #[route("/blobs")]
    Blobs {},
    #[route("/blobs/:id")]
    Blob { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    launch(|| rsx! {Router::<Route>{}})
}

#[component]
fn NavBar() -> Element {
    // Identify which page is active

    rsx! {
        nav {
            class: "bg-cyan-400 py-4 shadow-md flex",
            div {
                class: "container mx-auto flex justify-between items-center",
                Logo {}
                div { class: "flex space-x-4",
                    Link {
                        to: Route::Blogs {},
                        class: "text-white hover:text-gray-200 transition duration-300 border-b-2 border-transparent hover:border-white pb-1",
                        "Blogs"
                    }
                    Link {
                        to: Route::Blobs {},
                        class: "text-white hover:text-gray-200 transition duration-300 border-b-2 border-transparent hover:border-white pb-1",
                        "Blobs"
                    }
                }
                div {
                    class: "flex items-center",
                    input {
                        r#type: "text",
                        placeholder: "Search...",
                        class: "px-4 py-2 rounded-md bg-gray-200 text-gray-800 focus:outline-none focus:bg-gray-100 transition duration-300"
                    }
                    button {
                        class: "bg-gray-200 text-gray-800 px-4 py-2 ml-2 rounded-md hover:bg-gray-300 transition duration-300",
                        "Search"
                    }
                }
            }
        }
    }
}

#[component]
fn MainLayout() -> Element {
    rsx! {
        NavBar {}

        Outlet::<Route> {}

        div {
            "Footer"
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Blogs {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Blogs() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { class: "text-green", "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        log::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[component]
fn Blobs() -> Element {
    rsx! {
        "Blobs"
    }
}

#[component]
fn Blob(id: i32) -> Element {
    rsx! {
        "Blob {id}"
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

#[component]
fn Logo() -> Element {
    rsx! {
        div { class: "flex p-2",
            div { class: "logo bg-black rounded-lg p-3 shadow-md",
                span { class: "font-bold text-xl text-white", "d" }
                span { class: "font-bold text-xl text-cyan-400", " tech" }
            }
        }
    }
}

#[component]
fn LogoV2() -> Element {
    rsx! {
        div { class: "flex p-2",
            div { class: "logo bg-cyan-400 rounded-lg p-3 shadow-md",
                span { class: "font-bold text-xl text-white", "d" }
                span { class: "font-bold text-xl text-black", " tech" }
            }
        }
    }
}
