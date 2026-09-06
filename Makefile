# Builds the HTML and PDF fact sheet from the Rust sources in src/.
# Same pipeline as schoenlein-henoch: genpdf writes the PDF, lopdf adds
# the link annotations, DejaVu Sans is embedded.

HTML = ovarian-cancer-84.html
PDF  = ovarian-cancer-84.pdf
BIN  = target/release/infoblatt
QUELLEN = src/main.rs src/inhalt.rs src/html.rs src/pdf.rs src/blatt.css Cargo.toml

.PHONY: all open pruef clean

all: $(PDF)

$(BIN): $(QUELLEN)
	cargo build --release --offline

$(PDF) $(HTML): $(BIN)
	./$(BIN)

open: $(PDF)
	open $(PDF)

# Visual check page by page. From ten pages on the numbering switches
# from pruef-4.png to pruef-04.png.
pruef: $(PDF)
	rm -f pruef*.png
	pdftoppm -png -r 70 $(PDF) pruef

clean:
	rm -f $(PDF) $(HTML) pruef*.png
	cargo clean
