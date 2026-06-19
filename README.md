# pdf-desktop

Native **desktop PDF editor** for macOS, Windows, and Linux — built entirely in
**Rust with [Dioxus](https://dioxuslabs.com)** and powered by the
[`pdf-core`](../pdf-editor-rust-core) engine. No Electron, no JavaScript app
code, no WASM: the engine is linked **natively** and runs in-process.

> UI: Acrobat-familiar layout, [Claude-warm design](../pdf-app-core/DESIGN.md),
> simple enough for first-timers.

## Architecture

```
pdf-desktop (this repo)
  ├─ Dioxus desktop shell  (window, wry webview, native menu, file dialogs)
  ├─ pdfpage:// protocol    (renders pages → PNG into the webview)
  └─ uses ─────────────────────────────────────────────────────────────┐
pdf-app-core   session · viewport · tools · undo/redo · components · theme
  └─ uses ─────────────────────────────────────────────────────────────┤
pdf-core       parse · render (tiny-skia) · writer · forms · crypto · sign
```

Only the rendered **page** crosses into the webview (as PNG via the
`pdfpage://` custom protocol). All chrome is native Dioxus RSX styled with the
shared design tokens.

## Quick start

```bash
# one-time
cargo install dioxus-cli            # provides `dx`
rustup target add wasm32-unknown-unknown   # dx tooling dep

# run the app (hot-reload)
dx serve --platform desktop

# release bundle for the current OS (.dmg / .msi / .AppImage)
dx bundle --platform desktop --release
```

## Status

Scaffold in place (D0 skeleton). Implementation proceeds through the phase plan
in [`phase/`](./phase/README.md): **D0** viewer → **D1** nav/search → **D2**
editing → **D3** forms/signatures → **D4** desktop polish → **D5** package/CI.

See [`SPEC.md`](./SPEC.md) for the full product + technical specification.

## Repo layout

```
src/
  main.rs              window + pdfpage:// protocol + menu, mounts Router
  routes.rs            Welcome / Editor routes
  views/               welcome.rs, editor.rs (mounts shared EditorShell)
  platform/            render_protocol.rs, menu.rs, dialog.rs (OS-specific)
assets/
  desktop.css          welcome/window chrome (theme.css+components.css synced)
build.rs               copies shared CSS from pdf-app-core
Dioxus.toml            bundle config (icons, identifiers, per-OS installers)
```
