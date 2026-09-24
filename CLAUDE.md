# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`oc` — a fact sheet on ovarian cancer at age 84 and above (survival; surgery
vs. chemotherapy vs. radiation), generated as HTML and PDF from Rust sources.
GPL-3.0. Document text is English.

## Build

```bash
make            # cargo build --release && ./target/release/infoblatt → ovarian-cancer-84.{html,pdf}
make pruef      # page images (pruef-N.png) for visual checking
make open
```

`FONT_DIR` overrides the font directory (default `./fonts`, DejaVu Sans,
embedded). The Makefile builds `--offline`; run `cargo build --release` once
online if the crate cache is missing. The PDF is checked in so it can be
downloaded without a Rust toolchain: rebuild and commit it with every content
change. The HTML is a by-product and stays untracked.

## Architecture

Same pipeline as `~/software/schoenlein-henoch` (and adhs-expert): pure Rust,
`genpdf` 0.2 typesets, `lopdf` overlays link annotations.

**All document text lives as data in `src/inhalt.rs`; content changes go
there only.** The `@page` `content:` line in `src/blatt.css` must contain
`inhalt::KOPFZEILE` verbatim (a `debug_assert` checks).

`pdf.rs`/`html.rs` are copied from schoenlein-henoch with only the fixed
strings localised ("Sources", "Page"). Read that repo's CLAUDE.md before
touching them; its pitfalls all apply here. The two that bite first:

- **Links are the underlines.** `LINK_MARKE` may never be used as a stroke
  colour anywhere else; `render()` aborts if the number of underlines found
  differs from the number drawn. Never remove that check.
- **A word wider than its column (or the line) is silently dropped by
  genpdf.** `render()` aborts on it instead. Fix by shortening the link text
  (`Verweis.text`) or adjusting `gewichte` in `src/inhalt.rs`. Long URLs in
  `QUELLEN` need a short display text.

Wrap measurements in `Span::N` ("12.3 months", "19.5 %") so they never break
across lines. Change content with a small Python script that counts its
anchor before replacing, then run `make` before committing.

## Side notes outside the fact sheet

`README.md` carries generic notes gathered while using this repo for a
real case (tumour profiling, lab printouts, HIN Mail and DICOM handling,
the 2026 drug landscape, discharge and home-care logistics, warning signs
and pain medication before the first cycle, nutrition, sample
delivery through a home-care pharmacy, preparing and enriching with
powdered oral nutrition, dry mouth, home-care time windows, night-time
restlessness and abdominal cramps, vomiting and weakness leading to
admission, who to notify on admission, questions for the admitting
physicians and how to reach them, records requests to every treating
department, blocked ureteric stents,
a one-page weekly timetable with link annotations, calendar invites and threaded replies via the Gmail REST API — never the Gmail MCP connector or
browser automation for that; the user has said so explicitly). They are
not part of the document. Keep them free of any personal data: no patient
or physician names, birth dates, addresses, or email threads. Only public
institutional contacts and URLs.
