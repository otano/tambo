#let cartel(
  // Métadonnées de l'œuvre (français)
  author-fr: "",
  title-fr: "",
  date-fr: "",
  text-fr: [],
  caption-fr: [],
  
  // Métadonnées de l'œuvre (anglais)
  author-en: "",
  title-en: "",
  date-en: "",
  text-en: [],
  caption-en: [],
  
  // Paramètres de mise en page
  paper-size: "a6",
  flipped: true,
  bg-color: rgb("#f5f4f0"),
  text-color: rgb("#222222"),
  body,
) = {
  set page(
    paper: paper-size,
    flipped: flipped,
    margin: (x: 1.5cm, y: 1cm),
    fill: bg-color,
  )

  set text(
    size: 9pt,
    fill: text-color,
  )

  let author(name) = text(weight: "bold", name)
  let artwork(title) = text(style: "italic", title)

  grid(
    columns: (1fr, 1fr),
    gutter: 1.5cm,
    [
      // Colonne française
      #block(
        below: 0.4em,
        [
          #author(author-fr) \
          #artwork(title-fr) \
          #text(size: 8pt, fill: rgb("#555555"))[#date-fr]
        ]
      )
      
      #v(0.1em)
      
      #text(size: 7.5pt, justified: true)[#text-fr]
      
      #v(0.3em)
      #text(size: 7pt, fill: rgb("#666666"))[#caption-fr]
    ],
    [
      // Colonne anglaise
      #block(
        below: 0.4em,
        [
          #author(author-en) \
          #artwork(title-en) \
          #text(size: 8pt, fill: rgb("#555555"))[#date-en]
        ]
      )
      
      #v(0.1em)
      
      #text(size: 7.5pt, justified: true)[#text-en]
      
      #v(0.3em)
      #text(size: 7pt, fill: rgb("#666666"))[#caption-en]
    ]
  )
  
  // Inclusion du contenu additionnel si besoin
  body
}

#import sys: inputs
#let d = inputs.data

#cartel(
  author-fr: d.at("Auteur"),
  title-fr: d.at("Titre"),
  date-fr: d.at("Date"),
  text-fr: d.at("explicatif") != none
    ? (d.at("explicatif"),)
    : (),
  caption-fr: d.at("Credit line") != none
    ? (d.at("Credit line"),)
    : (),

  author-en: d.at("Auteur"),
  title-en: d.at("Titre"),
  date-en: d.at("Date"),
  text-en: d.at("traduction") != none
    ? (d.at("traduction"),)
    : (),
  caption-en: (),
)
