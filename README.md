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

**Checklist of results still missing before chemotherapy.** When the
diagnosis rests on cytology alone, list what has never come back or was
never ordered: immunohistochemistry on the ascites cell block (confirms
a gynaecological origin), histology of any biopsies taken at an earlier
endoscopy, the original cytology report itself, somatic BRCA/HRD, FRα
and PD-L1 on the cell block while untreated material is left, germline
BRCA from blood (recommended for every high-grade serous carcinoma), the
liquid biopsy result, final culture reports with antifungal
susceptibility, and inconclusive pleural cytology. Stent exchanges take
no tissue. For each item name who holds it (gynaecological oncology,
the referring clinic, pathology).

Get the original cytology report before calling immunohistochemistry
"pending": a summary in a later discharge letter may refer to a
different sample (pleura) while the ascites cell block was stained weeks
ago. A typical panel reads PAX8 positive (Müllerian origin), CDX2
negative (not intestinal), p53 mutation-type and p16 abnormal (fits
high-grade serous). The referring practice often holds this report and
the endoscopy histology, and forwards them on request. A liquid biopsy
covers somatic BRCA only in part (false negatives with little tumour
DNA in blood) and gives no HRD score; HRD needs tissue, which a
cell-poor cell block rarely provides; germline BRCA is a separate blood
test.

Reading a liquid biopsy report. The hospital's pathology wraps the
vendor report (some 26 pages) in a short local one; the comment on the
local pages is what the pathologists want acted on. The table of short
variants with allele fractions (VAF) sits on the last local page. Extract
the text with `pdftotext -layout` and search it instead of paging
through.

- A pathogenic variant in a cancer susceptibility gene (BRCA1/2 and the
  like) at a VAF near 50 % while the ctDNA tumour fraction is reported
  as low (under 1 %) is very probably inherited: tumour DNA alone could
  not reach that share. The report says it cannot tell germline from
  somatic; a separate germline blood test and genetic counselling
  settle it.
- Tumour drivers show up at small fractions (TP53 below 1 % fits
  high-grade serous). Variants flagged as possible clonal haematopoiesis
  (ASXL1 and similar) come from the blood itself.
- A BRCA mutation counts as homologous recombination deficiency, so a
  tissue HRD test matters less. On average such tumours respond better
  to platinum, and PARP inhibitors (olaparib, niraparib, rucaparib)
  become an option as maintenance after a response; dose and choice
  depend on kidney function.
- A low tumour fraction also means a negative result would have
  excluded nothing.

Arranging the germline test. It can be done in the same hospital: the
university hospital runs a genetic counselling clinic for hereditary
breast and ovarian cancer inside its gynaecology department
(gyn.onkologie@usz.ch, +41 44 255 51 50; self-referral or referral).
Swiss law ties a genetic test to counselling and written consent; the
test itself is one blood tube, which can be drawn during an inpatient
stay, and the result takes some weeks. Basic insurance usually covers
it for a patient with ovarian cancer; the clinic clarifies that first.
Relatives are counselled by the same clinic, but only once the variant
is confirmed in the patient, and then tested for that one variant.

What it costs in Switzerland: the full analysis of BRCA1/2 (usually a
gene panel) is about CHF 3600; a targeted test for a variant already
known in the family is a few hundred francs, from about CHF 300, and
faster. Counselling is billed as a medical consultation. Basic insurance
has to cover counselling and test when the Swiss (SAKK) criteria are
met, which a patient with ovarian cancer usually does, and first-degree
relatives once the family variant is known. The clinic normally obtains
a cost approval beforehand, because the insurer decides case by case;
deductible and the 10 % co-payment apply as for any service.

Sources: [Costs and coverage, Institute of Medical Genetics UZH](https://www.medgen.uzh.ch/dam/jcr:423590fb-9c91-4ed2-8c5d-3676ba79f8e0/21.1.1%20Info_Kosten_2023_04_13.pdf),
[Genetic counselling, Kantonsspital Baden](https://blog.ksb.ch/wissen/brustkrebs-was-bringt-eine-genetische-beratung/),
[Hereditary breast and ovarian cancer, Krebsliga](https://shop.krebsliga.ch/files/kls/webshop/PDFs/deutsch/erblich-bedingter-brust-und-eierstockkrebs-011004011111.pdf),
[Genetic counselling HBOC, USZ](https://www.usz.ch/fachbereich/brustzentrum/angebot/genetische-beratung-bei-familiaerem-brust-und-eierstockkrebs-hboc-syndrom/).

Ask for it in one mail to the physician who is the agreed contact, with
the treating oncologist and the department that ordered the liquid
biopsy in copy, each told in one sentence why they are copied. Quote
the pathology comment that recommends counselling, attach the report,
and do not state the patient's consent on her behalf: it is obtained in
the counselling session. Thank whoever made the test possible, also the
colleague outside the hospital who suggested it.

Telling the family about a possibly inherited variant. First-degree
relatives each have a 50 % chance of carrying it. BRCA1/2 variants raise
the risk of breast and ovarian cancer in women and, mainly BRCA2, of
prostate, male breast and pancreatic cancer; surveillance programmes
exist for carriers. Write the message in this order: what was tested,
the result and what is still unproven, what it means for the patient's
treatment (usually good news), what it may mean for relatives, next
steps. State plainly that nobody has to act now: confirmation in the
patient comes first, then each relative decides after counselling, and
testing is voluntary and targeted at the one variant. Attach the
original report, and say that the summary is a relative's reading, not
a medical assessment. Consider telling siblings in person before a mail
goes round.

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

Patients who like a malted cocoa drink can keep it as a flavour rather
than as the main source. A 20 g portion in 2 dl milk gives about
200 kcal and 8 g protein, most of it from the milk. Similar shop
products: malt plus cocoa (the closest), plain malted milk (milder),
cocoa-only powders (no malt). Chocolate or mocha sip feeds from the
pharmacy give 300 to 400 kcal and 12 to 20 g protein in 125 to 200 ml.
The simplest trick: stir a teaspoon of the malted drink into the
high-density powder shaken with warm milk, so it tastes familiar but
carries the full portion. Malt, cocoa and milk contain potassium and
phosphate; harmless while potassium is low, but ask the ward once
creatinine or potassium rise again.

## Evening checks on the ward (notes)

When a relative reports new symptoms from the bedside, a few questions
separate the harmless from the urgent. Each item below came up in one
evening; none replaces the ward physician.

- **Pain suddenly back at 7/10.** Check the obvious first: an infusion
  in the elbow crease kinks whenever the arm bends, and the analgesic
  stops. Ask for the missed dose and a cannula on the forearm or hand.
- **Pain eases after the bladder empties.** With double-J or tumour
  stents, a full bladder pushes urine back up to the kidney (reflux) and
  hurts in the flank. With a catheter in place the bladder should never
  fill: tubing without kinks, bag below bladder level, flush if it does
  not drain. Bladder spasms around catheter plus stents respond to
  tamsulosin or trospium (less confusing than oxybutynin in the elderly).
- **Cloudy urine.** Common with a catheter. Clear urine in the bag does
  not rule out a blocked stent on one side, because the other kidney
  still drains.
- **37.3 °C under paracetamol.** Paracetamol lowers temperature; the
  leukocyte trend is the better signal.
- **Diffuse anterior abdominal pain with leukocytes jumping overnight.**
  Think of peritonitis (hard abdomen, pain on release), *Clostridioides
  difficile* (soft, foul-smelling stool under broad-spectrum antibiotics;
  proton-pump inhibitors roughly double the risk), bowel obstruction
  (no stool or wind, vomiting), more ascites, and only then tumour pain.
  Ask for a targeted *C. difficile* test (GDH antigen, toxin or PCR), not
  a general stool culture; a negative GDH makes it very unlikely, and the
  diarrhoea is then usually the antibiotic itself. Visitors wash hands
  with soap, since alcohol does not kill the spores.
- **Cough on every change of position, choking only when upset.** Points
  to pleural effusion or fluid pushing the diaphragm up, or to saliva
  when breathing and swallowing fall out of step. Check oxygen
  saturation and breathlessness at rest; keep the upper body at 30 to 45
  degrees.
- **Read the labels on the drip stand.** A Ringer-acetate bag at 10 ml/h
  only keeps the vein open and delivers about 1 mmol potassium a day; it
  is not potassium replacement. Paracetamol 1 g bottles: in a frail,
  underweight patient with abnormal liver values ask for at most 3 g a
  day, and for a second scheduled analgesic (not an NSAID with kidney
  injury) if pain stays above 4.
- **Check the tablets too.** Fluconazole raises amlodipine levels
  (hypotension, oedema), prolongs QT (worse with low potassium), needs a
  renal dose, and slows paclitaxel breakdown; tell the oncologist.
- **Potassium replacement looks like this.** Effervescent tablets,
  slow-release potassium chloride tablets or capsules, syrup, or an
  infusion bag labelled KCl. A resin powder stirred into water does the
  opposite (binds potassium) and should stop once potassium is low.
- **Creatinine rises again after falling for days.** Two causes to
  separate: fluid loss (diarrhoea, poor intake), which responds to
  fluids, and renewed obstruction of a stent, which needs ultrasound. A
  side that was hard to see on the last scan is not cleared. Read it
  together with CRP and leukocytes: all three rising means the infection
  is not controlled, and chemotherapy will wait.
- **Check why a drug was started before agreeing to stop it.** A proton
  pump inhibitor raises the risk of *C. difficile*, but if an earlier
  gastroscopy showed severe ulcerating reflux oesophagitis it is the
  treatment (about eight weeks) and protects against bleeding. Reports
  from the referring clinic may arrive weeks later and change the
  answer; a falling haemoglobin then raises the question of bleeding
  from the oesophagus.
- **Antibiotics before admission blunt cultures.** Two days of
  amoxicillin-clavulanate from a general practitioner can leave urine
  and blood cultures empty, so a later enterococcal bacteraemia says
  little about when the infection began.
- **The cover sheet of an ultrasound carries the weight.** A drop of
  1.5 kg in a day with diarrhoea points to volume loss as the cause of a
  rising creatinine, when the kidneys show no new dilatation.
- **Infusion or tablet for pain.** Same strength at the same dose;
  the infusion acts within minutes and does not depend on swallowing or
  absorption, the tablet needs 30 to 60 minutes. Switch to oral or
  subcutaneous forms a few days before discharge, and remember that an
  infusion fails silently when the line kinks.
- **Feeling better while the labs get worse.** Less pain on the phone
  often reflects good analgesia or less diarrhoea; CRP, leukocytes and
  creatinine decide whether the infection is retreating. Two worse days
  in a row are a trend: the question for the team is what changes in
  treatment.
- **A CT after two worse days** looks for the source: obstruction or
  abscess around kidneys and stents, a walled-off collection, more
  ascites, bowel obstruction or perforation, gallbladder, lung bases.
  With an eGFR in the twenties it is often done without or with reduced
  contrast, which shows abscesses less well; ask whether contrast was
  given and watch creatinine for two to three days. CT goes through
  radiology, so it appears in PACSonWEB: request a new reference code
  from the radiology archive for each new study, attach the signed
  release, and list any earlier study that was announced but not
  visible.
- **Cough and vomiting before admission.** A chest radiograph may
  already say "aspiration possible" in a lower lobe while showing no
  pneumonia. That fits coughing on position changes; ask what the next
  CT shows in the lung and whether a speech therapist should assess
  swallowing.
- **Soft, salty, lukewarm food** suits a sore oesophagus and loose
  stool: mashed potato, egg dishes, broth with semolina or egg, puréed
  vegetable soups with cream, polenta, risotto, poached fish, cottage
  cheese; banana when potassium is low. Avoid acid (citrus, tomato),
  spice, very hot food and raw vegetables for now.
  From a Japanese kitchen the same idea reads: miso soup with silken
  tofu, chawanmushi (steamed egg custard), rice porridge with egg
  (okayu), rice soup with egg and finely cut chicken or fish (zosui),
  soft udon with egg, simmered pumpkin or sweet potato; kinako or sesame
  paste stirred in for protein and calories. Everything cooked through,
  lukewarm, in cups rather than bowls, five or six times a day. Leave
  out raw fish and raw egg, pickles, wasabi, tempura, konnyaku and large
  amounts of seaweed. A complete powdered feed (one single-portion
  bottle, 55 g, is one portion) is more than protein: a spoonful or two
  per cup adds energy, protein and vitamins, stirred in once the food no
  longer steams.
- **Haemoglobin drifting towards 70 g/l.** Many hospitals transfuse
  around 70, earlier with symptoms. Ask whether blood is lost in stool
  or urine.
- **Appetite stays poor while inflammation is high.** Infection, ascites
  pressing on the stomach, antibiotics, antifungals and residual
  uraemia all dampen it. Six to eight tiny portions, energy-dense sip
  feeds, cold foods that smell less, no pressure at the table, a look in
  the mouth for thrush, and a request for the ward dietitian and an
  antiemetic before meals if there is nausea.
- **Microbiology lab or infectious diseases?** The lab signs off results
  for the treating physicians and does not discuss them with families.
  For bacteraemia plus candida plus stents before chemotherapy, the
  useful request is an infectious-diseases consult via the ward.

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

The web viewer may refuse to show the report ("you are not authorised
to view the report"), and the download dialog greys out "include
report". Download "DICOM format" anyway, without the bundled viewer and
with the original study data: the signed report still travels as a
Basic Text SR series and reads out with the `ContentSequence` walk
above. A report also names the prior study it was compared with, which
reveals examinations that are not in the list.

Plain radiographs (CR) come as JPEG Lossless, which `pydicom` cannot
decode without GDCM or pylibjpeg. `dcmdjpeg in out` from DCMTK
decompresses them first. Then scale between the 0.5 and 99.5
percentiles, invert if `PhotometricInterpretation` is `MONOCHROME1`,
and crop the lower lung zones at full resolution; the browser shows the
same image at about a quarter of its size.

A CT downloaded on the evening of the scan may lack the radiologist's
report: only an Enhanced SR acquisition protocol and the dose report
are inside, because the report is signed the next morning and then
arrives as a separate PDF. Check `StudyDate` and the archive size
before reading anything (a radiograph is about 10 MB, a CT several
hundred): downloading the wrong study twice is easy. To look at a CT
volume, decompress one thin series with `dcmdjpeg`, sort by
`ImagePositionPatient[2]`, stack, and reslice coronally with the aspect
ratio slice spacing over pixel spacing; montages of every tenth slice
in lung and soft-tissue windows give the overview. A native scan (no
contrast, chosen for a low eGFR) shows abscesses and peritoneal nodules
poorly.

Describe such images as provisional and correct yourself against the
report: fat stranding around the kidneys or a "consolidation" seen by a
layperson may be nothing, while a distended stomach was confirmed. An
*infiltrate* is lung tissue that looks denser because the air spaces
hold fluid or inflammatory cells, most often pneumonia; "small, new,
possible" means exactly that.

Ultrasound arrives differently: as small JPEG stills (about 700 pixels
wide) attached to a secure mail, without the written report and without
side labels. The DICOM originals add full resolution, adjustable
contrast, header data and, most usefully, cine loops if the examiner
saved any. Neither replaces the report, because the examiner watched
live. Ask for both. Intraoperative fluoroscopy from a stent exchange
may carry another department's code (the operating theatre belongs to
trauma surgery) but can be requested from the operating urologist.

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
- **Bounces from hospital addresses.** Large hospitals often build
  addresses from *all* first names (`firstsecond.last@…`), so the obvious
  `first.last@…` returns `550 5.1.1 User unknown`. Check the physician's
  page on the hospital website before sending, copy the department's
  general address so the mail lands even if one address is wrong, and
  after sending search for `from:mailer-daemon newer_than:1d`. Only the
  failed recipient needs the resend; say so, because the others get a
  duplicate.
- **Asking for an interim report.** During a long stay, the ward
  physician who visited can be asked in writing for an interim report
  (*Zwischenbericht*): the reason for admission, procedures, kidney
  values, culture results with the planned treatment, and the plan for
  catheter and stents. Copy the senior physician who signs the
  operation reports, name the oncologist who needs it for planning, and
  attach the signed release from confidentiality.
- **Signed versus unsigned release.** The blank template of a release
  from confidentiality and the signed scan tend to sit side by side with
  near-identical names. Keep only the signed one where attachments are
  picked up, put "signed" and the date in its filename, and check the
  name (and size: a scan is several times larger) before sending. If the
  template went out anyway, answer in the same thread at once with the
  signed version and a one-line apology; no need to repeat the request.
- **No bounce is not proof of the right recipient.** A guessed
  `first.last@…` that does not bounce may belong to a namesake in another
  department. Look the address up before sending; if patient data went
  to the wrong person, send a short request to delete it without
  forwarding, and resend to the correct address.
- **When the clinic asks to route communication through the patient.**
  Agree to bundle (one physician or the department address, fewer
  copies), but state plainly that the patient has mandated the relative
  with a signed release and that the right of access applies through a
  representative. It worked: the next reply promised all reports and
  brought the missing images.
- **The formal records request.** The complaints office answers a
  copied complaint within a day, confirms the right of access and points
  to the hospital's online form for requesting records. Upload a copy of
  the patient's identity card and, when a relative asks on her behalf,
  the signed release as authorisation. The form is the official channel:
  the 30-day deadline runs from its receipt, so file it even while mails
  with the ward continue. List exactly what is wanted (lab values,
  culture results, imaging reports, DICOM including ultrasound and
  fluoroscopy, operation reports, implant cards).
  In practice the form asks for more than the reply suggests: scope
  (single reports or the whole record, one or all clinics), a date
  range, the patient's details, and for a third party the requester's
  own address and identity card, front and back as two separate files,
  plus one "proof" upload. Combine the signed release and the patient's
  identity card into one PDF for that slot. The free-text remark holds
  about 250 characters, so put the list of wanted data in one dense
  sentence. Watermark every identity copy ("only for <hospital>, records
  request, <date>") and keep it small (a few hundred kB). The fields sit
  in an embedded form that browser automation cannot reach for uploads;
  uploads, the captcha and the submit button stay with the user anyway.
- **When the clinic director answers on paper.** A typical reply: the
  team informs the patient, relatives can join those talks on request,
  no day-by-day release of documentation during a stay, questions belong
  to rounds or an arranged meeting. It is defensible, because the access
  right comes with a 30-day deadline, not same-day delivery. Take the
  offer literally (ask for a callback today and a fixed slot after
  rounds) and state separately, without heat, what remains in dispute.
  Do not suggest that a frail patient collect and relay her own lab
  printouts; the relative with the release wants them sent directly.
- **Read the thread before a follow-up.** Before sending an addendum to
  someone, fetch the thread again: a reply may have arrived in between
  ("I would wait and see"), and an addendum that ignores it reads oddly.
  Refer to it in the first sentence.
- **One mail per audience.** An administrative request (a missing
  section of a lab report) goes to the agreed contact alone; a medical
  request that needs other departments goes in a separate mail with
  those departments in copy. Check the drafts list before creating a
  second draft to the same person: the first may already be sent.
- **A letter that arrives as a photo.** A phone "scan" is a PDF with one
  image and no text layer: `pdfimages` extracts it at full resolution
  (`pdftoppm` at screen resolution is unreadable), then rotate and
  downscale before reading.
- **Implant cards.** Under Art. 20 of the Medical Devices Ordinance
  (MepV, SR 812.213, in force since 26 May 2021, mirroring Art. 18 EU
  MDR) the manufacturer supplies an implant card for implantable devices
  (intended to stay at least 30 days), and the hospital fills in the
  patient's details, hands it over and keeps the information readily
  accessible. Tumour ureteric stents stay for months, but wards may say
  no card is required; ask for copies of the implant labels from the
  operation record (REF, LOT, UDI, each side, each exchange) as a
  fallback.
  The exemptions in Art. 18(3) MDR are sutures, staples, dental
  fillings, braces, crowns, screws, wedges, plates, wires, pins, clips
  and connectors; ureteric stents are not among them. Swissmedic
  supervises hospitals on this.
- **Draft vanished before sending.** `drafts.send` answering "Message
  not a draft" usually means the user already sent it from the Gmail UI,
  often with recipients added. Check the Sent folder before resending.
- **Cite the right law.** A cantonal hospital is a cantonal body, so the
  federal Data Protection Act (Art. 25 DSG) does not apply directly. In
  Zurich the basis is § 20 IDG (access to one's own personal data),
  § 28 IDG (decision within 30 days, or a reasoned delay) and § 19 of
  the cantonal patient law (inspection of the record and copies; copies
  may carry a cost-covering fee). Official sources: zhlex.zh.ch and the
  cantonal data protection commissioner (datenschutz.ch).
- **When it escalates.** A clinic may answer a heated phone call with a
  written warning that direct communication could be restricted. Reply
  in writing, factually, one recipient, and copy the hospital's data
  protection office and complaints office if the issue is access to
  records. What helps more than arguing: ask that the signed release be
  noted visibly in the record so nurses can answer the phone; ask for
  one named contact person; separate data that already exist (labs,
  culture results, imaging reports and images, sendable at once) from
  physician letters that take time; name the staff who handled it well.
  With half-private insurance the patient has a free choice of senior
  physician; asking to be told in advance of chief-physician rounds, so
  a relative can attend, is reasonable.

## Preparing a joint meeting and a second opinion (notes)

When the ward invites patient and relative to a meeting, accept at once
and bring one printed page: questions as checkboxes in blocks (infection
and course, stomach and nutrition, kidney/blood/stents, chemotherapy,
organisation), the last known values in a grey line, room for notes.
Update it when a report answers a question before the meeting. Check
the printer name with `lpstat -a` before `lp -d`.

For a second opinion from a physician elsewhere, reply in the thread in
which the phone call was arranged: diagnosis with how it was secured,
course in dated bullets, last labs, the latest imaging assessment, and
at most three questions. Attach the original reports (imaging, cytology
with immunohistochemistry, endoscopy with histology) and the one-page
course summary; offer the rest.

What such a meeting produces. The complaints manager may attend and take
minutes; a request to record audio or video will be refused. A typical
agreement: the right of access and the release are undisputed, there is
no duty to deliver daily, documents go to the relative through one named
physician in a fixed rhythm (for example every three days), medical
questions belong to rounds, and imaging is fetched by the relative
through the patient portal. Keep to it: one recipient, no parallel
requests to the secretariat, nursing or other clinics.

Check each batch against what was expected. A "cumulative report" can
contain the blood count only; search the text for creatinine, CRP and
potassium before assuming the chemistry is there, and ask the named
physician for the missing section in one short mail that first thanks
for what arrived.

## A one-page course summary (notes)

After a transfer and a readmission nobody in the family holds the whole
story. One landscape page does: left column the referring clinic (with
reports that arrived late) and the first stay as a dated table, right
column the current stay as a dated table with today's row in red, then
open points and the state of records and communication. Sources and
"no medical assessment" in one grey line under the title.

With `genpdf`, a two-column `TableLayout` row does not split a column
across pages: when one column is a line too long, a whole section jumps
to page two. Check `pdfinfo` for the page count and `pdftotext -f 2` to
see what spilled, then shorten sentences or move a section to the
shorter column, and only then touch margins. A second small crate can
reuse the first one's compiled dependencies offline by copying
`Cargo.lock`, linking the fonts directory and building with
`CARGO_TARGET_DIR` pointing at the first crate's `target`.

Update the page whenever a fact changes (form submitted, new values),
and write where a statement came from when it is not a document
("by phone").

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

When the tumour itself causes the stents to fail, chemotherapy is the
only treatment that can take the pressure off the ureters. The family
may then want to start treatment sooner than the team plans. Write to
the oncologist and put these in the mail:

- a short chain of evidence from the operation reports, such as
  extrinsic compression and kinking
- that the patient herself feels ready and wants to start
- a request to visit her on the ward the next day
- the wish for inpatient treatment, so that kidneys, infection, fluids
  and stents are monitored closely
- a request to coordinate with urology and discuss the case at the
  tumour board

Ask for the criteria for starting rather than demanding a start during
active sepsis. Most oncologists hold chemotherapy until the fever,
inflammatory markers and creatinine improve. Carboplatin is dosed from
kidney function. Paclitaxel is cleared mainly by the liver, so a
possible compromise is to start weekly paclitaxel alone and add
carboplatin once the kidneys recover. That depends on liver values.

If the family edits the draft in the mail program, read the current
draft back before changing it again, so their edits are kept. Phrase a
firm request as a sentence, not as a question.

Photographs from the room are often the fastest source of facts during
an admission.

- The whiteboard names the room and the nursing shifts. A printed line
  such as "discharge at 10:00" is the ward's standard time, not a
  discharge date.
- An orange fluid card means a 24-hour balance was ordered. Ask how much
  urine came out over the same period.
- The bedside chart gives the trend of temperature, pulse, blood
  pressure, oxygen saturation and pain score. It also shows the
  resuscitation status and whether an advance directive is on file, and
  lists the medication actually given.

Check that medication list against what is known from home. A drug the
patient did not tolerate may have been restarted, e.g. a proton-pump
inhibitor. A potassium binder such as polystyrene sulfonate points to
high potassium and can cause constipation. An antihypertensive may still
run while blood pressure is low after sepsis.

A decision against resuscitation discussed only verbally is recorded as
an emergency order, but "no advance directive" still stands. A short
written directive, drawn up calmly with the family or the GP, protects
the patient's wishes at later admissions too.

A delayed first cycle can come forward once the fever settles and the
pulse normalises on antibiotics, e.g. to the next working day instead
of a week later. When the date moves, rewrite a pending request mail to
the oncologist rather than sending the outdated one.

Comparing two stent exchanges on one page helps the family and the
team. Align both episodes by day relative to the procedure (day −1, 0,
+1 …), not by calendar date. Set creatinine and eGFR, potassium and
sodium, CRP, procalcitonin and leukocytes, and haemoglobin side by side.
Highlight the values that are out of range today, and state what each
percentage refers to. Creatinine typically peaks on the day of the
procedure and falls from the next day. It falls fastest in the first
three days and settles after about a week. Recovery is slower when
sepsis adds its own kidney injury.

When the swelling of weeks disappears within days of a working stent
(postobstructive diuresis), the obstruction was the main cause of the
fluid overload. Sodium, potassium, magnesium and phosphate then shift
quickly, so daily labs matter. At home, a daily weight taken at the same
time every morning is the earliest warning that the stents are failing
again. Call if the patient gains 1 to 2 kg in two or three days, passes
less urine, or the legs, abdomen or breathlessness get worse.

A bladder catheter after a stent exchange keeps bladder pressure low,
which helps drainage through the stents, and makes hourly urine output
measurable. It adds a bacteriuria risk of about 3 to 7 % per catheter
day, may add some bleeding and bladder spasms, and makes catheter urine
samples look infected almost by default. Cultures taken from the renal
pelvis during the operation are more informative.

Published reference points, useful for orientation and not a
substitute for the team:

- KDIGO: urine output below 0.5 ml/kg/h for 6 h defines acute kidney
  injury.
- eGFR by the BIS1 equation for women around 80: median about 63, 5th
  percentile about 46 ml/min/1.73 m².
- AABB 2023: transfusion is considered below 70 g/l haemoglobin in
  stable inpatients, and below 80 g/l with cardiovascular disease.
- In ovarian cancer, about 7 % of patients develop hydronephrosis. Stent
  and nephrostomy fail at similar rates, about 17 to 19 % in a year.
  Obstruction found together with the cancer diagnosis resolves more
  often, and renal atrophy predicts failure.

Sources: [KDIGO AKI guideline](https://kdigo.org/wp-content/uploads/2016/10/KDIGO-2012-AKI-Guideline-English.pdf),
[eGFR reference values, Kidney International](https://www.kidney-international.org/article/S0085-2538(25)00252-2/fulltext),
[AABB 2023](https://pubmed.ncbi.nlm.nih.gov/37824153/),
[Ovarian cancer and ureteral obstruction](https://pmc.ncbi.nlm.nih.gov/articles/PMC11816973/),
[Stent failure prediction](https://pmc.ncbi.nlm.nih.gov/articles/PMC10613761/).

How a stent infection arises, and whether it returns:

- Biofilm forms on every stent within days; colonisation rises with
  dwell time (about 28 % at 15 to 30 days, 46 % at 30 to 60 days), often
  with a sterile urine culture. Count dwell time from the first foreign
  body, not from the last exchange.
- In a prospective cohort, febrile stent-associated infections were
  caused by E. coli (38 %), enterococci (14.5 %) and candida (9 %);
  about a fifth went on to sepsis. Risk factors: female sex,
  comorbidity, a urinary infection in the previous three months, a
  bladder catheter. Prior antibiotics, steroids and raised glucose
  favour candida.
- It can recur as long as stents stay. What lowers the risk: treating
  the current infection to the end, early removal of the bladder
  catheter, a urine culture before and targeted antibiotic at each
  exchange, a fixed exchange schedule. What removes it: a tumour that
  shrinks under chemotherapy and frees the ureters.
- Tumour and infection feed each other only indirectly: the tumour
  obstructs and weakens, and raises CRP, leukocytes and platelets on its
  own; the infection delays chemotherapy, costs strength and lowers the
  kidney function that carboplatin dosing depends on.

Sources: [Stent colonisation, prospective study](https://pmc.ncbi.nlm.nih.gov/articles/PMC11623820/),
[Febrile stent-associated urinary infections](https://pubmed.ncbi.nlm.nih.gov/37160208/),
[Stent failure in malignant obstruction](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC5941491/).

Cultures after urosepsis:

- If the same organism grows in all blood-culture bottles and in both
  renal pelves, the stents were the source. *Enterococcus faecalis* is
  usually ampicillin-sensitive, so piperacillin-tazobactam covers it.
- Cephalosporins (ceftriaxone, cefuroxime prophylaxis) do not work
  against enterococci and may have let them take hold.
- Enterococcal bacteraemia in the elderly raises the question of
  endocarditis. Ask about an infectious-disease consult, follow-up blood
  cultures, an echocardiogram (TEE if needed), and the planned treatment
  duration before chemotherapy starts.
- A transthoracic echocardiogram that finds "no larger vegetations" with
  moderate image quality lowers the suspicion of endocarditis but does
  not exclude small vegetations; follow-up blood cultures reported as
  "result follows" are simply not finished (two to five days).
- *Candida* in the renal pelvis with stents is often colonisation. The
  IDSA 2016 guideline treats asymptomatic candiduria only in neutropenia
  (i.e. once chemotherapy lowers the white count), before urological
  procedures, or with pyelonephritis or fungus balls. Fungus balls can
  block stents.

Which stent is in the patient:

- Operation reports often give only "tumour stent, Ch 7/28 cm". Implant
  cards for polymer stents are often not handed out, because there is no
  obligation for non-metal devices.
- The operating-room material record holds manufacturer, REF and LOT.
  Ask the senior urologist for it. Ward nurses usually cannot supply it.
- Reinforced tumour stents differ in radial stiffness by a factor of
  three to four between brands. Flow simulations show stagnant zones,
  where encrustation starts, in some designs but not others.
- A single-centre series of 182 reinforced tumour stents found
  failure-free survival of 89 % at one month and 52 % at five months,
  despite a nominal six-month dwell time. Bilateral insertion, intrinsic
  obstruction and urinary infection at insertion predicted failure.
- Compression resistance does not protect against biofilm or
  encrustation. Material, coating, flow and dwell time do.
- Options when stents keep failing: thicker (Ch 8–8.5) reinforced stents,
  tandem stents, metal stents (after infection is cleared), nephrostomy,
  or a subcutaneous nephrovesical bypass.
- The Swiss UDI register (swissdamed) lists ureteral stents only by
  article number and size. It currently covers few manufacturers, so it
  is useful only once the REF is known.

How secure is the diagnosis:

- A "high-grade serous" finding on ascites cytology, without a
  laparoscopy or tissue biopsy, is a strong suspected diagnosis.
- ESMO 2023 and the ESGO–ESMO–ESP consensus prefer histology from an
  image-guided or surgical biopsy before chemotherapy, with enough tumour
  cells (≥30 %) for BRCA/HRD testing. Cytology is acceptable only as an
  exception and should be backed by immunohistochemistry on a cell block.
- An ultrasound-guided core biopsy of the omentum under local
  anaesthetic is the gentle route for frail patients (ISUOG/ESGO 2025).
- Cheap blood tests help too: a CA-125/CEA ratio above 25 favours a
  Müllerian origin over a gastrointestinal one.
- The referring hospital's transfer letter lists which tests and
  biopsies were still pending, and who received copies (e.g. the
  endoscopist). Use it to chase missing histology.

Liver values and chemotherapy: paclitaxel is dose-adjusted by bilirubin
and AST/ALT, not by GGT or alkaline phosphatase. A cholestatic pattern
with normal bilirubin allows full dosing. Carboplatin follows kidney
function. For unexplained cholestasis in peritoneal carcinomatosis, start
with ultrasound, then MRCP (no contrast needed). ERCP is only for rising
bilirubin or cholangitis. Sepsis cholestasis typically raises bilirubin
more than GGT.

Protein in acute kidney injury: do not stop meat. ESPEN suggests about
0.8–1.0 g/kg/day without dialysis, and more for cancer once the kidneys
recover. Avoid processed meat (salt, phosphate), and count protein drinks
towards the total. A large meat meal raises measured creatinine for a
few hours. Polystyrene sulfonate lowers potassium and can constipate.
Stop it once potassium is normal. Soft stool after days without one is
a good sign. With three or more watery stools a day on antibiotics, test
for *C. difficile*. Hot-water bottles on the abdomen are fine if not too
hot, covered and not lain on, but check the skin: old, oedematous skin
burns easily.

One-page research summaries are useful when the family wants to ask a
specialist informed questions. Verify authorship through Europe PMC
(`AUTH:"Name XY"` search) rather than search-engine snippets. Show the
PubMed ID in blue. After converting with `pdftocairo`, overlay link
annotations. Find each ID's box with `pdftotext -bbox`, and pair the
label with the number spatially, because words in narrow table cells do
not come out in reading order.

Where hospital images live. Urology ultrasounds are often made on the
department's own device (e.g. a bkSpecto unit labelled "Abdomen/UROLOGIE"),
and intra-operative fluoroscopy stays in the urology system. Neither
reaches the radiology archive, so a PACSonWEB access from radiology lists
only CT, X-ray and radiology ultrasound. Request urology images from the
urology secretariat. They may come by HIN as JPEGs; ask again for DICOM
originals and the written findings. The cover sheet of an ultrasound
series records height and weight. Comparing it with later ward weights
can quantify how much fluid left the body after drainage was restored.

PACSonWEB: every new access letter has its own reference code, and the
browser keeps the old session until you log out, so log out before
entering a new code. Studies listed on the letter can appear hours after
the access is issued. If they are still missing, ask the radiology
archive. The family logs in itself. An assistant should not type access
codes into login forms, but can open and read studies once the session
exists.

Printing: a PDF that looks right on screen can print as shifted
letters, e.g. "0 XP D" for a four-letter name. The printer's Ghostscript
filter misreads the embedded font. Converting text to outlines with
Ghostscript itself repeats the error. What works is rewriting the file
with Poppler (`pdftocairo -pdf in.pdf out.pdf`) and checking it with
`gs -sDEVICE=txtwrite` before printing or sending it.

## License

GPL-3.0.
