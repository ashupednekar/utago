use dioxus::prelude::*;

use super::icons::{Icon, IconKind};
use super::mascot::MascotPill;
use super::Route;

#[component]
pub fn LibraryScreen() -> Element {
    rsx! {
        main { class: "utago-app browse-app",
            header { class: "topbar",
                Link { class: "back-button", to: Route::Home {},
                    Icon { kind: IconKind::ArrowLeft }
                    span { "Home" }
                }
                MascotPill {}
            }

            section { class: "add-hero browse-hero",
                p { class: "eyebrow", "library" }
                h1 { "Browse everything" }
                p { "Search saved tracks before adding a new source." }
            }

            LibraryBrowsePanel { compact: false }
        }
    }
}

#[component]
pub fn LibraryBrowsePanel(compact: bool) -> Element {
    let mut query = use_signal(String::new);
    let songs = library::featured_songs();
    let needle = query().trim().to_lowercase();

    let mut visible = Vec::new();
    for song in songs {
        let matches = needle.is_empty()
            || song.title.to_lowercase().contains(&needle)
            || song.artist.to_lowercase().contains(&needle);

        if matches {
            visible.push(song);
        }
    }

    rsx! {
        section {
            class: if compact { "browse-panel browse-panel--compact" } else { "browse-panel" },
            div { class: "section-heading section-heading--split",
                div { class: "section-heading__title",
                    Icon { kind: IconKind::Music }
                    h2 { "Browse library" }
                }
                Link { class: "text-action", to: Route::AddSong {},
                    Icon { kind: IconKind::Plus }
                    span { "Add" }
                }
            }

            label { class: "browse-search",
                Icon { kind: IconKind::Search }
                input {
                    r#type: "search",
                    placeholder: "Search songs or artists",
                    value: "{query}",
                    oninput: move |event| query.set(event.value()),
                }
            }

            div { class: "browse-list",
                if visible.is_empty() {
                    div { class: "browse-empty",
                        Icon { kind: IconKind::Music }
                        p { "No songs match that search." }
                    }
                }

                for song in visible {
                    article { class: "song-row song-row--browse",
                        div { class: "song-row__art",
                            Icon { kind: IconKind::Wave }
                        }
                        div { class: "song-row__meta",
                            h3 { "{song.title}" }
                            p { "{song.artist}" }
                        }
                        span { class: "song-row__duration", "{format_duration(song.duration_ms)}" }
                    }
                }
            }
        }
    }
}

fn format_duration(duration_ms: u32) -> String {
    let total_seconds = duration_ms / 1000;
    format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
}
