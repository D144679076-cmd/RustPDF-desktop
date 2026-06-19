//! App routes — welcome (no document) and editor (document open).

use dioxus::prelude::*;

use crate::views::{Editor, Welcome};

/// Top-level routes. The welcome screen is the landing page; opening a file
/// navigates to the editor.
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Welcome {},
    #[route("/editor")]
    Editor {},
}
