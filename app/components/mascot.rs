use dioxus::prelude::*;

const README_MASCOT_URL: &str =
    "https://github.com/user-attachments/assets/1bfd6531-2658-4e83-ba74-b7c9961f3d47";

#[component]
pub fn MascotHero() -> Element {
    rsx! {
        div { class: "mascot-hero",
            img {
                class: "mascot-image mascot-image--hero",
                src: README_MASCOT_URL,
                alt: "Utago mascot",
            }
            div { class: "mascot-hero__meter",
                span {}
                span {}
                span {}
                span {}
                span {}
            }
        }
    }
}

#[component]
pub fn MascotPill() -> Element {
    rsx! {
        div { class: "mascot-pill",
            img {
                class: "mascot-image mascot-image--pill",
                src: README_MASCOT_URL,
                alt: "",
            }
            span { "utago" }
        }
    }
}
