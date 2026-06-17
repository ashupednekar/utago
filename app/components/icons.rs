use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    ArrowLeft,
    Home,
    Link,
    Mic,
    Music,
    Play,
    Plus,
    Search,
    Spark,
    Wave,
}

#[component]
pub fn Icon(kind: IconKind) -> Element {
    match kind {
        IconKind::ArrowLeft => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M19 12H5" }
                path { d: "M12 19l-7-7 7-7" }
            }
        },
        IconKind::Home => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M3 11.5 12 4l9 7.5" }
                path { d: "M5.5 10.5V20h13v-9.5" }
                path { d: "M9.5 20v-5h5v5" }
            }
        },
        IconKind::Link => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M10 13a5 5 0 0 0 7.1 0l2-2a5 5 0 0 0-7.1-7.1l-1.2 1.2" }
                path { d: "M14 11a5 5 0 0 0-7.1 0l-2 2A5 5 0 0 0 12 20.1l1.2-1.2" }
            }
        },
        IconKind::Mic => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 3a3 3 0 0 0-3 3v6a3 3 0 0 0 6 0V6a3 3 0 0 0-3-3Z" }
                path { d: "M19 10v2a7 7 0 0 1-14 0v-2" }
                path { d: "M12 19v3" }
                path { d: "M8 22h8" }
            }
        },
        IconKind::Music => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M9 18V5l11-2v13" }
                path { d: "M9 9l11-2" }
                path { d: "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" }
                path { d: "M17 19a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" }
            }
        },
        IconKind::Play => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M8 5v14l11-7Z" }
            }
        },
        IconKind::Plus => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 5v14" }
                path { d: "M5 12h14" }
            }
        },
        IconKind::Search => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" }
                path { d: "m21 21-4.3-4.3" }
            }
        },
        IconKind::Spark => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 3l1.6 5.1L19 10l-5.4 1.9L12 17l-1.6-5.1L5 10l5.4-1.9L12 3Z" }
                path { d: "M19 15l.8 2.2L22 18l-2.2.8L19 21l-.8-2.2L16 18l2.2-.8L19 15Z" }
            }
        },
        IconKind::Wave => rsx! {
            svg {
                class: "ui-icon",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M4 12h2" }
                path { d: "M8 7v10" }
                path { d: "M12 4v16" }
                path { d: "M16 8v8" }
                path { d: "M20 11v2" }
            }
        },
    }
}
