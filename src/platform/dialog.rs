//! Native file dialogs (via `rfd`) and recent-files persistence.

use std::path::PathBuf;

/// Show a native "Open PDF" dialog. Returns the chosen path, or `None` if the
/// user cancelled.
pub fn open_pdf() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("PDF", &["pdf"])
        .set_title("Open PDF")
        .pick_file()
}

/// Show a native "Save As" dialog seeded with `suggested_name`. Returns the
/// chosen path, or `None` if cancelled.
pub fn save_pdf_as(suggested_name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("PDF", &["pdf"])
        .set_file_name(suggested_name)
        .set_title("Save PDF As")
        .save_file()
}

/// Directory for app config and the recent-files list, per-OS.
pub fn config_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "pdf-engine", "pdf-desktop")
        .map(|d| d.config_dir().to_path_buf())
}
