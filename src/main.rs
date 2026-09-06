// Fact sheet: ovarian cancer at age 84 and above.
// Copyright (C) 2026 Zeno R.R. Davatz
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at your
// option) any later version. See LICENSE.
//
// Not a substitute for medical advice.
//
//   cargo run --release
//   cargo run --release -- --html out.html --pdf out.pdf
//
// Font directory via $FONT_DIR (default: ./fonts).

mod html;
mod inhalt;
mod pdf;

use std::env;
use std::path::PathBuf;

use anyhow::Result;

const DEFAULT_HTML: &str = "ovarian-cancer-84.html";
const DEFAULT_PDF: &str = "ovarian-cancer-84.pdf";
const DEFAULT_FONT_DIR: &str = "fonts";

fn arg(args: &[String], name: &str, vorgabe: &str) -> PathBuf {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(vorgabe))
}

fn schreibe(d: &inhalt::Dokument, html_out: &PathBuf, pdf_out: &PathBuf, font_dir: &str) -> Result<()> {
    let html = html::render(d);
    std::fs::write(html_out, &html)?;
    println!("→ {} ({} B)", html_out.display(), html.len());

    let links = pdf::render(d, pdf_out, font_dir)?;
    let bytes = std::fs::metadata(pdf_out)?.len();
    println!("→ {} ({bytes} B, {links} links)", pdf_out.display());
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let html_out = arg(&args, "--html", DEFAULT_HTML);
    let pdf_out = arg(&args, "--pdf", DEFAULT_PDF);
    let font_dir = env::var("FONT_DIR").unwrap_or_else(|_| DEFAULT_FONT_DIR.to_string());
    schreibe(&inhalt::BLATT, &html_out, &pdf_out, &font_dir)
}
