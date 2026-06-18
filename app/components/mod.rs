use dioxus::prelude::*;

pub mod add_song;
mod home;
mod icons;
mod library_browser;
mod mascot;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/library")]
    BrowseLibrary {},
    #[route("/add")]
    AddSong {},
    #[route("/add/local")]
    AddLocalSong {},
}

#[component]
pub fn Home() -> Element {
    rsx! {
        home::HomeScreen {}
    }
}

#[component]
pub fn BrowseLibrary() -> Element {
    rsx! {
        library_browser::LibraryScreen {}
    }
}

#[component]
pub fn AddSong() -> Element {
    rsx! {
        add_song::AddSongScreen { initial_mode: add_song::SongEntryMode::Spotify }
    }
}

#[component]
pub fn AddLocalSong() -> Element {
    rsx! {
        add_song::AddSongScreen { initial_mode: add_song::SongEntryMode::Local }
    }
}
