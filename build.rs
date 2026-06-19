//! Sync the shared design-system stylesheets from `pdf-app-core` into this
//! crate's `assets/` dir so Dioxus `asset!()` can bundle them. This keeps the
//! Claude design tokens single-sourced in `pdf-app-core/assets/`.

use std::path::Path;

fn main() {
    let shared = Path::new("../pdf-app-core/assets");
    let local = Path::new("assets");
    let _ = std::fs::create_dir_all(local);

    for file in ["theme.css", "components.css"] {
        let src = shared.join(file);
        let dst = local.join(file);
        if src.exists() {
            let _ = std::fs::copy(&src, &dst);
        }
        println!("cargo:rerun-if-changed={}", src.display());
    }
}
