#import sys: inputs
#set page(paper: "a4", margin: 2cm)
#set text(font: "Linux Libertine", size: 11pt)

#let d = inputs.data

#align(center)[
  #text(size: 18pt, weight: "bold")[#d.at("Titre")]
  #v(0.5cm)
  #text(size: 12pt)[#d.at("Auteur")]
  #v(0.3cm)
  #text(size: 10pt, fill: gray)[#d.at("Date")]
]

#v(1cm)

#grid(
  columns: (1fr, 1fr),
  gutter: 1cm,
  [
    *Pays:* #d.at("Pays") \
    *Ville:* #d.at("Ville") \
    *Domaine:* #d.at("Domaine") \
    *Désignation:* #d.at("Designation rédigee")
  ],
  [
    *Prêteur:* #d.at("Prêteur") \
    *N° inventaire:* #d.at("N° inventaire prêteur") \
    *Crédit:* #d.at("Credit line") \
    *DEXID:* #d.at("DEXID")
  ],
)

#if d.at("Image ref") != none {
  v(1cm)
  align(center)[
    #image(d.at("Image ref"), width: 60%)
  ]
}
