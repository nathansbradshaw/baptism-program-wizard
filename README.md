# Baptism Program Builder (Rust + WebAssembly)

A small Rust web app for creating a 4-page, half-fold baptism program that prints on one US Letter sheet, double-sided.

## Stack

- Rust, compiled to WebAssembly (`wasm32-unknown-unknown`)
- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) + `web-sys` for DOM access — no JS UI framework
- [Trunk](https://trunkrs.dev) for the build/dev-server pipeline
- Plain HTML/CSS shell (`index.html`, `static/app.css`)

The whole editor — rendering, state, event handling — is Rust. Trunk compiles it
to a `.wasm` file plus a small JS loader it generates automatically; there is
no hand-written JavaScript in this project.

## Run

Install the wasm target and [Trunk](https://trunkrs.dev) once:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Then start the dev server:

```bash
trunk serve --open
```

Trunk rebuilds and reloads the browser automatically as you edit.

The four-page preview updates while you work. Each page is built from movable
elements rather than a prescribed form. You can:

- Add, duplicate, reorder, and remove headings, text, program items, callout
  sections, hymns, scripture/quote blocks, markdown text, images, decorations,
  and spacing on any page
- Numbered elements in the sidebar match numbered markers in the live preview;
  hovering or focusing either one highlights its counterpart, and clicking a
  preview element jumps straight to its editor card
- Start from one of seven program templates: four full programs (child
  baptism + confirmation, child baptism confirmed later, convert baptism,
  multiple candidates) and three one-page-service layouts with hymns on the
  inside left, split between inside left and back, or two hymns together
- **Copy page** / **Paste page** to move a complete page's content into
  another program window — backed by the system clipboard, IndexedDB, and
  local/session storage so it works across windows and survives a reload
- **Expand page** plus the +/− zoom control (60%–120%) to inspect one page
  up close
- Choose a style preset or set the paper, text, and accent colors yourself
- Create printer-friendly programs with the black-and-white preset and optional
  automatic grayscale conversion for images
- Switch between classic, clean, and rounded typefaces
- Add as many JPEG, PNG, or WebP image elements as the layout allows, choose
  from the built-in art library (photography, hand-drawn, and AI-generated
  pieces), or upload a custom image for a decoration
- Click **Download save** to keep a self-contained editable JSON file
- Click **Load save** to reopen one of those files later
- Load save files from the earlier fixed-form version; they are upgraded into
  editable elements automatically
- Click **Print / PDF** to print or save the imposed booklet, or **Download
  sample PDF** for a plain reading-order preview

Images are resized in the browser before being added. They are embedded in both
the downloaded save and the printable preview, so no separate image files are
needed after the program has been saved.

## Printing

The browser print view creates two 11 × 8.5 inch landscape pages:

```text
Outside: [ Back Cover | Front Cover ]
Inside:  [ Inside Left | Inside Right ]
```

Print double-sided and fold vertically in half. On many printers this means **flip on short edge**, but test one copy first.

## GitHub Pages

This is a static site: `trunk build --release` produces a self-contained
`dist/` folder (HTML, CSS, the compiled `.wasm`, and its JS loader) with no
server-side logic. Pushing to `main` runs `.github/workflows/pages.yml`, which
builds with Trunk and publishes `dist/` to GitHub Pages.

To enable it on GitHub: **Settings → Pages → Build and deployment → Source →
GitHub Actions**.

## Next improvements

- Shareable links
- QR code for digital program
