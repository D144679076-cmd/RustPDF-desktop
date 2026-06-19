# Phase D1 — Navigation & Search

**Status:** Not started
**Effort:** ~1 week
**Outcome:** Outline/bookmarks sidebar, full-text search with jump-to-hit, text selection + copy.

## Tasks

### D1.1 — Outline / bookmarks
- Read with `pdf_core::document::outline::read_outline(doc)`.
- Render an indented, collapsible tree in `LeftPanel` (Bookmarks tab). Each item
  → `dest_page`; click scrolls the canvas to that page; ring current destination.

### D1.2 — Full-text search
- Search tab: input + result count. On submit call
  `pdf_core::text::search_document(doc, query, case_sensitive)` (requires the
  `search` feature; already enabled).
- List hits as `Page n · …snippet…` with the match bolded. Clicking a hit scrolls
  to `result.page_index` and flashes a coral overlay rect at `result.rect`
  (convert via `Viewport::pdf_to_css`).
- `Ctrl+F` focuses the search box (menu `find` event).

### D1.3 — Text selection + copy
- On the `Select` tool, extract spans with
  `pdf_core::text::extract_text_from_page(doc, i)`.
- Pointer drag on `.page-overlay` → map endpoints via `Viewport::css_to_pdf` →
  pick spans in range → draw translucent `--accent` selection rects.
- `Ctrl+C` copies the joined text via `arboard::Clipboard`.

## Acceptance
- Bookmarks navigate; search finds + highlights + jumps; selection copies real text.
- Works on `multipage.pdf`.

## Report
`.doc/d1-navigation-search-<date>.md`.
