//! Welcome / empty state — the landing screen when no document is open.
//!
//! Centered on `--paper` with a serif title, a dashed drop zone, a coral
//! "Choose File" action, and (later) recent-file cards. See DESIGN.md §6.

use std::path::PathBuf;
use std::sync::Arc;

use dioxus::prelude::*;

use pdf_app_core::pdf_core::error::PdfError;
use pdf_app_core::pdf_core::parser::objects::PdfDocument;
use pdf_app_core::session::Session;

use crate::{platform, routes::Route, AppState};

/// The welcome screen.
#[component]
pub fn Welcome() -> Element {
    let state = use_context::<AppState>();
    let nav = use_navigator();

    // Password-dialog state — set when the opened file is encrypted.
    let mut pending: Signal<Option<(PathBuf, String)>> = use_signal(|| None);
    let mut password = use_signal(String::new);
    let mut pw_wrong = use_signal(|| false);
    let mut open_error: Signal<Option<String>> = use_signal(|| None);

    // Commit a successfully-opened session into global state and navigate.
    let commit = move |session: Session, bytes: Vec<u8>| {
        let page_count = session.page_count();
        let sizes: Vec<(f64, f64)> = (0..page_count)
            .map(|i| session.page_size(i).unwrap_or((595.0, 842.0)))
            .collect();
        // Publish the document to the process-global render-protocol slot so
        // pdfpage:// requests can render pages without holding a Dioxus borrow.
        if let Ok(doc) = PdfDocument::parse(bytes) {
            platform::render_protocol::set_active_document(Arc::new(doc));
        }
        state.page_sizes.set(sizes);
        state.session.set(Some(session));
        state.current_page.set(0);
        state.has_document.set(true);
        nav.push(Route::Editor {});
    };

    // Open a file path: read bytes, try Session::open, handle errors.
    let try_open = move |path: PathBuf| {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "document.pdf".to_string());

        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                open_error.set(Some(format!("Couldn't read file: {e}")));
                return;
            }
        };

        match Session::open(&name, bytes.clone()) {
            Ok(session) => commit(session, bytes),
            Err(PdfError::Encrypted { .. }) => {
                pending.set(Some((path, name)));
                pw_wrong.set(false);
                open_error.set(None);
            }
            Err(e) => {
                open_error.set(Some(format!("Couldn't open PDF: {e}")));
            }
        }
    };

    // Attempt to unlock an encrypted PDF with the entered password.
    let try_unlock = move || {
        let Some((path, name)) = (*pending.read()).clone() else {
            return;
        };
        let pw = password.read().as_bytes().to_vec();
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                open_error.set(Some(format!("Couldn't read file: {e}")));
                pending.set(None);
                return;
            }
        };
        match Session::open_with_password(&name, bytes.clone(), &pw) {
            Ok(session) => {
                pending.set(None);
                commit(session, bytes);
            }
            Err(PdfError::Encrypted { .. }) => {
                pw_wrong.set(true);
            }
            Err(e) => {
                open_error.set(Some(format!("Couldn't open PDF: {e}")));
                pending.set(None);
            }
        }
    };

    rsx! {
        div { class: "welcome",
            div { class: "welcome-card",
                h1 { class: "welcome-title", "Open a PDF to get started" }
                p { class: "welcome-sub",
                    "Edit text, annotate, fill forms, and sign — all offline."
                }
                div { class: "dropzone",
                    p { "Drag a PDF here" }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            if let Some(path) = platform::dialog::open_pdf() {
                                try_open(path);
                            }
                        },
                        "Choose File"
                    }
                }
                if let Some(err) = &*open_error.read() {
                    div { class: "toast toast-danger", "{err}" }
                }
            }

            // Password dialog — shown when opened file is encrypted.
            if pending.read().is_some() {
                div { class: "password-overlay",
                    div { class: "password-dialog",
                        h2 { class: "welcome-title", "Password required" }
                        p { class: "welcome-sub",
                            "This PDF is encrypted. Enter the password to open it."
                        }
                        input {
                            r#type: "password",
                            class: "pw-input",
                            placeholder: "Password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    try_unlock();
                                }
                            },
                        }
                        if *pw_wrong.read() {
                            p { class: "pw-error", "Incorrect password." }
                        }
                        div { class: "pw-actions",
                            button {
                                class: "btn btn-ghost",
                                onclick: move |_| {
                                    pending.set(None);
                                    password.set(String::new());
                                },
                                "Cancel"
                            }
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| try_unlock(),
                                "Unlock"
                            }
                        }
                    }
                }
            }
        }
    }
}
