//! pdf-desktop — native desktop PDF editor (Dioxus desktop shell).
//!
//! Responsibilities that live *here* (everything else is in `pdf-app-core`):
//! - Create the desktop window and register the `pdfpage://` custom protocol
//!   that streams rendered pages as PNG into the webview.
//! - Native menu bar, keyboard shortcuts, file dialogs, recent files.
//! - Hold the open [`Session`] in a Dioxus signal and route between the
//!   welcome screen and the editor.

mod platform;
mod routes;
mod views;

use dioxus::prelude::*;

use pdf_app_core::session::Session;
use pdf_app_core::theme::Mode;
use pdf_app_core::viewport::Viewport;

/// Global app state shared via Dioxus context.
#[derive(Clone, Copy)]
pub struct AppState {
    /// Active light/dark theme.
    pub mode: Signal<Mode>,
    /// Whether a document is currently open (drives welcome vs editor route).
    pub has_document: Signal<bool>,
    /// The open PDF session; `None` on the welcome screen.
    pub session: Signal<Option<Session>>,
    /// Zoom / fit-mode state for the page canvas.
    pub viewport: Signal<Viewport>,
    /// Page sizes in PDF points `(width, height)`, indexed by page number.
    /// Set when a session opens; empty when no document is open.
    pub page_sizes: Signal<Vec<(f64, f64)>>,
    /// Zero-based index of the page currently scrolled into view.
    pub current_page: Signal<usize>,
}

fn main() {
    env_logger::init();

    // Register the page-render protocol before launching so `<img
    // src="pdfpage://…">` resolves. The handler forwards to
    // `pdf_app_core::render_service::render_png` against the open session.
    let cfg = dioxus::desktop::Config::new()
        .with_menu(platform::menu::build_menu())
        .with_custom_protocol("pdfpage".to_string(), platform::render_protocol::handle);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(cfg)
        .launch(App);
}

/// Root component: provides global state, the design-system stylesheets, and
/// the router.
#[component]
fn App() -> Element {
    use_context_provider(|| AppState {
        mode: Signal::new(Mode::Light),
        has_document: Signal::new(false),
        session: Signal::new(None),
        // Default dpr=2 for crisp HiDPI rendering; 1x screens show slight
        // over-sampling which is visually acceptable for D0.
        viewport: Signal::new(Viewport {
            dpr: 2.0,
            ..Default::default()
        }),
        page_sizes: Signal::new(Vec::new()),
        current_page: Signal::new(0),
    });

    rsx! {
        // Shared design tokens + component styles, bundled from pdf-app-core.
        document::Link { rel: "stylesheet", href: asset!("/assets/theme.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/components.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/desktop.css") }
        Router::<routes::Route> {}
    }
}
