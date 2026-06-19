# pdf-desktop — Claude Instructions

## Project
Native desktop PDF editor (macOS/Windows/Linux). Rust + Dioxus desktop. Links
`pdf-core` natively (no WASM/FFI). Shares `pdf-app-core` with `pdf-mobile`.

## Architecture
- `src/main.rs` — window, `pdfpage://` protocol, native menu, Router.
- `src/views/` — Welcome + Editor (Editor mounts `pdf_app_core::components::EditorShell`).
- `src/platform/` — OS-specific only: render_protocol, menu, dialog.
- Shared logic & UI live in `pdf-app-core`; do not duplicate it here.

## Rules
| # | Rule |
|---|------|
| R1 | No panic on runtime/external data. `unwrap()`/`expect()` only on compile-time constants (e.g. menu accelerators) or in `#[cfg(test)]`. |
| R2 | OS-specific code stays in `src/platform/`. Everything document-related goes through `pdf-app-core`. |
| R3 | Design tokens are single-sourced in `pdf-app-core` (`theme.rs`/`theme.css`). Never hardcode colors/spacing — use the CSS variables. |
| R4 | Doc-comment every public fn (`///`: what, params, return). |
| R5 | One concern per file; split >600 lines. |
| R6 | Follow the phase plan in `phase/`. Update `**Status:**` on completion and write a `.doc/<scope>-<date>.md` report. |

## Design
UI spec: [`../pdf-app-core/DESIGN.md`](../pdf-app-core/DESIGN.md). Acrobat-familiar
layout, Claude warm-paper aesthetic. Progressive disclosure: top bar = file
actions only; tools + their options live in the right Tools panel.

## Verification (before reporting complete)
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
dx serve --platform desktop   # manual smoke test with tests/fixtures PDFs
```

## Commit style
`type(scope): description` — feat/fix/refactor/test/docs/chore. Stage specific files.
```
Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
```
