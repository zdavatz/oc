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

A university gynaecology department may send in only 10 to 20 liquid
biopsies a year, so the test is not yet routine. It still gives
information that can change treatment. A BRCA change or another
homologous-recombination defect makes a PARP inhibitor such as olaparib
an option for maintenance after chemotherapy. A repeat test shows
whether the tumour fraction in the blood falls under treatment. Ask when
the result is expected, and make sure the result of a sample taken
before chemotherapy reaches the family as well.

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

**Same filename, different versions.** Hospital systems name exported
reports by type and patient, not by version: two secretaries sending the
provisional and the final discharge report an hour apart produce
identical filenames. Extracting a second EML into the same folder
silently overwrites the first. Prefix extracted attachments with sender
and timestamp, keep every version, and check the print footer
(`Druckdatum … / 7` vs `/ 8`) or the word "provisorisch" before treating
a file as final. Verify with `md5sum` when a folder arrives twice.

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
- **The discharge report itself.** A Swiss internal-medicine discharge
  report may state that it omits the epicrisis "for administrative
  relief": diagnosis list and procedure only, no synthesis. Read it for
  what the ward letters did not say: pending results (liquid biopsy,
  immunohistochemistry), the intended chemotherapy regimen (a
  single-agent plan for a very old patient contradicts EWOC-1 and is
  worth questioning), what the urologists actually placed (tumour stents
  versus plain double-J), and what was dropped from the medication list
  (low-molecular heparin, thiamine) versus what was added (fixed
  paracetamol, protein powder, on-demand magnesium). Ask for the report
  by e-mail as well as on paper, and for the final version when it comes.
- **Home care (Spitex) after discharge.** The hospital issues a
  physician's Spitex order (scope, frequency, duration, insurance class)
  and a physiotherapy prescription (nine sessions, first within five
  weeks). In the city of Zurich the provider is Spitex Zürich; a patient
  is served by a neighbourhood team plus, for complex care such as
  catheters, a "Home & Care" unit, each with its own phone and mailbox.
  Write to both once, giving a relative's phone and e-mail, and ask for
  the framework contract, needs assessment, care plan, schedule and
  service statements to be sent electronically and kept updated.
- **Check the Spitex package on arrival.** The framework contract
  contains a data-protection consent page (health information to
  relatives, invoices to a relative, online translation); it may come
  back blank even after the visit. Ask for it to be filled in and signed
  at the next visit, naming the relative for information and a separate
  relative and e-mail address for invoices; the team sends a corrected
  scan the same day. The medication report is compiled by the nurse from
  what the patient and relatives say, not from the discharge letter, so
  diff it against the letter (proton-pump inhibitor, protein supplement,
  fixed vs. as-needed paracetamol, exact vitamin product). A drug the
  patient will not take ends up in the "reserve" list; if the discharge
  letter had it as fixed, that is a question for the next oncology visit.
  Every entry marked "P" is self-administered; the service only sets out
  the pills. The needs assessment may cut the hospital's order (three
  visits a day to one); a new assessment can be requested. The same
  filename convention (`Medikamentenbericht_<timestamp>.pdf`) reappears
  with each revision, so keep the timestamp.
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

## At home before the first chemotherapy (notes)

The days between discharge and the first outpatient cycle are when
problems surface that nobody is watching for.

- **Fever plus abdominal pain means a call the same day.** In peritoneal
  carcinomatosis with ureteric stents or nephrostomies, the usual causes
  are a blocked or infected stent (risk of urosepsis), infected ascites,
  or a narrowing bowel. Do not wait for the next planned appointment. An
  infection has to be ruled out before the first cycle anyway. Go
  straight to the emergency number if there are rigors, confusion, little
  or no urine, vomiting without stool or wind, or a hard abdomen.
- **Fixed paracetamol hides fever.** A "slight" temperature on four
  fixed doses a day counts as real fever. Tell the physician when the last
  dose was taken.
- **Watch the paracetamol total, not the single dose.** One gram per dose
  is normal, but after weeks of poor intake, with low albumin, old age or
  unclear liver values, the usual daily ceiling is about 3 g, often less.
  Count reserve tablets and combination products too. A patient who
  doubles her own dose is reporting worse pain. Tell the home-care nurses,
  who set out the pills, and the physician.
- **Opioids with carboplatin and paclitaxel** are routinely combined, and
  there is no relevant pharmacokinetic interaction. Morphine metabolites
  accumulate when the kidneys are weak, so hydromorphone or a low,
  spaced morphine dose is usual in the very old. Constipation never wears
  off and can tip into obstruction when there are tumour nodules on the
  bowel, so a fixed laxative starts on day one. The paclitaxel
  premedication (dexamethasone, an antihistamine) adds drowsiness on
  chemo days. Paclitaxel causes muscle aches two to three days after the
  infusion, which analgesics help, and later a neuropathy, which opioids
  barely touch.
- **Reaching the oncology team.** Senior physicians often do not answer
  e-mail. The outpatient clinic's practice assistants do, and scan every
  mail into the record. Keep their direct line next to the main switchboard.
  Some GP practices bill a physician's e-mail reply as a phone
  consultation.
- **Release of confidentiality.** A form filled in on screen is not
  signed. Send a scan or photo of the signed page. A hospital form
  releases only that hospital's physicians, so a GP practice may want its
  own copy.

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

Expect several parties to end up advising on nutrition at once: a
private dietetics practice the patient saw earlier, the hospital's own
dietitians, and a product manufacturer. Their written advice will
overlap (protein powder three times a day, enriched compote, milk drinks
between meals) but each covers only part of the picture; a private
sheet may ignore kidney function and prediabetes, the hospital may not
know about the private one. Collect every written recommendation in the
family file and bring them to one appointment to be reconciled.

A manufacturer's dietitian will, correctly, decline to advise on a
patient under active hospital treatment and refer back to the hospital's
own dietetics service, but will supply product samples through that
service. So route the request through the clinic secretariat: ask for
samples of two or three flavours to be ready at the first chemotherapy
appointment, so the patient can test tolerance and taste before anything
is prescribed.

In practice the clinic may pass such a request on to the external
dietetics centre that handles its outpatient follow-up. That centre
orders the samples, one small bottle per flavour, from a home-care
pharmacy that delivers medical nutrition. The samples then come to the
home a few days later instead of to the chemo appointment. The same
centre later applies for basic-insurance cover once the product is
tolerated, and it offers larger bags. The delivery day is known but not
the hour, and no tracking number is sent by default. To check, write to
the pharmacy's customer service with the patient's name, birth date and
address, the ordering dietitian and the order date. Keep the pharmacy's
free customer-service number with the other contacts. If fever or new
abdominal pain appears before the samples arrive, the treating
physicians decide first whether the patient should take them.

The powder products come as a starter kit of single-portion bottles
(55 g each). Fill with water or milk to the mark (120 ml) and shake, or
use three heaped scoops in a shaker, or stir the powder into food:
yoghurt, porridge, muesli, apple sauce, compote, soup, mashed potatoes,
milk coffee or a malted milk drink. Never boil it, because the protein
flocculates; stir it into warm food at the end. Start with one scoop and
work up to a full portion. Once mixed, use it the same day (24 h in the
fridge). One portion adds about 250 kcal and 21 g protein. Count it
against any protein powder already prescribed, so total protein stays
matched to kidney function.

A small breakfast (a cup of malted milk drink, an egg, a little
porridge) gives roughly 380 kcal and 19 g protein. That is about a
quarter of a day's need. Enrich it rather than enlarge it: stir a
portion of the powder or the prescribed protein powder into the
porridge once it has stopped steaming, cook it with whole milk and
finish with butter or cream, and scramble the egg with cream.
A dry mouth usually means too little fluid, especially with fever, pain
or large urine volumes after ureteric stents. It can also mean oral
thrush. Offer small sips every 10 to 15 minutes, ice chips, a saliva
gel from the pharmacy and gentle mouth care. That care matters anyway
before carboplatin and paclitaxel.

## Working with the hospital's DICOM images (notes)

Swiss hospitals hand out imaging via a PACSonWEB reference code (patient
login with code and birth date, "view and download"). "Bilder
herunterladen" offers DICOM Original as a ZIP; with Chrome set to ask for
a download location the dialog cannot be driven remotely, so download by
hand and hand over the path. A single ZIP held all studies (3 GB, ~2200
files, with DICOMDIR). Keep it out of the repository (`.git/info/exclude`).

What is inside besides pixels, and worth extracting with `pydicom`:

- **Radiology reports as DICOM SR** (Basic Text SR, series description
  "Radiologischer Befundbericht"): walk `ContentSequence` and collect
  `TextValue`. These are the signed reports, sometimes including ones not
  yet sent on paper.
- **AI reports as secondary-capture PDFs** (e.g. contextflow CFA Chest
  CT): multi-frame RGB images, render frame by frame. They flag nodules
  the radiologist may have judged differently; treat as questions for the
  physician, not findings.
- **Dose reports** and scanner metadata (model, kVp, CTDIvol, contrast).

Rendering: read `pixel_array`, apply `RescaleSlope`/`RescaleIntercept` to
get Hounsfield units, then window (soft tissue 40/400, lung -600/1500,
bone 400/1800). A 20-line script with `pydicom`, `numpy` and `Pillow`
does this; `pydicom`'s `stop_before_pixels=True` makes the inventory pass
fast. For side-by-side comparison of two dates, pair slices by anatomy,
not by z-coordinate, when the scanners differ.

What CT shows in diffuse peritoneal carcinomatosis: no mass. Omental fat
loses its dark homogeneity ("omental caking" in its early form),
mesentery turns streaky, fluid appears everywhere, ureters and bile
ducts dilate. Zooming does not reveal a tumour, because the cell layer
is thinner than the contrast resolution. This is why diagnosis came from
ascites cytology and why FAPI-PET, not CT, would image the disease
itself.

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

Two patterns that proved useful beyond PDFs:

- **Appointments as calendar invites.** Write one `.ics` file with all
  events (`METHOD:REQUEST`, a `VTIMEZONE` for Europe/Zurich, the
  relatives as `ATTENDEE` with `RSVP=FALSE`, a `VALARM` twelve hours
  before) and attach it as `text/calendar; method=REQUEST`. Opening the
  file imports every event at once; a single new appointment goes out the
  same way. Put location, phone number for cancellation and the ward's
  24-hour cancellation rule into `DESCRIPTION`.
- **Replying in a thread.** Fetch the original with `format=metadata`,
  set `In-Reply-To` and `References` to its `Message-ID`, prefix the
  subject with `Re:` only if missing, and pass `threadId` alongside
  `raw` in `drafts.create`. Providers who answer in five words ("Thu
  10/11 o'clock") get a reply that lists everything else on that day so
  they can spot the collision themselves.

## A one-page weekly timetable (notes)

Once several providers visit the home (nursing service, physiotherapist,
meal delivery) plus hospital appointments, relatives need a single page,
not the long report. What worked: A4 landscape, one column per day, one
row per hour slot (7:00 to 14:30 plus a "12:00" row for the delivered
meals), each cell a bold title plus two or three short lines. Below the
grid: a one-line preview of next week, a contacts line with every
e-mail address, and the current medication list as one paragraph.

Built with the same genpdf helpers as the report, plus a second binary
that overlays link annotations with `lopdf`: run `pdftotext -bbox` on
the rendered page, take the bounding box of every word that contains
`@` (mailto) or matches a drug name (search page on ch.oddb.org), and
write `Link` annotations with those rectangles. No underline drawing is
needed; colouring the words is enough. Pitfalls: a word wider than its
column is silently dropped by genpdf (split it with a hyphen and a
space), empty grid rows collapse unless given a minimum padding, and
every extra line in the footer pushes the page to two, so trim text
rather than fonts below 8 pt.

Ask each provider once, in writing, for delivery days and what happens
when nobody opens the door (the meal service leaves the parcel in the
letterbox); that answer goes into the timetable so nobody has to stay
home for it.

Show home-care visits as a window, not a fixed time. The nursing service
plans with a tolerance of one to two hours and will object if the family
timetable shows its planning sheet's slot (e.g. 9:00–9:45) as fixed.
Agree a window for ordinary days (e.g. 8:30–10:30) and fixed times only
on days with hospital, physiotherapy or GP appointments. When the
service moves a visit, the newest mail from the planners wins over what
a nurse said in the morning. Confirm it, and check that the gap after
physiotherapy still leaves the patient time to rest.

New night-time restlessness (kicking, agitation) with daytime
drowsiness in an old patient is often the first sign of delirium from
infection, dehydration or pain. It can also come from low magnesium,
calcium or potassium, rising creatinine, iron deficiency (restless legs),
constipation or a full bladder. Report it together with any fever.
Colicky abdominal cramps with no stool in peritoneal carcinomatosis
can mean a narrowing bowel. Get a same-day assessment and give no
stimulant laxative until obstruction is ruled out. Call emergency
services for vomiting, no wind or a hard, distended abdomen.

Vomiting that doesn't stop is not always a bowel obstruction. The
tumour may be irritating the peritoneum, a partial obstruction may
resolve, or there may be constipation, a urinary or gut infection, or
rising creatinine or calcium. None of this can be told apart at home,
and every one of them costs fluid and electrolytes. When vomiting comes
with weakness, fever and drowsiness, the patient belongs in hospital
that day. The simplest route is often the clinic that treats her
anyway, which has her full record.

The moment the patient is admitted, send one short mail to each party
instead of one long one to all:

- The family gets the full course.
- The home-care service is asked to pause all visits, naming the next
  one.
- The physiotherapist is told which session is cancelled.
- The meal service is asked to stop deliveries.
- The dietitian and GP are told for information only, and their later
  appointments stay booked.

Give the emergency physician what the record may lack: self-changed
doses (paracetamol can hide fever), stents and the date of their
placement, the last bowel movement, drugs stopped for intolerance, and
current oral nutrition.

Several physicians will hand over during the evening, so repeat those
points to each of them. Ask every one of them:

- What is the working diagnosis: obstruction, infection, stents, or
  electrolytes and kidney function?
- Which tests are being done: blood, urine, ultrasound or CT?
- What treatment is she getting now: IV fluids, antiemetics, analgesia,
  antibiotics?
- Is she admitted, to which ward, and on which number can the family ask?
- Does the next day's planned consultation still stand?
- Is the first cycle of chemotherapy postponed until the cause is clear?

In hospital the physicians decide on laxatives. Tell them which reserve
laxative is on the home list (a stimulant such as sodium picosulfate or
an osmotic one such as macrogol) and when it was last taken. Stimulant
laxatives are withheld while obstruction is suspected.

Emergency and ward physicians rarely send e-mail, and their addresses are
not in any document the family has. Hospital addresses often follow a
first-name.last-name pattern, but do not guess one: a wrong guess sends
medical details to a stranger. Ask on the ward for the physician's
address or card, or write to the clinic secretariat, which files every
mail in the record and forwards it. For questions the same evening,
the ward's phone number is faster than mail.

Once the admitting department is known, use the names the patient or
relatives are given on the ward, e.g. a surgeon's full name, to get the
real addresses. Then send the records request straight to the treating
physicians and that department's secretariat as well, not only to the
clinic that planned the chemotherapy. A patient can move from one clinic
to another within hours (emergency, then urology), and each keeps its
own documents.

What such a request asks for, sent as early as possible:

- admission, interim, operation and discharge reports
- blood and urine values, including the urine culture with antibiotic
  resistance testing
- all imaging, both as written findings and as DICOM
- the current medication list

Attach the signed release of confidentiality every time, and ask the
new department to coordinate with the oncologist about the postponed
cycle.

Blocked or infected ureteric stents can cause fever, flank and abdominal
pain, vomiting and weakness all at once. They are exchanged by
cystoscopy under a short anaesthetic, without an incision. The first
chemotherapy is usually delayed until the infection is treated.

Urine backing up above a failed stent stretches the renal pelvis. That
causes colicky flank and abdominal pain, often with nausea and vomiting,
and with fever if the urine is infected. Once drainage is restored, the
pain usually eases within hours to a few days. In peritoneal
carcinomatosis a stent often fails because tumour presses on the ureter
from outside. Other causes are encrustation, clots, infection, or a stent
that has moved or kinked. The urologists then choose between:

- two stents side by side in each ureter
- metal stents, which resist outside pressure and stay in longer
- a percutaneous nephrostomy, the most reliable option under tumour
  pressure but a burden in daily life

Chemotherapy that shrinks the tumour may relieve the pressure by itself.

Ask the team:

- What did the removed stents show?
- What did the urine culture grow, and how long will antibiotics run?
- What is the kidney function after the exchange?
- Which long-term solution is planned, and when will they decide?
- When is discharge, and is the postponed first cycle confirmed?

Hospital teams often prefer to send findings in one batch rather than
piece by piece. Accept that, and ask the ward directly for the room
number and visiting times.

Reconstruct the timeline of the stents from the discharge letter before
you ask why they failed. The letter usually lists every device: soft
double-J catheters from an external clinic, then tumour stents with a
larger calibre and length placed by cystoscopy. It also gives the
planned exchange interval, often six months. Count the days from the
last placement to the failure. Failure within two weeks points to tumour
pressure from outside or to infection, not to wear. The physician who
signs a discharge letter from internal medicine is usually not the
urologist who placed the stents. The operator's name is only in the
urology operation report, so request every operation report by date.

The anaesthesia pre-assessment printed for an emergency procedure is
worth photographing. It often holds what no other document gives the
family on the first day:

- the working diagnosis, e.g. suspected urosepsis with acute kidney
  injury
- the night's blood values: creatinine, leukocytes, CRP, procalcitonin,
  NT-proBNP
- the oxygen requirement and the antibiotic with its start time
- the ASA class
- the patient's own decision on intensive care and resuscitation

Tell the family about that decision with care. It belongs to the
patient.

When a same-day conversation is needed, say so at the top of the mail
in capitals, with a phone number. Name the dates of the procedures and
the question, and ask for a physician who knows the case.

What a urology operation report for a stent exchange contains:

- the operator and the physicians who visé or sign it, often a resident
  with a senior physician countersigning
- the duration, the anaesthesia and the indication
- the retrograde pyelography findings: how dilated each renal pelvis
  is, and any kinking or narrowing of the ureter
- the calibre and length of the stents placed, but usually not the
  product name
- any cultures taken from the renal pelvis
- whether a bladder catheter was left in
- the plan: follow-up ultrasound, antibiotics, thrombosis prophylaxis
  adapted to kidney function, the next exchange interval, and when to
  call early

A sepsis report may give a SOFA score. Two points or more define sepsis,
so six means real organ dysfunction. The copy list shows which outside
physicians are kept informed, e.g. the urologist who placed the first
catheters elsewhere.

If an early failure is followed by the identical stent again, that is
the question to ask. Why not a thicker stent, tandem stents, a metal
stent or a nephrostomy? And should the interval stay at six months, or
should ultrasound and creatinine be checked sooner?

## License

GPL-3.0.
