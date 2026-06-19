//! Welcome / empty state — the landing screen when no document is open.
//!
//! Centered on `--paper` with a serif title, a dashed drop zone, a coral
//! "Choose File" action, and (later) recent-file cards. See DESIGN.md §6.

use dioxus::prelude::*;

/// The welcome screen.
#[component]
pub fn Welcome() -> Element {
    rsx! {
        div { class: "welcome",
            div { class: "welcome-card",
                h1 { class: "welcome-title", "Open a PDF to get started" }
                p { class: "welcome-sub", "Edit text, annotate, fill forms, and sign — all offline." }
                div { class: "dropzone",
                    p { "Drag a PDF here" }
                    button { class: "btn btn-primary",
                        // Opens the native file dialog (platform::dialog::open_pdf),
                        // parses into a Session, then routes to the editor (D0).
                        "Choose File"
                    }
                }
            }
        }
    }
}
