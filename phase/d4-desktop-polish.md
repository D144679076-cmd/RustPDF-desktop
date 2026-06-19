# Phase D4 — Desktop Polish

**Status:** Not started
**Effort:** ~2 weeks
**Outcome:** Native menu actions, keyboard shortcuts, recent files, file associations, drag-and-drop, printing, dark mode, license activation.

## Tasks

### D4.1 — Menu + shortcuts
- Dispatch `muda` menu events (built in `platform/menu.rs`) to editor actions:
  open/save/save_as/print/undo/redo/find/zoom/fit/toggle_theme.
- Global accelerators match the menu.

### D4.2 — Recent files
- Persist the recent list (path + name) under `dialog::config_dir()`. Show as
  cards on the welcome screen; click re-opens.

### D4.3 — File association + drag-drop
- Register `.pdf` "open with" per OS (bundle metadata in `Dioxus.toml` + OS
  handler). Accept OS file-open events and drag-drop onto the welcome dropzone
  (`.drag-over` state) and the canvas.

### D4.4 — Print
- Render pages to a print-ready PDF / send to the OS print dialog. Simplest: emit
  the current document via `Session::save()` to a temp file and invoke the OS
  print path; refine to direct rasterized print later.

### D4.5 — Theme + license
- Wire the top-bar sun/moon to `Mode::toggled()`; persist the choice.
- License activation dialog → `pdf_core::license::activate(key)`. Gate features
  with `license::require(tier, feature)`; Free shows the save watermark and
  disables edit tools with an upsell.

## Acceptance
- All menu items + shortcuts work. Recent files persist across launches.
- Double-clicking a `.pdf` in the OS opens it in the app. Print produces output.
- Dark mode persists. License gating behaves per tier.

## Report
`.doc/d4-desktop-polish-<date>.md`.
