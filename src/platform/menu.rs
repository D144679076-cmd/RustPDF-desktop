//! Native application menu bar (via `muda`, re-exported by dioxus-desktop).
//!
//! Provides the standard File / Edit / View / Tools menus with the keyboard
//! accelerators desktop users expect. Menu events are dispatched to the editor
//! in D4; this builds the structure.

use dioxus::desktop::muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};

/// Build the full menu bar. Accelerators use the platform-native modifier
/// (`Cmd` on macOS, `Ctrl` elsewhere) which `muda` maps from `CmdOrCtrl`.
pub fn build_menu() -> Menu {
    let menu = Menu::new();

    let file = Submenu::new("File", true);
    let _ = file.append_items(&[
        &MenuItem::with_id("open", "Open…", true, Some("CmdOrCtrl+O".parse().unwrap())),
        &MenuItem::with_id("save", "Save", true, Some("CmdOrCtrl+S".parse().unwrap())),
        &MenuItem::with_id("save_as", "Save As…", true, Some("CmdOrCtrl+Shift+S".parse().unwrap())),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id("print", "Print…", true, Some("CmdOrCtrl+P".parse().unwrap())),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::close_window(None),
    ]);

    let edit = Submenu::new("Edit", true);
    let _ = edit.append_items(&[
        &MenuItem::with_id("undo", "Undo", true, Some("CmdOrCtrl+Z".parse().unwrap())),
        &MenuItem::with_id("redo", "Redo", true, Some("CmdOrCtrl+Shift+Z".parse().unwrap())),
        &PredefinedMenuItem::separator(),
        &MenuItem::with_id("find", "Find…", true, Some("CmdOrCtrl+F".parse().unwrap())),
    ]);

    let view = Submenu::new("View", true);
    let _ = view.append_items(&[
        &MenuItem::with_id("zoom_in", "Zoom In", true, Some("CmdOrCtrl+Plus".parse().unwrap())),
        &MenuItem::with_id("zoom_out", "Zoom Out", true, Some("CmdOrCtrl+-".parse().unwrap())),
        &MenuItem::with_id("fit_width", "Fit Width", true, None),
        &MenuItem::with_id("toggle_theme", "Toggle Dark Mode", true, None),
    ]);

    let _ = menu.append_items(&[&file, &edit, &view]);
    menu
}
