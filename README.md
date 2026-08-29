# Baptism Program Builder (Rust)

A small Rust web app for creating a 4-page, half-fold baptism program that prints on one US Letter sheet, double-sided.

## Stack

- Rust
- Axum
- Askama templates
- Plain HTML/CSS

No JavaScript framework is required.

## Run

```bash
cargo run
```

Then open:

```text
http://127.0.0.1:3000
```

If port 3000 is already in use, choose another address:

```bash
BAPTISM_PROGRAM_ADDRESS=127.0.0.1:3001 cargo run
```

The four-page preview updates while you work. Each page is built from movable
elements rather than a prescribed form. You can:

- Add, duplicate, reorder, and remove headings, text, program items, hymns,
  scripture/quote blocks, images, decorations, and spacing on any page
- Copy every element on one page and paste it into another program window
- Expand the selected page and adjust its preview from 60% to 120%
- Match numbered sidebar elements to their counterparts in the live preview
- Start from standard templates or one-page-service layouts with hymns together
  on the inside left or split between the inside left and back
- Choose a style preset or set the paper, text, and accent colors yourself
- Create printer-friendly programs with the black-and-white preset and optional
  automatic grayscale conversion for images
- Switch between classic, clean, and rounded typefaces
- Add as many JPEG, PNG, or WebP image elements as the layout allows
- Click **Download save** to keep a self-contained editable JSON file
- Click **Load save** to reopen one of those files later
- Load save files from the earlier fixed-form version; they are upgraded into
  editable elements automatically
- Click **Print / PDF** to print or save the imposed booklet

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

The editor has no server-side logic (Askama renders `editor.html` as-is), so
it can be hosted as a static site. Pushing to `main` runs
`.github/workflows/pages.yml`, which copies `templates/editor.html` to
`dist/index.html`, copies `static/` alongside it, and publishes `dist/` to
GitHub Pages.

To enable it on GitHub: **Settings → Pages → Build and deployment → Source →
GitHub Actions**.

## Next improvements

- Shareable links
- QR code for digital program
