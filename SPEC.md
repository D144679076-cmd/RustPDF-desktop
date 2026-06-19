# pdf-desktop — Specification

**Status:** Draft v1 · **Date:** 2026-06-19 · **Owner:** pdf-engine

## 1. Purpose

A native, offline-first desktop PDF editor that reaches feature parity with the
existing web editor while delivering native performance and OS integration
(menus, file associations, printing). Built in Rust/Dioxus so the engine links
natively rather than through WASM.

**Independent** from `web-editor/` and the `packages/*` TS SDK — it shares only
the `pdf-core` engine and the `pdf-app-core` application crate.

## 2. Target platforms

| OS | Min version | Webview | Installer |
|----|-------------|---------|-----------|
| macOS | 11.0 Big Sur | WKWebView | `.dmg` / `.app` (notarized) |
| Windows | 10 1809+ | WebView2 | `.msi` / `.exe` (NSIS) |
| Linux | glibc 2.31+ | WebKitGTK | `.AppImage` / `.deb` |

Arch: x86_64 + aarch64 (Apple Silicon, ARM Linux).

## 3. Core flows

1. **Open** — file dialog / drag-drop / OS "open with" → parse via
   `Session::open` → route to editor. Encrypted files prompt for a password.
2. **View** — scroll multi-page on the canvas, zoom (25–500%), fit width/page,
   thumbnails, outline, full-text search (Ctrl+F).
3. **Edit** — text edit with rich formatting, annotations
   (highlight/underline/strike/freetext/ink/stamp/note), page ops
   (rotate/delete/reorder/extract/merge), undo/redo.
4. **Forms** — fill AcroForm fields, import/export FDF & XFDF, flatten.
5. **Sign** — draw/type/image signature; verify existing signatures.
6. **Save** — incremental `save_append` (default) or `Save As`. Print.

## 4. Technical design

### 4.1 Process model
Single process. Dioxus desktop creates one wry webview window. UI is RSX; the
engine runs on the main process and renders pages synchronously, off-loading
heavy renders to a worker thread pool in D5 if needed.

### 4.2 Page rendering bridge
Custom `pdfpage://render/{page}?scale=&dpr=` protocol
(`platform/render_protocol.rs`) → `pdf_app_core::render_service::render_png` →
`pdf_core::render::render_page_rgba` → PNG. The active document is published to
a process-global `RwLock<Option<Arc<PdfDocument>>>` on open/save/commit so the
protocol callback (outside Dioxus scope) can render the latest state.

### 4.3 State
`AppState` (Dioxus context): theme `Mode`, `has_document`. The open `Session`,
`Viewport`, `ToolState`, and `History` live in signals owned by the editor view
(see pdf-app-core). Edits go through `Session::edit`, which bumps a `revision`
counter; page `<img>` `src` includes the revision to bust the webview cache.

### 4.4 OS integration
- Native menu bar (`muda`) with standard accelerators.
- File dialogs + recent files (`rfd` + `directories`).
- Clipboard for copied text (`arboard`).
- File-type association + "open with" handler (D4).
- Printing via render-to-PDF/printer (D4).

### 4.5 Licensing
`pdf_core::license` tier checks surfaced as UI state: Free = view + watermark on
save; Pro = full edit; Enterprise = signatures. Activation dialog in D4.

## 5. Non-goals (v1)
- Real-time collaboration.
- Cloud sync / accounts.
- Native (non-webview) GPU renderer — revisit only if webview pixel throughput
  becomes a bottleneck (see Risks).

## 6. Performance targets
- Open a 50-page PDF < 500 ms to first page painted.
- Zoom/scroll at 60fps for text-heavy pages via tiled `pdfpage://` requests +
  render cache + lazy offscreen pages.
- Cold app launch < 1.5 s.

## 7. Risks & mitigations
- **Webview pixel throughput at high zoom** → tile requests, thumbnail
  downscaling, render cache in pdf-app-core; native renderer as a later option.
- **Font availability for rendering** → bundle the engine's core-fonts; verify
  the `render` font resolver path at D0.
- **Per-OS webview quirks** → CI matrix on all three OSes from D5.

## 8. Verification
- `cargo test` (workspace: pdf-app-core unit tests + desktop integration).
- `dx serve` smoke test: open fixtures `minimal.pdf`, `multipage.pdf`,
  `encrypted_aes256.pdf`, `form.pdf` from `pdf-editor-rust-core/tests/fixtures`.
- `dx bundle` produces a launchable installer per OS.
