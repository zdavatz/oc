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
- **After a relieved obstruction.** If a nephrostomy or a catheter
  exchange suddenly produces litres of urine, the cause was post-renal and
  is fixed. Expect post-obstructive polyuria for days: balance intake
  against output, replace sodium, potassium and magnesium, keep diuretics
  and potassium tablets paused, and read the creatinine with a one-day lag.
  A pigtail with a bag is a stopgap; ask about a double-J stent before
  discharge.
- **Refeeding, second wave.** Once the kidney recovers and polyuria sets
  in, magnesium and phosphate drop again while potassium normalises.
  Supplements that were stopped during the hyperkalaemia need to come
  back, and magnesium is often missing from the medication card
  altogether. Check all three daily until intake is stable.
- **Radiology reports beat lab sheets.** A CT report carries the working
  diagnosis, the reason for the kidney failure (ureteric stenoses, stents,
  nephrostomies), gallbladder and bile-duct findings that explain
  cholestatic enzymes, and incidental lung nodules. Ask for the reports
  before the images. Positive ascites cytology means a cell block can
  replace a biopsy for molecular profiling.
- **Missing values** worth requesting: albumin (also for the Geriatric
  Vulnerability Score), bilirubin, GGT, urea, calcium, coagulation, iron
  status, and a current medication card.

A separate throw-away Rust crate (genpdf, same DejaVu fonts, plain
`fonts::from_files`) was used to typeset such a lab summary for the family
and merge it with the scanned originals via `pdftk`. It lives outside this
repository because it contains patient data.

## Receiving documents from a Swiss hospital

Hospitals send records through HIN Mail. The Gmail message is only a
notification; the real mail sits on `verapp-verify-mail.hin.ch` behind
the link (SMS verification on first open, then browser-bound). The
attachments are stored encrypted on a CDN and decrypted in the browser
with a key from the link, so there is no command-line download. The
practical route: turn off "Ask where to save each file" in Chrome, click
"Als EML herunterladen", then split the EML with Python's `email`
module:

```python
import email, os
from email import policy
m = email.message_from_binary_file(open('mail.eml', 'rb'), policy=policy.default)
for part in m.walk():
    if part.get_filename():
        open(os.path.join('out', part.get_filename()), 'wb').write(part.get_payload(decode=True))
```

Lab PDFs from the hospital system have a text layer (`pdftotext -layout`);
scanned printouts do not and need `pdfimages -j` plus reading the images.

## Discharge and outpatient chemotherapy (notes)

What the discharge papers of a Swiss university hospital contain and
what to check before the first cycle:

- **Two appointment letters.** One from the gynaecologic oncology clinic
  for the informed-consent visit (blood draw, blood pressure, compression
  stockings), one from the oncology day clinic listing the cycles. Six
  weekly slots mean the weekly carboplatin/paclitaxel schedule recommended
  for the very old.
- **A repeat prescription** with fixed items (B vitamins, thiamine, skin
  and nasal care) and on-demand items for the expected side effects:
  loperamide, laxatives, antiemetics, paracetamol, a sleep aid, a mouth
  rinse. Check what is missing against the last ward medication card:
  electrolyte supplements, PPI, antihypertensives, low-molecular heparin.
- **Before the first cycle** is the last moment to get a cell block from
  ascites or tissue profiled (BRCA, HRD, FRα, PD-L1); chemotherapy changes
  the material afterwards.
- **Ongoing document delivery.** Ask the treating senior physician in
  writing to send every lab sheet, image (DICOM), pathology and molecular
  report and every updated medication list by e-mail as they appear, and
  name the clinic secretariats in CC. A signed release of medical
  confidentiality makes this routine.
- **Electronic patient record (EPD) in Zurich, 2026.** Post Sanela stops
  operating EPDs at the end of 2026; Abilis is the remaining provider.
  Opening requires the patient in person with a biometric ID; a proxy for
  a relative can be set up. For the coming weeks direct e-mail delivery is
  faster than an EPD.

## Nutrition during weekly carboplatin/paclitaxel (notes)

After weeks of poor intake and with albumin in the high twenties, the
priority is protein and calories in small volumes, plus a fibre that
regulates rather than pushes. Swiss products that fit:

- **High-density oral nutrition** such as Omanda Moltein PLUS: 250 kcal
  and 21 g protein in 120 ml, fully balanced FSMP, several flavours (helps
  with chemotherapy taste changes). Two servings a day cover much of the
  protein need. Comparable to Fresubin or Resource but about twice as
  concentrated.
- **Partially hydrolysed guar gum (PHGG)**, e.g. Digesan FIBRE, 5 g a day:
  normalises stool in both directions, which suits the alternating
  diarrhoea and constipation under paclitaxel.
- Not suitable: protein-only low-calorie variants when calories are
  needed; rehydration solutions made for short bowel or high-output stoma
  (high sodium, no potassium).

Ask the treating oncologist or the hospital dietitian to prescribe the
oral nutrition (MiGeL reimbursement for diagnosed malnutrition) and to
match total protein to kidney function.

## Drug landscape, September 2026 (notes)

Ovarian, tubal and primary peritoneal high-grade serous carcinoma are one
disease in every guideline. What changed recently, with brand names:

| Drug | Brand | Mechanism | Setting | Status |
|---|---|---|---|---|
| Olaparib | Lynparza | PARP inhibitor | First-line maintenance, BRCA or HRD | NCCN 2026 added HRD without BRCA |
| Niraparib | Zejula | PARP inhibitor | First-line maintenance, all-comers | established |
| Rucaparib | Rubraca | PARP inhibitor | BRCA; maintenance in EU | ESMO update Jan 2026 |
| Bevacizumab | Avastin | anti-VEGF | First line and maintenance, HRD-negative | established |
| Mirvetuximab soravtansine | Elahere | ADC against FRα | Platinum-resistant, FRα high | FDA/EMA 2024, Swissmedic March 2025, ESMO 2025 |
| Pembrolizumab + paclitaxel | Keytruda | PD-1 | Platinum-resistant, PD-L1 positive | FDA 2026; drug approved in CH, this indication not |
| Relacorilant + nab-paclitaxel | Lifyorli | glucocorticoid receptor antagonist | Platinum-resistant, no biomarker | FDA March 2026 (ROSELLA); EMA pending since Oct 2025; not Swissmedic-approved |
| Avutometinib + defactinib | Avmapki Fakzynja | RAF/MEK + FAK | Low-grade serous, KRAS only | FDA 2025 |
| Sofetabart mipitecan | – | ADC against FRα | Platinum-resistant | breakthrough Jan 2026 |
| SIM0505 | – | ADC | Platinum-resistant | fast track 2026 |

Practical consequences: first line is still carboplatin + paclitaxel
(weekly in the very old), surgery only after response (ASCO 2025). When
tissue or an ascites cell block is profiled, order BRCA, HRD, FRα and
PD-L1 in one go. HIPEC requires cytoreductive surgery. Of the new drugs,
only relacorilant lacks any Swiss authorisation (single import under
Art. 9b HMG, reimbursement under Art. 71c KVV); pembrolizumab would be
off-label (Art. 71a/b KVV); Elahere is approved in Switzerland.

Source: [Swissmedic public summary, Elahere](https://www.swissmedic.ch/swissmedic/en/home/about-us/publications/public-summary-swiss-par/public-summary-swiss-par-elahere.html).

Sources: [NCCN changes into 2026](https://www.onclive.com/view/experts-unpack-the-most-notable-nccn-guideline-changes-heading-into-2026),
[ESMO 2026 summary](https://reference.medscape.com/cc2/p10/esmo-guideline-epithelial-ovarian-cancer-2026a1000di9),
[ASCO neoadjuvant guideline 2025](https://ascopubs.org/doi/10.1200/JCO-24-02589),
[ROSELLA, Lancet 2026](https://www.thelancet.com/journals/lancet/article/PIIS0140-6736(26)00462-9/fulltext),
[FDA approvals Q1 2026, AACR](https://www.aacr.org/blog/2026/04/01/fda-approvals-in-oncology-january-march-2026/),
[Mirvetuximab + carboplatin, SGO 2026](https://www.oncozine.com/sgo-2026-phase-2-trial-highlights-efficacy-and-safety-of-mirvetuximab-soravtansine-carboplatin-in-platinum-sensitive-ovarian-cancer/).

## Sending mail with attachments

Gmail's web connector cannot carry real attachments and driving the Gmail
UI in a browser is unreliable. Use the Gmail REST API directly with the
user's own OAuth token (scope `gmail.compose`): build an `EmailMessage`
with the PDF, `drafts.create` with the raw base64 body, then `drafts.send`.
An existing draft can be sent by id without re-uploading. If the token has
expired (`invalid_grant`), a one-off `InstalledAppFlow` login renews it.

## License

GPL-3.0.
