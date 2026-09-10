# oc

Fact sheet: ovarian cancer at age 84 and above. Survival, and surgery vs.
chemotherapy vs. radiation. Generated as HTML and PDF from Rust sources.
The PDF `ovarian-cancer-84.pdf` is checked in and can be downloaded directly.

## Build

```bash
make            # builds ovarian-cancer-84.{html,pdf}
make pruef      # page images for visual checking
```

Requires a Rust toolchain. See `CLAUDE.md` for the architecture and the
content-editing workflow.

## Molecular tumour profiling in Switzerland (notes)

Collected while organising a profiling for a patient in this age group.
No patient data here; the process is generic.

**Tissue is the gold standard.** FoundationOne CDx from tumour tissue
(archived paraffin block, image-guided core biopsy, or a cell block from
ascites/pleural fluid) reports BRCA1/2, HRD/LOH, copy-number changes and
tumour mutational burden reliably. Fluid drained via a catheter can go to
pathology for a cell block; that often yields enough tumour cells for
sequencing without a separate procedure.

**Liquid biopsy (ctDNA) is the fallback** when no tissue is available.
FoundationOne Liquid CDx sequences 324 genes from cell-free DNA in plasma.
Limitations: false negatives with low tumour fraction, no HRD score, and
age-related clonal haematopoiesis can be mistaken for tumour mutations.
Ask for a germline BRCA test alongside.

**Process (Switzerland).** Roche does not run the test itself; the
Institute of Pathology and Molecular Pathology of the University Hospital
Zurich (Schlieren) coordinates it as licensee. Steps:

1. A physician orders the test (order form, patient consent), ideally
   after a tumour board discussion.
2. Cost coverage: either a prior approval from the health insurer, or
   private payment (order of CHF 4'000–5'000).
3. For ctDNA: the practice requests the "Liquid Biopsy Versandbox" from
   the institute (fmi.pathologie@usz.ch, +41 43 253 18 18). Only the
   supplied cfDNA stabilisation tubes may be used. Two tubes of about
   8.5 ml, room temperature, no cooling, no centrifugation.
4. Ship to: Institut für Pathologie und Molekularpathologie, Wagistrasse 2,
   8952 Schlieren. Report to the ordering physician after about 2–3 weeks.

**Patient records.** Access to a hospital file by a relative needs the
patient's signed release from medical confidentiality (USZ form
"Entbindung vom ärztlichen Berufsgeheimnis", returned to the legal
department). State explicitly that the complete file is wanted, including
all imaging as DICOM, all laboratory values, interim and progress reports,
pathology, operation and discharge reports, delivered electronically.
Otherwise imaging and interim notes are commonly omitted.

Sources:

- USZ, Foundation Medicine order form:
  https://www.usz.ch/app/uploads/2023/08/foundation-medicine-order-form_de.pdf
- Roche Diagnostics Switzerland, FoundationOne Liquid CDx:
  https://diagnostics.roche.com/ch/de/article-listing/foundation-one-liquid-cdx.html

## Reading a hospital lab printout (notes)

Generic checklist that came out of reading cumulative lab sheets for a
patient in this age group. Not medical advice; it is a list of what to ask.

- **Kidney trend first.** Creatinine and eGFR over consecutive days. A
  doubling within days is acute kidney injury. Ask: ultrasound for
  hydronephrosis (ovarian cancer obstructs ureters), diuretics paused,
  drainage volumes and albumin replacement, urine sodium and urea to
  separate volume depletion from tubular damage, nephrology consult if no
  improvement in 48 h. eGFR below ~30 blocks carboplatin dosing.
- **Potassium against the medication card.** Potassium supplements are
  often still on the card after refeeding while the kidney is failing;
  above 5.5 mmol/l is an emergency (ECG, stop intake, binder).
- **Refeeding.** Weeks without food followed by low phosphate, potassium,
  magnesium and ketones in urine. Thiamine, slow build-up, daily
  electrolytes.
- **Effusion cytology.** Protein and LDH in pleural or ascitic fluid, and
  whether malignant cells were seen. A transudate without tumour cells
  changes the stage and means the fluid is not usable as a cell block for
  sequencing.
- **Missing values** worth requesting: albumin (also for the Geriatric
  Vulnerability Score), bilirubin, GGT, urea, calcium, coagulation, iron
  status, and a current medication card.

A separate throw-away Rust crate (genpdf, same DejaVu fonts, plain
`fonts::from_files`) was used to typeset such a lab summary for the family
and merge it with the scanned originals via `pdftk`. It lives outside this
repository because it contains patient data.

## Sending mail with attachments

Gmail's web connector cannot carry real attachments and driving the Gmail
UI in a browser is unreliable. Use the Gmail REST API directly with the
user's own OAuth token (scope `gmail.compose`): build an `EmailMessage`
with the PDF, `drafts.create` with the raw base64 body, then `drafts.send`.
An existing draft can be sent by id without re-uploading. If the token has
expired (`invalid_grant`), a one-off `InstalledAppFlow` login renews it.

## License

GPL-3.0.
