# Phase D2 — Editing

**Status:** Not started
**Effort:** ~2–3 weeks
**Outcome:** Edit text with rich formatting, create annotations, page operations, undo/redo, save & save-as.
**Note:** This phase stabilizes the `pdf-app-core` overlay/tool components that `pdf-mobile` M3 reuses.

## Tasks

### D2.1 — Tool plumbing
- Bind the Tools rail to `pdf_app_core::tools::ToolState` (signal). Property panel
  content swaps on the active `Tool` (DESIGN.md §7).
- Every mutation flows through `Session::edit(|editor| …)` and records a
  `commands::Command` on the `History` signal (Ctrl+Z / Ctrl+Y, menu undo/redo).

### D2.2 — Text edit
- `Tool::TextEdit`: click a text block → build the edit model
  (`pdf_core::editor::build_text_model` / engine's text-edit entry) → inline
  editable overlay. Commit via the editor's `replace_text_block` equivalent.
- Property panel: font, size, B/I/U, color swatches, alignment → `RichFormat`
  (`editor.apply_format`).

### D2.3 — Annotations
- `Tool::Annotate(kind)` sub-tools. Build appearance with
  `pdf_core::forms::appearance::{highlight_appearance, freetext_appearance,
  ink_appearance, stamp_appearance, …}` and add via
  `pdf_core::writer::annotations::add_annotation(editor, page, ap, rect)`.
- Color/opacity/stroke come from `ToolSettings`. Render created annots on the
  overlay immediately (optimistic), reconcile on save.

### D2.4 — Page operations
- `Tool::Pages`: thumbnail grid with per-page rotate ⟲⟳, delete, drag-reorder.
- Extract selection → `pdf_core::editor::extract_pages(bytes, range)`.
- Merge/insert another PDF → `MergeBuilder`.

### D2.5 — Save
- `Save` → `Session::save()` (incremental `save_append`) → `std::fs::write` to the
  original path. `Save As` → `dialog::save_pdf_as`. Title dot clears; success toast.
- Re-publish the committed document to the render protocol and bump revision so
  the canvas reflects edits.

## Acceptance
- Edit text, add each annotation kind, rotate/delete/reorder/extract/merge pages.
- Undo/redo across all of the above. Save produces a valid PDF that re-opens with
  edits intact.

## Report
`.doc/d2-editing-<date>.md`.
