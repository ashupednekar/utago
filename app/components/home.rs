use audio::AudioBytes;
use dioxus::prelude::*;
use inference::RecognitionMatch;

use super::icons::{Icon, IconKind};
use super::mascot::{MascotHero, MascotPill};
use super::Route;

#[derive(Clone, PartialEq)]
enum ListenState {
    Idle,
    Matched(RecognitionMatch),
}

#[component]
pub fn HomeScreen() -> Element {
    let mut listen_state = use_signal(|| ListenState::Idle);
    let state = listen_state();
    let layout_class = if matches!(&state, ListenState::Matched(_)) {
        "home-layout home-layout--matched"
    } else {
        "home-layout"
    };

    let (status, detail, byte_count) = match &state {
        ListenState::Idle => ("Hum along", "Ready to drop into the lyric", 0usize),
        ListenState::Matched(match_info) => (
            "Matched",
            "Karaoke handoff staged",
            match_info.received_bytes,
        ),
    };

    rsx! {
        main { class: "utago-app utago-app--home",
            header { class: "topbar",
                MascotPill {}
                div { class: "topbar-actions",
                    Link {
                        class: "topbar-link",
                        to: Route::BrowseLibrary {},
                        Icon { kind: IconKind::Search }
                        span { "Browse" }
                    }
                    Link {
                        class: "icon-button",
                        to: Route::AddSong {},
                        title: "Add song",
                        Icon { kind: IconKind::Plus }
                    }
                }
            }

            div { class: "{layout_class}",
                section { class: "home-main",
                    div { class: "hero-zone",
                        MascotHero {}
                        div { class: "hero-copy",
                            p { class: "eyebrow", "sing search" }
                            h1 { "Utago" }
                            p { "Find the song, land on the line." }
                        }

                        button {
                            class: "hum-button",
                            onclick: move |_| {
                                let sample = AudioBytes::placeholder_hum_phrase();
                                let match_info = inference::handle_audio_bytes(sample);
                                listen_state.set(ListenState::Matched(match_info));
                            },
                            div { class: "hum-button__rings",
                                span {}
                                span {}
                                span {}
                            }
                            div { class: "hum-button__icon",
                                Icon { kind: IconKind::Mic }
                            }
                            span { class: "hum-button__label", "{status}" }
                            span { class: "hum-button__detail", "{detail}" }
                        }

                        div { class: "capture-readout",
                            span { class: "capture-readout__dot" }
                            span { "{byte_count} bytes" }
                            span { "inference::handle_audio_bytes" }
                        }
                    }

                    nav { class: "quick-actions", aria_label: "primary actions",
                        Link { class: "quick-action quick-action--accent", to: Route::BrowseLibrary {},
                            Icon { kind: IconKind::Search }
                            span { "Browse library" }
                        }
                        Link { class: "quick-action", to: Route::AddSong {},
                            Icon { kind: IconKind::Plus }
                            span { "Add song" }
                        }
                    }

                    if matches!(&state, ListenState::Idle) {
                        ListeningDeck {}
                    }
                }

                if let ListenState::Matched(match_info) = state.clone() {
                    aside { class: "home-side",
                        KaraokeStage { match_info }
                    }
                }
            }
        }
    }
}

#[component]
fn ListeningDeck() -> Element {
    rsx! {
        section { class: "listening-deck",
            div { class: "deck-line" }
            div { class: "deck-copy",
                Icon { kind: IconKind::Play }
                div {
                    h2 { "Timestamp lane" }
                    p { "Waiting for a match." }
                }
            }
        }
    }
}

#[component]
fn KaraokeStage(match_info: RecognitionMatch) -> Element {
    let song = library::active_song();
    let active_index = active_lyric_index(&song.lyrics, match_info.timestamp_ms);
    let progress =
        (match_info.timestamp_ms as f32 / song.duration_ms as f32 * 100.0).clamp(0.0, 100.0);

    let mut lyric_rows = Vec::new();
    for (index, line) in song.lyrics.iter().enumerate() {
        lyric_rows.push((
            format_timestamp(line.timestamp_ms),
            line.text.clone(),
            index == active_index,
        ));
    }

    rsx! {
        section { class: "karaoke-stage",
            div { class: "karaoke-stage__top",
                div {
                    p { class: "eyebrow", "{match_info.confidence_percent}% confidence" }
                    h2 { "{match_info.title}" }
                    p { "{match_info.artist}" }
                }
                div { class: "match-chip", "{format_timestamp(match_info.timestamp_ms)}" }
            }

            div { class: "playhead",
                div {
                    class: "playhead__fill",
                    style: "width: {progress}%;",
                }
            }

            div { class: "lyrics-stack",
                for (time, text, is_active) in lyric_rows {
                    p {
                        class: if is_active { "lyric-line lyric-line--active" } else { "lyric-line" },
                        span { "{time}" }
                        b { "{text}" }
                    }
                }
            }
        }
    }
}

fn active_lyric_index(lines: &[library::LyricLine], timestamp_ms: u32) -> usize {
    let mut active = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.timestamp_ms <= timestamp_ms {
            active = index;
        }
    }
    active
}

fn format_timestamp(timestamp_ms: u32) -> String {
    let total_seconds = timestamp_ms / 1000;
    format!("{:02}:{:02}", total_seconds / 60, total_seconds % 60)
}
