# Phase D5 — Package & CI

**Status:** Not started
**Effort:** ~1 week
**Outcome:** Signed installers for all three OSes, produced by a GitHub Actions matrix.

## Tasks

### D5.1 — Bundles
- `dx bundle --platform desktop --release` per OS:
  - macOS → `.dmg` / `.app`
  - Windows → `.msi` / `.exe` (NSIS)
  - Linux → `.AppImage` / `.deb`
- Finalize `Dioxus.toml` bundle metadata, icons (`assets/icons/`), identifiers.

### D5.2 — CI matrix
- GitHub Actions `os: [macos-latest, windows-latest, ubuntu-latest]`:
  - cache cargo + target; `cargo fmt --check`, `cargo clippy -- -D warnings`,
    `cargo test`.
  - `dx bundle` and upload installers as artifacts on tags.

### D5.3 — Signing / notarization
- macOS: codesign + notarize + staple (secrets: Developer ID, app-specific pw).
- Windows: Authenticode sign the `.msi`/`.exe` (secret: code-signing cert).
- Linux: optional GPG-sign the `.deb`.

### D5.4 — Release
- On `v*` tag: build all, sign, attach to a GitHub Release. Auto-update channel
  is a follow-up (note in report).

## Acceptance
- A tag yields downloadable, signed, launchable installers for macOS/Win/Linux.
- CI is green on all three OSes.

## Report
`.doc/d5-package-ci-<date>.md`.
