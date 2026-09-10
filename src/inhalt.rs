// Content of the fact sheet: ovarian cancer at age 84 and above.
// Copyright (C) 2026 Zeno R.R. Davatz
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at your
// option) any later version. See LICENSE.
//
// Not a substitute for medical advice.
//
// All text lives here as data; `html.rs` and `pdf.rs` are two outputs of
// the same structure. To change wording, change it here only.

/// Piece of text inside a paragraph.
pub enum Span {
    /// Running text.
    T(&'static str),
    /// Bold.
    B(&'static str),
    /// Italic.
    I(&'static str),
    /// Must not wrap – values such as "83 g/l". In HTML `span.nb`,
    /// in the PDF via non-breaking spaces.
    N(&'static str),
    /// Linked word inside a sentence: display text and target. In the PDF
    /// the click area sits behind the word, marked by the underline.
    L(&'static str, &'static str),
}

/// A clickable line.
pub struct Verweis {
    pub text: &'static str,
    pub url: &'static str,
}

/// A complete sheet: title, text and sources. `src/html.rs` and
/// `src/pdf.rs` typeset any document handed to them.
pub struct Dokument {
    pub titel: &'static str,
    pub titel2: &'static str,
    pub untertitel: &'static str,
    pub stand: &'static str,
    /// Running head. In HTML `html::render` replaces the default text of
    /// the `@page` rule in `blatt.css` with it.
    pub kopfzeile: &'static str,
    pub blocks: &'static [Block],
    pub quellen: &'static [(&'static str, Verweis)],
}

pub struct Tabelle {
    /// Empty when the table has no header row.
    pub kopf: &'static [&'static str],
    /// Column weights for the PDF layout.
    pub gewichte: &'static [usize],
    pub zeilen: &'static [&'static [&'static [Span]]],
    /// Rules under the rows.
    pub linien: bool,
    /// Chronicle: first column is a year, no rules, no header.
    pub chronik: bool,
}

pub enum Block {
    H2(&'static str),
    H3(&'static str),
    P(&'static [Span]),
    /// Side note, set smaller.
    Klein(&'static [Span]),
    Liste(&'static [&'static [Span]]),
    Tab(&'static Tabelle),
    /// Box with the key figures.
    Lead {
        werte: &'static str,
        blocks: &'static [Block],
    },
    /// Red warning box.
    Alarm {
        titel: &'static str,
        blocks: &'static [Block],
    },
    Adresse {
        name: &'static str,
        rolle: &'static [Span],
        zeilen: &'static [&'static [Span]],
        links: &'static [Verweis],
    },
}

pub const TITEL: &str = "Ovarian Cancer";
pub const TITEL2: &str = "at Age 84 and Above";
pub const UNTERTITEL: &str =
    "Survival, and what the evidence says about surgery, chemotherapy and radiation therapy";
pub const STAND: &str =
    "Fact sheet for the patient and her family · as of 10 September 2026 · to take along to the doctor's appointment";
pub const KOPFZEILE: &str = "Ovarian Cancer at Age 84 and Above";

use Block::*;
use Span::{B, L, N, T};

// ---------------------------------------------------------------------------
// Tables
// ---------------------------------------------------------------------------

static T_ALTER: Tabelle = Tabelle {
    kopf: &["Age at diagnosis", "Survival, advanced disease (stage III–IV)"],
    gewichte: &[30, 70],
    linien: true,
    chronik: false,
    zeilen: &[
        &[&[B("Under 45")], &[T("5-year relative survival over "), N("45 %"), T(".")]],
        &[&[B("65–69")], &[T("1-year relative survival "), N("57 %"), T(".")]],
        &[&[B("70–79")], &[T("1-year relative survival "), N("43–45 %"), T(".")]],
        &[&[B("80 and older")], &[T("1-year relative survival "), N("25–33 %"), T("; 5-year net survival "), N("19.5 %"), T(" (stage IIIC–IV, 2024 data) versus "), N("45.1 %"), T(" under 70.")]],
        &[&[B("85 and older")], &[T("5-year relative survival about "), N("8 %"), T(" (older SEER data).")]],
    ],
};

static T_BEHANDLUNG: Tabelle = Tabelle {
    kopf: &["Treatment received", "What the studies report"],
    gewichte: &[32, 68],
    linien: true,
    chronik: false,
    zeilen: &[
        &[&[B("No treatment, supportive care only")], &[T("Median survival about "), N("1–4 months"), T(" in advanced disease.")]],
        &[&[B("Chemotherapy alone, no surgery")], &[T("Median survival roughly "), N("8–19 months"), T(". Patients 75 and older: 5-year survival about "), N("7 %"), T(".")]],
        &[&[B("Chemotherapy, then interval surgery")], &[T("Patients 75 and older, median age 79: 5-year survival about "), N("25 %"), T(". Same overall survival as upfront surgery, fewer complications.")]],
        &[&[B("Full standard care in fit patients")], &[T("Over 80: median progression-free survival "), N("12.3 months"), T(" versus "), N("16.4 months"), T(" at 65–69. No significant survival difference between patients in their 70s and over 80 who received standard chemotherapy.")]],
    ],
};

static T_OP: Tabelle = Tabelle {
    kopf: &["Operation", "Age 75", "Age 85"],
    gewichte: &[50, 25, 25],
    linien: true,
    chronik: false,
    zeilen: &[
        &[&[B("Primary (upfront) debulking")], &[N("12.3 %")], &[N("26.0 %")]],
        &[&[B("Interval debulking after chemotherapy")], &[N("4.2 %")], &[N("7.2 %")]],
    ],
};

static T_GRUPPEN: Tabelle = Tabelle {
    kopf: &["Fitness", "Usual approach", "Realistic outlook"],
    gewichte: &[22, 44, 34],
    linien: true,
    chronik: false,
    zeilen: &[
        &[
            &[B("Fit"), T(" – independent, well nourished, few other illnesses")],
            &[T("Standard treatment: carboplatin/paclitaxel, often weekly, with surgery – preferably chemotherapy first and interval debulking.")],
            &[T("Median survival often 2–4 years; about 20–25 % alive at 5 years in advanced disease; better in early stage.")],
        ],
        &[
            &[B("Vulnerable"), T(" – some limitations, low albumin, moderate comorbidity")],
            &[T("Chemotherapy with adapted doses or schedules; surgery only after a good response and if complete removal is expected.")],
            &[T("Median survival roughly 1–2 years.")],
        ],
        &[
            &[B("Frail"), T(" – dependent, malnourished, major comorbidity, dementia")],
            &[T("Single-agent carboplatin, or supportive care with palliative radiation for local symptoms.")],
            &[T("Months rather than years; the goal shifts to comfort and quality of life.")],
        ],
    ],
};

// ---------------------------------------------------------------------------
// The document
// ---------------------------------------------------------------------------

pub static DOKUMENT: &[Block] = &[
    Lead {
        werte: "Fitness matters more than age · chemotherapy first, surgery second · radiation only for symptoms",
        blocks: &[
            P(&[T("Three sentences that carry everything that follows.")]),
            P(&[B("First: age alone does not decide the outcome."), T(" Frailty, nutrition, functional status and the amount of tumour left after surgery predict survival far better than the date of birth. Fit women over 80 who receive standard treatment do nearly as well as women in their seventies.")]),
            P(&[B("Second: at this age, surgery is safer after chemotherapy than before it."), T(" Upfront surgery kills about one in four 85-year-olds within 90 days; surgery after three or four cycles of chemotherapy kills about one in fourteen, with the same overall survival.")]),
            P(&[B("Third: radiation does not treat the disease, it treats symptoms."), T(" It has no curative role in ovarian cancer at any age, but it relieves pain and bleeding from single tumour deposits in three out of four patients, with few side effects.")]),
        ],
    },

    H2("The big picture"),
    P(&[T("Women aged 80 and older make up almost "), N("10 %"), T(" of ovarian cancer patients, and most are diagnosed at an advanced stage (FIGO III–IV), as younger women are. What differs is not the disease but how it is treated: among patients over 80 only about "), N("35 %"), T(" receive "), L("standard-of-care treatment", "https://doi.org/10.3390/cancers13050952"), T(", against "), N("78 %"), T(" of those aged 65–69. Over "), N("40 %"), T(" of women over 85 receive no definitive treatment at all. Part of that is appropriate – frailty is real – and part of it is age bias.")]),
    P(&[T("The tool that separates the two is a geriatric assessment. The "), L("Geriatric Vulnerability Score", "https://www.thelancet.com/journals/lanhl/article/PIIS2666-7568(22)00002-2/fulltext"), T(" combines albumin, lymphocyte count, activities of daily living, instrumental activities of daily living and a depression score. It was validated in 447 women aged 70 and older across six countries and predicts survival independently of the treatment given. French and American guideline groups recommend such an assessment before any treatment decision in older patients.")]),

    H2("Survival figures by age"),
    P(&[T("Population registries mix fit and frail, treated and untreated women. The numbers below are therefore averages that a fit, treated 84-year-old can expect to beat, and a frail one may not reach.")]),
    Tab(&T_ALTER),
    Klein(&[T("Sources: SEER analyses cited in "), L("CancerNetwork", "https://www.cancernetwork.com/view/ovarian-cancer-elderly-women"), T(" and "), L("Cancer Epidemiology 2024", "https://www.sciencedirect.com/science/article/abs/pii/S1877782124001760"), T("; 1-year rates from "), L("Ries 1993", "https://pubmed.ncbi.nlm.nih.gov/8420672/"), T(".")]),

    H2("Survival by treatment received"),
    P(&[T("The spread between groups is far larger than the spread between ages. That is the central finding of the last fifteen years of research in this population.")]),
    Tab(&T_BEHANDLUNG),
    Klein(&[T("Sources: "), L("SOFOG-GINECO-FRANCOGYN position paper", "https://pmc.ncbi.nlm.nih.gov/articles/PMC8909025/"), T(" for supportive care and chemotherapy alone; "), L("AJOG 2019", "https://www.ajog.org/article/S0002-9378(19)31011-7/abstract"), T(" for chemotherapy with and without interval surgery in patients 75 and older; "), L("Cancers 2021", "https://doi.org/10.3390/cancers13050952"), T(" and "), L("the 956-patient cohort", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10751294/"), T(" for standard care in the fit.")]),

    H2("Surgery"),
    P(&[B("Why it matters."), T(" Across all ages, the strongest predictor of survival is whether the operation leaves no visible tumour behind. In the "), L("FRANCOGYN cohort", "https://pmc.ncbi.nlm.nih.gov/articles/PMC7290352/"), T(" of 1,123 women, residual disease roughly tripled the risk of death (hazard ratio 3.2), and this held in every age group. Women over 65 more often end up with incomplete surgery – and that, not their age, is what costs them.")]),
    P(&[B("The risk at 80 and older."), T(" Upfront debulking surgery is a large abdominal operation, and octogenarians tolerate it badly. In one series the "), L("30-day mortality", "https://pubmed.ncbi.nlm.nih.gov/23714708/"), T(" was "), N("18.8 %"), T(" in women over 80 against "), N("4.0 %"), T(" in younger women; octogenarians were nine times more likely to die and "), N("70 %"), T(" more likely to have a complication within a month. Emergency operations are worse still: "), N("20 %"), T(" 30-day mortality in the elderly against "), N("5.6 %"), T(" for a planned operation.")]),
    P(&[B("Surgery after chemotherapy changes the picture."), T(" The largest study, "), L("47,117 women", "https://jamanetwork.com/journals/jamasurgery/fullarticle/2732440"), T(" with stage IIIC–IV disease in the United States, compared death within 90 days after upfront surgery and after interval surgery, that is surgery after three or four cycles of chemotherapy:")]),
    Tab(&T_OP),
    P(&[T("After upfront surgery the risk accelerates with every year past 71; after interval surgery it does not accelerate at all. A study of women over 75 found the "), L("same overall survival", "https://www.sciencedirect.com/science/article/abs/pii/S1701216320303637"), T(" with either sequence, with fewer complications after chemotherapy first – complete removal at the interval operation should still be the goal. And adding the interval operation to chemotherapy is worth it: in patients 75 and older, chemotherapy alone gave a 5-year survival of "), N("7 %"), T(", chemotherapy followed by "), L("interval surgery", "https://www.ajog.org/article/S0002-9378(19)31011-7/abstract"), T(" gave "), N("25 %"), T(".")]),
    P(&[B("Who should not be operated on."), T(" Poor nutrition (low albumin), poor performance status (ASA 3 or higher), stage IV disease and marked frailty mark "), L("poor surgical candidates", "https://pubmed.ncbi.nlm.nih.gov/23197271/"), T(". For them the usual recommendation is chemotherapy alone, or supportive care.")]),
    Klein(&[T("What this means at 84: upfront surgery is reserved for the very fit with limited disease and a surgeon confident of complete removal. For most others the route is chemotherapy first, then an operation if the response is good and the patient still fit – and no operation if not.")]),

    H2("Chemotherapy"),
    P(&[B("The standard stays the standard."), T(" Carboplatin plus paclitaxel is first-line treatment at every age. The "), L("EWOC-1 trial", "https://ascopost.com/news/may-2021/first-line-single-agent-carboplatin-vs-carboplatinpaclitaxel-in-older-women-with-ovarian-cancer/"), T(" randomised 120 vulnerable women aged 70 and older with stage III–IV disease – a Geriatric Vulnerability Score of 3 or more – to carboplatin alone, three-weekly carboplatin/paclitaxel, or weekly carboplatin/paclitaxel. Carboplatin alone gave significantly "), B("worse"), T(" survival, and the trial was stopped early. About "), N("30 %"), T(" of the women on carboplatin alone stopped treatment, against fewer than "), N("10 %"), T(" on the combinations. The lesson the authors drew, and the title of the accompanying editorial, was "), L("time to stop undertreating", "https://pubmed.ncbi.nlm.nih.gov/33885717/"), T(".")]),
    P(&[B("Weekly dosing is often kinder."), T(" In a retrospective series of patients over 70, weekly carboplatin/paclitaxel gave a "), L("longer overall survival", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10751294/"), T(" than three-weekly dosing, "), N("57 months"), T(" against "), N("30"), T(", and a longer progression-free survival, "), N("19 months"), T(" against "), N("8"), T(". The comparison was not randomised, but the weekly schedule spreads the dose and is easier on bone marrow and nerves. In the same 956-patient cohort, women over 80 who received standard chemotherapy had no statistically significant difference in survival from women in their seventies.")]),
    P(&[B("What can be adapted."), T(" Doses are reduced to kidney function – carboplatin is dosed by creatinine clearance, and this must be done properly at 84. Fewer cycles are common: "), N("23 %"), T(" of patients over 75 receive fewer than six, against "), N("10 %"), T(" of younger patients. Single-agent carboplatin is inferior to the combination but still far better than no treatment, and is a reasonable choice when the combination cannot be tolerated.")]),
    P(&[B("Maintenance therapy."), T(" After chemotherapy, PARP inhibitors (for BRCA-mutated or HRD-positive tumours) and bevacizumab are options. A small Chinese series of "), L("surgically ineligible elderly patients", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12530882/"), T(" treated with chemotherapy and then a PARP inhibitor reported a median survival of 57 months – but with fifteen patients and a median age of 73, that is a hint, not a promise. The practical point is to ask for BRCA and HRD testing of the tumour: it opens the door to an oral, generally well-tolerated maintenance drug.")]),
    P(&[B("What to watch for."), T(" Nerve damage from paclitaxel, low blood counts, fatigue, falls and kidney function. Older patients treated in the "), L("FRANCOGYN cohort", "https://pmc.ncbi.nlm.nih.gov/articles/PMC7291201/"), T(" did not have more chemotherapy toxicity than younger ones once the dose was adjusted, but they had less reserve when something went wrong.")]),

    H2("Radiation therapy"),
    P(&[B("No role in first-line treatment."), T(" Ovarian cancer spreads across the whole abdominal cavity, and whole-abdominal irradiation was "), L("abandoned decades ago", "https://pmc.ncbi.nlm.nih.gov/articles/PMC7235852/"), T(" in favour of platinum chemotherapy, which reaches the same places with less harm. No guideline recommends radiation to treat the disease at any age.")]),
    P(&[B("A real role for symptoms."), T(" Where a single tumour deposit causes pain, vaginal or rectal bleeding, obstruction, or a brain or bone metastasis, a short course of radiation works well. In a modern series, "), L("three out of four patients", "https://pmc.ncbi.nlm.nih.gov/articles/PMC7897761/"), T(" had a clinical response within a month and two out of three still had it after three months; pain improved in "), N("70 %"), T(", bleeding was controlled in 13 of 14. An older series found the same: bleeding stopped or lessened in 15 of 21 patients, pain in 11 of 20.")]),
    P(&[B("Well suited to the very old."), T(" Modern techniques allow "), L("short schedules", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12691232/"), T(" of one to five sessions with little toxicity – in the series above, no serious side effects were seen in patients treated to the abdomen or pelvis. Fewer visits matter when travelling to the treatment centre is itself a burden.")]),
    Klein(&[T("So radiation is not an alternative to surgery or chemotherapy for controlling the disease. It is a tool for comfort, on its own or alongside the other treatments.")]),

    H2("Molecular profiling of the tumour"),
    P(&[B("Why."), T(" Whether a PARP inhibitor is an option depends on the tumour's genetics: a BRCA1/2 mutation or homologous recombination deficiency (HRD). Profiling also shows which other driver mutations the tumour carries. It does not explain "), B("why"), T(" it mutated; it shows "), B("what"), T(" is there, and a germline blood test shows whether any of it was inherited. Ask for both.")]),
    P(&[B("Tissue is the gold standard."), T(" A comprehensive panel such as "), L("FoundationOne CDx", "https://www.foundationmedicine.com/test/foundationone-cdx"), T(" from tumour tissue reports BRCA1/2, HRD by loss of heterozygosity, copy-number changes and tumour mutational burden reliably. Tissue can come from an archived paraffin block of an earlier operation or biopsy, from an image-guided core biopsy, or from a "), L("cell block", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9376088/"), T(" made from ascites or pleural fluid. Fluid drained through a catheter for comfort can be sent to pathology at the same time; it often contains enough tumour cells for sequencing without any additional procedure.")]),
    P(&[B("Liquid biopsy is the fallback."), T(" When no tissue can be obtained, "), L("FoundationOne Liquid CDx", "https://www.foundationmedicine.com/test/foundationone-liquid-cdx"), T(" sequences 324 genes from cell-free DNA in blood plasma. Two tubes of blood, no procedure. Its limits matter at this age: with little tumour DNA in the blood the test can be "), L("falsely negative", "https://pmc.ncbi.nlm.nih.gov/articles/PMC7519428/"), T("; it gives no HRD score; and age-related mutations of blood stem cells (clonal haematopoiesis, common over 80) can be mistaken for tumour mutations. A negative liquid result therefore excludes nothing.")]),
    Klein(&[T("Practical order: archived tissue if any exists; otherwise fluid or a biopsy if the patient is fit enough; liquid biopsy if not. Reports take two to three weeks. Insurers do not always cover the test; ask for prior approval or expect a bill in the low thousands.")]),

    H2("Reading the lab printout"),
    P(&[T("Hospitals hand out cumulative lab sheets on request. Five things to look for before the ward round, in this order.")]),
    Liste(&[
        &[B("Kidney trend."), T(" Creatinine and eGFR over consecutive days. A doubling within days is acute kidney injury. Ovarian cancer and ascites obstruct the ureters, so the first question is an ultrasound for hydronephrosis; a nephrostomy or ureteric stent reverses it within days. The second is volume: diuretics, drainage of ascites or effusion and weeks of poor intake all lower kidney perfusion. Urine sodium and urea help tell the two apart. An eGFR below about "), N("30 ml/min"), T(" blocks carboplatin dosing, so the kidney comes before any chemotherapy decision.")],
        &[B("Potassium against the medication card."), T(" Potassium supplements started during refeeding often stay on the card while the kidney fails. Above "), N("5.5 mmol/l"), T(" is an emergency: ECG, stop all intake, binder or insulin-glucose.")],
        &[B("Refeeding."), T(" Weeks without food followed by low phosphate, potassium and magnesium and ketones in the urine. Thiamine, a slow build-up and daily electrolytes; the supplements must be cut back as soon as the values normalise.")],
        &[B("Effusion cytology."), T(" Protein and LDH in pleural or ascitic fluid, and whether malignant cells were seen. A transudate without tumour cells does not count as stage IV, and it is not usable as a cell block for sequencing.")],
        &[B("What is missing."), T(" Albumin (also needed for the Geriatric Vulnerability Score), bilirubin and GGT alongside a raised alkaline phosphatase, urea, calcium, coagulation, iron status, and a medication card that is not older than the last creatinine.")],
    ]),
    Klein(&[T("Haemoglobin, CRP, platelets and the neutrophil-to-lymphocyte ratio round out the picture: anaemia of chronic disease, inflammation and reactive thrombocytosis are common in advanced ovarian cancer and each is an unfavourable but not decisive sign. Ask for a transfusion plan below "), N("80 g/l"), T(" and for a urine culture when leucocytes or yeasts appear.")]),

    H2("Putting it together for an 84-year-old"),
    P(&[T("The question is fitness, not age. Guidelines from NCCN, ESGO-ESMO and the "), L("French SOFOG-GINECO-FRANCOGYN group", "https://pmc.ncbi.nlm.nih.gov/articles/PMC8909025/"), T(" recommend a geriatric screen – G8, the Geriatric Vulnerability Score, or a full geriatric assessment – to sort patients into three groups.")]),
    Tab(&T_GRUPPEN),
    P(&[B("Three practical points."), T(" The patient's own wishes weigh heavily at this age; many prioritise independence and time at home over the last months of survival, and the evidence supports treating that as a legitimate medical goal. Early involvement of a palliative care team improves quality of life whatever treatment is chosen, and does not shorten it. And a second opinion from a gynaecologic oncologist who treats older women routinely is worth the trip, because under-treatment of the fit and over-treatment of the frail are both common.")]),
];

// ---------------------------------------------------------------------------
// Sources
// ---------------------------------------------------------------------------

pub static QUELLEN: &[(&str, Verweis)] = &[
    ("Under-Treatment of Older Patients with Newly Diagnosed Epithelial Ovarian Cancer Remains an Issue. Cancers 2021; 13: 952",
     Verweis { text: "https://doi.org/10.3390/cancers13050952", url: "https://doi.org/10.3390/cancers13050952" }),
    ("Effectiveness and safety of standard chemotherapy in older patients with ovarian cancer: a retrospective analysis by age group and treatment regimen (956 patients). PMC10751294",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10751294/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10751294/" }),
    ("Influence of Age on Treatment and Prognosis in Ovarian Cancer Patients. Cancers 2025. PMC12071169",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12071169/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12071169/" }),
    ("Retrospective study of elderly patients with advanced ovarian cancer who did not undergo surgery. PMC12530882",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12530882/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12530882/" }),
    ("Similar Overall Survival Using Neoadjuvant Chemotherapy or Primary Debulking Surgery in Patients Aged Over 75 Years with High-Grade Ovarian Cancer. J Obstet Gynaecol Can 2020",
     Verweis { text: "https://www.sciencedirect.com/science/article/abs/pii/S1701216320303637", url: "https://www.sciencedirect.com/science/article/abs/pii/S1701216320303637" }),
    ("Chemotherapy alone for patients 75 years and older with epithelial ovarian cancer – is interval cytoreductive surgery still needed? Am J Obstet Gynecol 2019",
     Verweis { text: "https://www.ajog.org/article/S0002-9378(19)31011-7/abstract", url: "https://www.ajog.org/article/S0002-9378(19)31011-7/abstract" }),
    ("Age-Associated Risk of 90-Day Postoperative Mortality After Cytoreductive Surgery for Advanced Ovarian Cancer (47,117 women). JAMA Surg 2019",
     Verweis { text: "https://jamanetwork.com/journals/jamasurgery/fullarticle/2732440", url: "https://jamanetwork.com/journals/jamasurgery/fullarticle/2732440" }),
    ("Perioperative morbidity and mortality in octogenarians with ovarian cancer. PMID 23714708",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23714708/", url: "https://pubmed.ncbi.nlm.nih.gov/23714708/" }),
    ("Thirty-Day Mortality After Primary Cytoreductive Surgery for Advanced Ovarian Cancer in the Elderly. Obstet Gynecol 2011",
     Verweis { text: "https://journals.lww.com/greenjournal/abstract/10.1097/aog.0b013e31822a6d56", url: "https://journals.lww.com/greenjournal/abstract/10.1097/aog.0b013e31822a6d56" }),
    ("Impact of Age on 30-Day Mortality and Morbidity in Patients Undergoing Surgery for Ovarian Cancer. PMID 26076218",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26076218/", url: "https://pubmed.ncbi.nlm.nih.gov/26076218/" }),
    ("Considerations in the surgical management of ovarian cancer in the elderly. PMID 23197271",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23197271/", url: "https://pubmed.ncbi.nlm.nih.gov/23197271/" }),
    ("Management and Survival of Elderly and Very Elderly Patients with Ovarian Cancer: An Age-Stratified Study of 1123 Women from the FRANCOGYN Group. PMC7290352",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7290352/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7290352/" }),
    ("Ovarian Cancer in the Elderly: Time to Move towards a More Logical Approach to Improve Prognosis – FRANCOGYN Group. PMC7291201",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7291201/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7291201/" }),
    ("The challenge of ovarian cancer care in the oldest old. Cancer Epidemiology 2024",
     Verweis { text: "https://www.sciencedirect.com/science/article/abs/pii/S1877782124001760", url: "https://www.sciencedirect.com/science/article/abs/pii/S1877782124001760" }),
    ("Survival prognosis model for elderly women with epithelial ovarian cancer based on the SEER database. PMC10570503",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10570503/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10570503/" }),
    ("Ovarian Cancer in Elderly Women. CancerNetwork",
     Verweis { text: "https://www.cancernetwork.com/view/ovarian-cancer-elderly-women", url: "https://www.cancernetwork.com/view/ovarian-cancer-elderly-women" }),
    ("Ries LA: Ovarian cancer. Survival and treatment differences by age. Cancer 1993. PMID 8420672",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/8420672/", url: "https://pubmed.ncbi.nlm.nih.gov/8420672/" }),
    ("EWOC-1 (GCIG-ENGOT-GINECO): carboplatin with or without paclitaxel in vulnerable elderly patients with stage III–IV ovarian cancer. J Clin Oncol 2019 (abstract 5508)",
     Verweis { text: "https://ascopubs.org/doi/10.1200/JCO.2019.37.15_suppl.5508", url: "https://ascopubs.org/doi/10.1200/JCO.2019.37.15_suppl.5508" }),
    ("Falandry C et al.: First-Line Single-Agent Carboplatin vs Carboplatin/Paclitaxel in Older Women With Ovarian Cancer (EWOC-1). JAMA Oncol 2021 – summary in The ASCO Post",
     Verweis { text: "ascopost.com – First-Line Single-Agent Carboplatin vs Carboplatin/Paclitaxel (EWOC-1)", url: "https://ascopost.com/news/may-2021/first-line-single-agent-carboplatin-vs-carboplatinpaclitaxel-in-older-women-with-ovarian-cancer/" }),
    ("Validation of the geriatric vulnerability score in older patients with ovarian cancer: an analysis from EWOC-1. Lancet Healthy Longev 2022",
     Verweis { text: "https://www.thelancet.com/journals/lanhl/article/PIIS2666-7568(22)00002-2/fulltext", url: "https://www.thelancet.com/journals/lanhl/article/PIIS2666-7568(22)00002-2/fulltext" }),
    ("Vulnerable Older Adults With Ovarian Cancer – Time to Stop Undertreating. JAMA Oncol 2021 editorial. PMID 33885717",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33885717/", url: "https://pubmed.ncbi.nlm.nih.gov/33885717/" }),
    ("Multi-Disciplinary Care Planning of Ovarian Cancer in Older Patients: Position Paper from SOFOG-GINECO-FRANCOGYN-SFPO. PMC8909025",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC8909025/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC8909025/" }),
    ("Chemotherapy of ovarian cancer in elderly patients. PMC4706529",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC4706529/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC4706529/" }),
    ("Epithelial Ovarian Cancer in Older Women: Defining the Best Management Approach. ASCO Educational Book 2015",
     Verweis { text: "https://ascopubs.org/doi/10.14694/EdBook_AM.2015.35.e311", url: "https://ascopubs.org/doi/10.14694/EdBook_AM.2015.35.e311" }),
    ("Rethinking the Role of Radiation Therapy in the Management of Epithelial Ovarian Cancer. PMC7235852",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7235852/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7235852/" }),
    ("Palliative Radiation Therapy for Metastatic, Persistent, or Recurrent Epithelial Ovarian Cancer: Efficacy in the Era of Modern Technology and Targeted Agents. PMC7897761",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7897761/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7897761/" }),
    ("The Role of Radiotherapy in Octogenarian Cancer Patients. PMC12691232",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12691232/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12691232/" }),
    ("FoundationOne CDx – comprehensive genomic profiling from tumour tissue. Foundation Medicine",
     Verweis { text: "https://www.foundationmedicine.com/test/foundationone-cdx", url: "https://www.foundationmedicine.com/test/foundationone-cdx" }),
    ("FoundationOne Liquid CDx – 324-gene profiling from circulating cell-free DNA. Foundation Medicine",
     Verweis { text: "https://www.foundationmedicine.com/test/foundationone-liquid-cdx", url: "https://www.foundationmedicine.com/test/foundationone-liquid-cdx" }),
    ("Cell block preparation from malignant effusions for molecular testing. PMC9376088",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9376088/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9376088/" }),
    ("Clonal hematopoiesis and false positives in plasma cell-free DNA testing. PMC7519428",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7519428/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC7519428/" }),
    ("Palliative radiotherapy for ovarian cancer. Int J Radiat Oncol Biol Phys 1987",
     Verweis { text: "https://www.sciencedirect.com/science/article/pii/0360301687902549", url: "https://www.sciencedirect.com/science/article/pii/0360301687902549" }),
];

/// The sheet.
pub static BLATT: Dokument = Dokument {
    titel: TITEL,
    titel2: TITEL2,
    untertitel: UNTERTITEL,
    stand: STAND,
    kopfzeile: KOPFZEILE,
    blocks: DOKUMENT,
    quellen: QUELLEN,
};
