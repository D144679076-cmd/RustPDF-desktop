//! Desktop platform integration: render protocol, native menu, file dialogs.
//!
//! These are the only OS-aware parts of the app. Everything document-related
//! delegates to `pdf-app-core`.

pub mod dialog;
pub mod menu;
pub mod render_protocol;
