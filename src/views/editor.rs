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

    // Read session metadata. If somehow Editor is reached with no session
    // (shouldn't happen in normal flow), defaults keep the UI renderable.
    let session_ref = state.session.read();
    let title = session_ref
        .as_ref()
        .map(|s| s.display_name().to_string())
        .unwrap_or_else(|| "Untitled.pdf".to_string());
    let page_count = session_ref.as_ref().map(|s| s.page_count()).unwrap_or(0);
    let revision = session_ref.as_ref().map(|s| s.revision()).unwrap_or(0);
    drop(session_ref);

    rsx! {
        EditorShell {
            title,
            page_count,
            mode: (state.mode)(),
            viewport: state.viewport,
            revision,
            page_sizes: (state.page_sizes)(),
            current_page: state.current_page,
        }
    }
}
