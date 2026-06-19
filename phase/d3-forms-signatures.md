# Phase D3 — Forms & Signatures

**Status:** Not started
**Effort:** ~2 weeks
**Outcome:** Fill AcroForm fields, import/export FDF & XFDF, flatten forms, sign and verify signatures.

## Tasks

### D3.1 — Form fill
- `Tool::Form`: list fields via `pdf_core::forms::read_form_fields(doc)` with
  type icons. Render interactive widgets on the overlay at each field rect.
- Edit values → `set_text_field` / `set_checkbox` (and choice/radio equivalents)
  through `Session::edit`. Record `Command::SetField` for undo.

### D3.2 — FDF / XFDF
- Property-panel actions: Import (`import_fdf` / `import_xfdf`) via file dialog;
  Export (`export_fdf` / `export_xfdf`) → save dialog.

### D3.3 — Flatten
- "Flatten form" → engine form-flatten (burn fields into page content), then
  re-render. Confirm with a dialog (irreversible without undo).

### D3.4 — Signatures
- `Tool::Sign`: draw (ink on overlay) / type / image signature; place rect on a
  page. Sign via `pdf_core::signatures::sign_document(bytes, key_der, cert_der,
  timestamp)` (requires `signatures` feature; Enterprise tier).
- Verify on open: `verify_signatures(doc)` → badges `✓ Valid` (success) /
  `✗ Invalid` (danger) / `? Unknown` in the Sign panel.

## Acceptance
- Fill + save `form.pdf`; re-open shows values. FDF/XFDF round-trips. Flatten
  produces non-interactive content. Sign then verify reports Valid; tamper →
  Invalid.

## Report
`.doc/d3-forms-signatures-<date>.md`.
