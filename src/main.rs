use dioxus::prelude::*;
use stable_hash::fast_stable_hash;
use std::fs::{self, read_to_string};
use std::path::PathBuf;
use std::str::FromStr;

use crate::components::button::{Button, ButtonVariant};
mod components;

fn main() {
    dioxus_desktop::launch::launch(app, vec![], vec![]);
}

fn app() -> Element {
    const _: Asset = asset!("/assets/dx-components-theme.css");
    let mut file_name = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut vibes = use_signal(|| Option::<String>::None);
    let vibes_display = vibes().map(|_| "inline").unwrap_or("none");
    let vibes_string = vibes()
        .map(|message| format!("Vibes: {message}"))
        .unwrap_or_else(String::new);
    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            onclick: move |_| {
                if let Some(text)= PathBuf::from_str(&file_name()).ok().and_then(|path| read_to_string(path).ok()){
                    content.set(text);
                }
            },
            "open"
        }
        Button {
            variant: ButtonVariant::Secondary,
            onclick: move |_|{
                if PathBuf::from_str(&file_name()).ok().and_then(|path| fs::write(path, &content()).ok()).is_some(){

                }
            },
            "save"
        }
        input {
            oninput: move |event| {
            file_name.set(event.value())
        }}
        Button {
            variant: ButtonVariant::Primary,
            onclick: move |_|{
                vibes.set(Some(complex_algorithm(content())));
            },
            "Get Vibes"
        }
        div {
            display: vibes_display,
            "{vibes_string}"
        }
        textarea {
            width: "100%",
            height: "50vh",
            resize: "none",
            value: content(),
            oninput: move |event| {
                content.set(event.value());
            }
        }
    }
}
const VIBES: &[&'static str] = &[
    "melancholy",
    "remorse",
    "happy",
    "zeal",
    "sensible",
    "thrill",
    "remor",
    "agitation",
    "ardor",
    "quaint",
    "disturbed",
    "despondent",
    "aura",
    "ambient",
    "impression",
    "mood",
    "quality",
    "palpation",
    "taction",
    "agape",
    "premonition",
    "apprehensive",
    "augury",
    "presentiment",
    "ambience",
    "evocation",
    "prenotion",
    "omen",
    "harbringer",
    "boding",
    "prognostication",
    "bodement",
    "hunch",
    "portent",
    "apprehensive",
    "misgiving",
];
// COPYRIGHT: Property of Vibe Vibe Corp™
fn complex_algorithm(content: String) -> String {
    let content = content.trim();
    if content.is_empty() {
        return "None Available.".to_string();
    }
    let index = (fast_stable_hash(&content) % (VIBES.len() as u128)) as usize;
    VIBES[index].to_string()
}
