use dioxus::prelude::*;

use super::icons::{Icon, IconKind};
use super::mascot::MascotPill;
use super::Route;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SongEntryMode {
    Spotify,
    Local,
}

#[component]
pub fn AddSongScreen(initial_mode: SongEntryMode) -> Element {
    let mut mode = use_signal(|| initial_mode);
    let mut spotify_link = use_signal(String::new);
    let mut local_description = use_signal(String::new);
    let mut local_lyrics = use_signal(String::new);
    let mut staged_notice = use_signal(String::new);

    let active_mode = mode();

    rsx! {
        main { class: "utago-app add-song-app",
            header { class: "topbar",
                Link { class: "back-button", to: Route::Home {},
                    Icon { kind: IconKind::ArrowLeft }
                    span { "Home" }
                }
                MascotPill {}
            }

            section { class: "add-hero",
                p { class: "eyebrow", "add song" }
                h1 { "Build the sing-along shelf" }
                p { "Spotify now. Local discovery next." }
            }

            section { class: "entry-tabs", aria_label: "song source",
                button {
                    class: if active_mode == SongEntryMode::Spotify { "entry-tab entry-tab--active" } else { "entry-tab" },
                    onclick: move |_| mode.set(SongEntryMode::Spotify),
                    Icon { kind: IconKind::Link }
                    span { "Spotify" }
                }
                button {
                    class: if active_mode == SongEntryMode::Local { "entry-tab entry-tab--active" } else { "entry-tab" },
                    onclick: move |_| mode.set(SongEntryMode::Local),
                    Icon { kind: IconKind::Search }
                    span { "Local" }
                }
            }

            section { class: "song-form",
                if active_mode == SongEntryMode::Spotify {
                    label { class: "field-block",
                        span { "Spotify link" }
                        input {
                            r#type: "url",
                            placeholder: "https://open.spotify.com/track/...",
                            value: "{spotify_link}",
                            oninput: move |event| spotify_link.set(event.value()),
                        }
                    }
                    div { class: "form-row",
                        button {
                            class: "primary-action",
                            onclick: move |_| {
                                let draft = library::SongDraft::from_spotify_link(spotify_link());
                                staged_notice.set(draft.staging_label());
                            },
                            Icon { kind: IconKind::Plus }
                            span { "Stage song" }
                        }
                        button {
                            class: "ghost-action",
                            disabled: true,
                            title: "Browse is planned",
                            Icon { kind: IconKind::Search }
                            span { "Browse" }
                        }
                    }
                } else {
                    label { class: "field-block",
                        span { "Song description" }
                        textarea {
                            rows: "5",
                            placeholder: "Scene, artist notes, venue, language, rough mood...",
                            value: "{local_description}",
                            oninput: move |event| local_description.set(event.value()),
                        }
                    }
                    label { class: "field-block",
                        span { "Lyrics" }
                        textarea {
                            rows: "9",
                            placeholder: "Paste lyrics here...",
                            value: "{local_lyrics}",
                            oninput: move |event| local_lyrics.set(event.value()),
                        }
                    }
                    div { class: "form-row",
                        button {
                            class: "primary-action primary-action--accent",
                            onclick: move |_| {
                                let draft = search::draft_local_song(
                                    local_description(),
                                    local_lyrics(),
                                );
                                staged_notice.set(draft.staging_label());
                            },
                            Icon { kind: IconKind::Spark }
                            span { "AI fill" }
                        }
                        Link { class: "ghost-action", to: Route::Home {},
                            Icon { kind: IconKind::Home }
                            span { "Done" }
                        }
                    }
                }

                if !staged_notice().is_empty() {
                    div { class: "staged-notice",
                        span { class: "capture-readout__dot" }
                        p { "{staged_notice}" }
                    }
                }
            }
        }
    }
}
