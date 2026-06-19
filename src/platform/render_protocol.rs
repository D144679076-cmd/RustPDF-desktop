//! `pdfpage://` custom-protocol handler for the desktop webview.
//!
//! Wired into [`dioxus::desktop::Config::with_custom_protocol`]. Parses the
//! request URI into a [`PageRequest`] and renders the page to PNG against the
//! currently-open document, which is published into a process-global slot when
//! a session opens (set by the editor on open; cleared on close).
//!
//! Using a global is deliberate: the wry protocol callback runs outside the
//! Dioxus reactive scope and cannot borrow component signals, so the open
//! document is shared here behind an `RwLock`.

use std::borrow::Cow;
use std::sync::{Arc, OnceLock, RwLock};

use pdf_app_core::pdf_core::parser::objects::PdfDocument;
use pdf_app_core::render_service;

use dioxus::desktop::wry::http::{Request, Response};

/// Process-global handle to the document currently being viewed/edited.
///
/// The editor publishes a fresh `Arc<PdfDocument>` here whenever the open
/// document changes (open, save, or commit) so renders reflect the latest
/// committed state.
static ACTIVE_DOC: OnceLock<RwLock<Option<Arc<PdfDocument>>>> = OnceLock::new();

fn slot() -> &'static RwLock<Option<Arc<PdfDocument>>> {
    ACTIVE_DOC.get_or_init(|| RwLock::new(None))
}

/// Publish the document the render protocol should serve pages from.
pub fn set_active_document(doc: Arc<PdfDocument>) {
    if let Ok(mut guard) = slot().write() {
        *guard = Some(doc);
    }
}

/// Clear the active document (on close).
pub fn clear_active_document() {
    if let Ok(mut guard) = slot().write() {
        *guard = None;
    }
}

/// Handle one `pdfpage://render/{page}?scale=&dpr=` request.
///
/// Returns `image/png` bytes on success, or an empty `404` body if there is no
/// active document or rendering fails (the UI shows a skeleton in that case).
pub fn handle(request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let uri = request.uri().to_string();

    let png = render_service::parse_request(&uri).and_then(|req| {
        let guard = slot().read().ok()?;
        let doc = guard.as_ref()?;
        render_service::render_png(doc, req).ok()
    });

    match png {
        Some(bytes) => Response::builder()
            .header("Content-Type", "image/png")
            .header("Cache-Control", "no-cache")
            .body(Cow::Owned(bytes))
            .unwrap_or_else(|_| not_found()),
        None => not_found(),
    }
}

fn not_found() -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(404)
        .body(Cow::Borrowed(&[][..]))
        .unwrap_or_else(|_| Response::new(Cow::Borrowed(&[][..])))
}
