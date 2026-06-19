# Phase D0 — Skeleton & Render Pipeline

**Status:** Not started
**Effort:** ~1 week
**Outcome:** Open a PDF and view it — multi-page scroll, zoom, fit modes, thumbnails. The viewer works end to end.

## Prerequisites
- `dioxus-cli` (`dx`) installed; `pdf-app-core` and `pdf-core` build natively.
- Verify the render font path: `pdf_core::render::render_page_rgba` must find
  core fonts on native. Confirm the engine's `render` feature bundles fonts (or
  point the `DirectoryFontResolver` at the engine's `core-fonts`). **This is the
  first thing to validate** — a blank/garbled render means fonts aren't resolving.

## Tasks

### D0.1 — Launch + protocol
- Implement `main.rs` exactly as scaffolded: register `pdfpage://` via
  `Config::with_custom_protocol`, mount `Router`.
- In `platform/render_protocol.rs`, confirm `handle` returns PNG for a published
  document. Unit-test `parse_request` (already covered in pdf-app-core).

### D0.2 — Open a document
- Wire `views/welcome.rs` "Choose File" → `platform::dialog::open_pdf()` →
  `std::fs::read` → `pdf_app_core::session::Session::open(name, bytes)`.
- On success: store `Session` in a `Signal`, publish
  `Arc<PdfDocument>` via `render_protocol::set_active_document`, set
  `has_document = true`, navigate to `Route::Editor`.
- On `PdfError::Encrypted`, show a password dialog → `Session::open_with_password`.
- On other errors: danger toast "Couldn't open file".

### D0.3 — Editor shell + page list
- `views/editor.rs`: feed real `title` + `page_count` from the session into
  `EditorShell`.
- `PageView` renders one `<img src="pdfpage://render/{i}?scale={s}&dpr={dpr}">`
  per page. Bind `{s}` to `Viewport.scale`, `{dpr}` to the window DPR, and append
  `&rev={session.revision()}` to bust the cache after edits.
- Size each `.page-frame` from `Session::page_size(i)` × scale so layout is
  correct before the PNG loads (skeleton block until `onload`).

### D0.4 — Zoom & fit
- SubBar: page prev/next + `n / total`, zoom slider (`MIN_SCALE`..`MAX_SCALE`),
  zoom %, fit menu (Actual / Width / Page).
- Use `Viewport::zoomed`, `Viewport::effective_scale`. Ctrl+wheel zooms; the
  slider and buttons mutate the viewport signal.

### D0.5 — Thumbnails
- Left rail "Thumbnails" tab opens `LeftPanel`; render a low-scale
  `pdfpage://render/{i}?scale=0.2` per page; clicking scrolls to the page;
  ring the current page in `--accent`.

## Acceptance
- `dx serve` → open `multipage.pdf` → all pages scroll, render crisply on HiDPI.
- Zoom 25–500% and all fit modes work; thumbnails navigate.
- `encrypted_aes256.pdf` prompts for a password and opens.
- `cargo clippy -- -D warnings` clean.

## Report
Write `.doc/d0-skeleton-render-<date>.md`. Note any font-resolver workaround.
