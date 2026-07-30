#let cartel(
  artiste: "",
  titre_fr: "",
  titre_en: "",
  annee: "",
  desc_fr: [],
  desc_en: [],
  credits: [],
  numero_inventaire: ""
) = {
  set page(width: 15cm, height: auto, margin: 2cm, fill: rgb("16162C"))

  set text(font: "inter", size: 10pt, lang: "fr", fill: white)

  text(size: 14pt)[#artiste]
  v(0.5em)

  text(size: 16pt, weight: 800, style: "italic", lang: "fr")[#titre_fr]
  v(0.3em)

  text(size: 11pt, style: "italic")[#titre_en]
  v(0.5em)

  text(size: 12pt)[#annee]
  v(1em)

  set par(justify: true, leading: 0.65em)
  text(size: 11pt)[#desc_fr]
  v(1em)

  set par(justify: false, leading: 0.65em)
  text(size: 9pt, tracking: -.2pt, lang: "en")[#desc_en]
  v(1.5em)

  text(size: 8pt)[#credits]
  text(size: 8pt)[
    #credits, #numero_inventaire
  ]
}

#import sys: inputs
#let d = inputs.data

#cartel(
  artiste: d.at("Auteur"),
  titre_fr: d.at("Titre"),
  titre_en: d.at("Titre"),
  annee: d.at("Date"),
  desc_fr: if d.at("explicatif") != none { d.at("explicatif") } else { "" },
  desc_en: if d.at("traduction") != none { d.at("traduction") } else { "" },
  credits: if d.at("Credit line") != none { d.at("Credit line") } else { "" },
  numero_inventaire: if d.at("N° inventaire prêteur") != none { d.at("N° inventaire prêteur") } else { "" },
)
