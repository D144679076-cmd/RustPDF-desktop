# pdf-desktop — Implementation Phases

Self-contained implementation plans, in order. Each file lists tasks, the
engine/app-core APIs to call, and acceptance criteria.

## Rules for all phases
1. Read the phase file fully before coding.
2. Reuse `pdf-app-core` — do not duplicate session/viewport/render/tool logic.
3. Use design tokens (CSS variables), never hardcoded styles. See
   [`../../pdf-app-core/DESIGN.md`](../../pdf-app-core/DESIGN.md).
4. Run `cargo fmt && cargo clippy -- -D warnings && cargo test` after each task.
5. Smoke-test with `dx serve` against `pdf-editor-rust-core/tests/fixtures/`.
6. On completion: set `**Status:** Complete — YYYY-MM-DD`, write a
   `.doc/<scope>-<date>.md` report.

## Phases

| File | Phase | Outcome | Effort |
|------|-------|---------|--------|
| [d0-skeleton-render.md](d0-skeleton-render.md) | D0 | Open + view a PDF: scroll, zoom, fit, thumbnails | ~1 wk |
| [d1-navigation-search.md](d1-navigation-search.md) | D1 | Outline, full-text search, text select + copy | ~1 wk |
| [d2-editing.md](d2-editing.md) | D2 | Text edit, annotations, page ops, undo/redo, save | ~2–3 wk |
| [d3-forms-signatures.md](d3-forms-signatures.md) | D3 | AcroForm fill, FDF/XFDF, flatten, sign + verify | ~2 wk |
| [d4-desktop-polish.md](d4-desktop-polish.md) | D4 | Menu, shortcuts, recent files, file assoc, print, license | ~2 wk |
| [d5-package-ci.md](d5-package-ci.md) | D5 | Installers + GitHub Actions matrix, signing/notarization | ~1 wk |

**Dependency:** D0 unblocks all. D2 stabilizes `pdf-app-core` components that
`pdf-mobile` M1+ depend on.
