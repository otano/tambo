
// version 260804-16:53

#set page(
  width: 150mm,
  height: 110mm,
  margin: 10mm,
  fill: rgb("#02092B"),
)

#let capitalize(s) = {
  if s == none or s == "" {
    ""
  } else {
    upper(s.slice(0, 1)) + s.slice(1)
  }
}
// ------------------------------------------------------------
// PARAMÈTRES
// ------------------------------------------------------------

#let blanc = rgb("#FFFFFF")
#let gris = rgb("#bbb")

#let cartel(
  Auteur: "",
  Date: "",
  titre_fr: "",
  titre_en: "",
  Technique: "",
  Preteur: "",
  Inventaire: "",
  Credit: "",
) = {

  // ----------------------------------------------------------
  // ARTISTE
  // ----------------------------------------------------------

  text(
    font: "Inter",
    size: 16pt,
    weight: "regular",
    fill: blanc,
  )[ #Auteur ]

  v(-2.2mm)

  // ----------------------------------------------------------
  // TITRE FRANÇAIS
  // ----------------------------------------------------------
 set par(leading:0.5em,)
  text(
    font: "Inter",
    size: 19pt,
    weight: 900,
    style: "italic",
    tracking: -0.1pt,
    fill: blanc,
  )[ #titre_fr ]

  v(0.1mm)

  // ----------------------------------------------------------
  // TITRE ANGLAIS
  // ----------------------------------------------------------
 set par(leading:0.6em,)
  text(
    font: "Inter",
    size: 16pt,
    weight: 500,
    style: "italic",
    tracking: 0pt,
    fill: blanc,
  )[ #titre_en ]

  v(-.6mm)

  // ----------------------------------------------------------
  // DATE
  // ----------------------------------------------------------

  text(
    font: "Inter",
    size: 14pt,
    weight: "regular",
    fill: blanc,
  )[ #Date ]

  // ----------------------------------------------------------
  // BLOC BAS
  // ----------------------------------------------------------


 
  place(
    bottom + left,
    dx: 0mm,
    dy: 0mm,
    block(
      width: 100%,
      text(
          font: "Inter",
          size: 9pt,
          weight: 400,
          
          fill: blanc,
        )[
          #if Technique != none or Technique != "" [
          #capitalize(Technique) \
          ]
          #if Preteur != none or Preteur != "" [
          #capitalize(Preteur) \
           ]
          #if Inventaire != "" and Credit !="" [
          #capitalize(Inventaire) / #capitalize(Credit)
          ] else if Inventaire != "" [
          #capitalize(Inventaire)
        ] else if Credit != "" [
          #capitalize(Credit)
          ] 
         
      ]
        )
       )



}



#import sys: inputs
#let d = inputs.data


#cartel(
  Auteur: d.at("Auteur"),
  Date: d.at("Date"),
  titre_fr: d.at("Titre"),
  titre_en: d.at("TitreEN"),
  Technique: if d.at("Technique") != none { d.at("Technique") } else { "" },
  Preteur: if d.at("Preteur") != none { d.at("Preteur") } else { "" },
  Inventaire: if d.at("Inventaire") != none { d.at("Inventaire") } else { "" },
  Credit: if d.at("Credit") != none { d.at("Credit") } else { "" },
)