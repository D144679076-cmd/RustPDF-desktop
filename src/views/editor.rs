//! Editor view — mounts the shared [`EditorShell`] from pdf-app-core.
//!
//! All the layout (top bar, rails, page canvas, Tools panel) is shared; this
//! view just supplies the open document's metadata and binds desktop concerns
//! (menu actions, shortcuts) to the shell.

use dioxus::prelude::*;

use pdf_app_core::components::EditorShell;

use crate::AppState;

/// The full editor, shown once a document is open.
#[component]
pub fn Editor() -> Element {
    let state = use_context::<AppState>();
    // Title and page count come from the open Session (wired in D0). Placeholder
    // values here keep the scaffold rendering before the session is connected.
    rsx! {
        EditorShell {
            title: "Untitled.pdf".to_string(),
            page_count: 0,
            mode: (state.mode)(),
        }
    }
}
