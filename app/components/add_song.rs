use dioxus::prelude::*;

use super::icons::{Icon, IconKind};
use super::mascot::MascotPill;
use super::Route;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SongEntryMode {
    Spotify,
    Local,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalEntryMode {
    SingleSong,
    Directory,
}

#[component]
pub fn AddSongScreen(initial_mode: SongEntryMode) -> Element {
    let mut mode = use_signal(|| initial_mode);
    let mut local_mode = use_signal(|| LocalEntryMode::SingleSong);
    let mut spotify_link = use_signal(String::new);
    let mut local_description = use_signal(String::new);
    let mut local_lyrics = use_signal(String::new);
    let mut local_directory = use_signal(String::new);
    let mut staged_notice = use_signal(String::new);

    let active_mode = mode();
    let active_local_mode = local_mode();

    rsx! {
        main { class: "utago-app add-song-app",
            header { class: "topbar",
                Link { class: "back-button", to: Route::Home {},
                    Icon { kind: IconKind::ArrowLeft }
                    span { "Home" }
                }
                div { class: "topbar-actions",
                    Link { class: "topbar-link", to: Route::BrowseLibrary {},
                        Icon { kind: IconKind::Search }
                        span { "Browse" }
                    }
                    MascotPill {}
                }
            }

            section { class: "add-hero",
                p { class: "eyebrow", "add song" }
                h1 { "Add a source" }
                p { "Choose Spotify or local only after starting an add flow." }
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
                    Icon { kind: IconKind::Folder }
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
                        Link { class: "ghost-action", to: Route::BrowseLibrary {},
                            Icon { kind: IconKind::Search }
                            span { "Browse library" }
                        }
                    }
                } else {
                    div { class: "local-mode-tabs", aria_label: "local import mode",
                        button {
                            class: if active_local_mode == LocalEntryMode::SingleSong { "entry-tab entry-tab--active" } else { "entry-tab" },
                            onclick: move |_| local_mode.set(LocalEntryMode::SingleSong),
                            Icon { kind: IconKind::Music }
                            span { "Single" }
                        }
                        button {
                            class: if active_local_mode == LocalEntryMode::Directory { "entry-tab entry-tab--active" } else { "entry-tab" },
                            onclick: move |_| local_mode.set(LocalEntryMode::Directory),
                            Icon { kind: IconKind::Folder }
                            span { "Directory" }
                        }
                    }

                    if active_local_mode == LocalEntryMode::SingleSong {
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
                    } else {
                        label { class: "field-block",
                            span { "Directory path" }
                            input {
                                r#type: "text",
                                placeholder: "/Music/local-scene/mixtapes",
                                value: "{local_directory}",
                                oninput: move |event| local_directory.set(event.value()),
                            }
                        }
                        label { class: "field-block",
                            span { "Bulk import notes" }
                            textarea {
                                rows: "5",
                                placeholder: "Folder naming pattern, language, venue, city, or lyric file format...",
                                value: "{local_description}",
                                oninput: move |event| local_description.set(event.value()),
                            }
                        }
                        p { class: "directory-note",
                            "Bulk add is staged as a directory scan placeholder for audio files plus sidecar lyrics."
                        }
                        div { class: "form-row",
                            button {
                                class: "primary-action primary-action--accent",
                                onclick: move |_| {
                                    let draft = search::draft_local_directory(
                                        local_directory(),
                                        local_description(),
                                    );
                                    staged_notice.set(draft.staging_label());
                                },
                                Icon { kind: IconKind::Folder }
                                span { "Queue directory" }
                            }
                            Link { class: "ghost-action", to: Route::BrowseLibrary {},
                                Icon { kind: IconKind::Search }
                                span { "Browse library" }
                            }
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
