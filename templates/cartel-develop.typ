// version 260804-16:53


#let capitalize(s) = {
  if s == none or s == "" {
    ""
  } else {
    upper(s.slice(0, 1)) + s.slice(1)
  }
}


// Définition de la fonction modèle
#let blanc = rgb("#FFFFFF")
#let gris = rgb("#bbb")

#let cartel(
  Auteur: "",
  Titre_fr: "",
  Titre_en: "",
  Date: "",
  Desc_fr: [],
  Desc_en: [],
  Credit: [],
  Technique:[],
  Preteur:[],
  Inventaire: ""
) = {
  // Configuration globale de la page
  set page(
    width: 150mm,
    height: 250mm,
    margin: 10mm,
    fill: rgb("16162C")
  )

  // Police et couleur par défaut
  set text(
    font: "Inter",
    size: 10pt,
    lang: "fr",
    fill: white
  )

  // ----------------------------------------------------------
  // ARTISTE
  // ----------------------------------------------------------

 text(
    font: "Inter",
    size: 16pt,
    weight: "regular",
    fill: blanc,
  )[ #Auteur ]

  // ----------------------------------------------------------
  // TITRE FRANÇAIS
  // ----------------------------------------------------------
  
 set par(leading:0.5em,)
  text(
    font: "Inter",
    size: 19pt,
    weight: 900,
    style: "italic",
    tracking: 0.01em,
    fill: blanc,
  )[ #Titre_fr ]
  
  v(0.1mm)

  // ----------------------------------------------------------
  // TITRE ANGLAIS
  // ----------------------------------------------------------
  set par(leading:0.5em,)
  text(
    font: "Inter",
    size: 16pt,
    weight: 500,
    style: "italic",
    tracking: -0.1pt,
    fill: blanc,
  )[ #Titre_en ]

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
  // DESCRIPTION FRANCAISE
  // ----------------------------------------------------------
  set par(
    justify: false,
    leading: 0.45em,
    
  )

  text(
    size: 14pt,
    weight:500,
    tracking: -0.1pt,

  )[#Desc_fr]
  v(1mm)

  // ----------------------------------------------------------
  // DESCRIPTION ANGLAISE
  // ----------------------------------------------------------
    set par(
    justify: false,
    leading: 0.55em
  )
  text(
    size: 12pt,
        weight:500,
    tracking: -0.1pt,
    style: "italic",
    lang: "en"
  )[#Desc_en]

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
  Titre_fr: d.at("Titre"),
  Titre_en: d.at("TitreEN"),
  Date: d.at("Date"),
  Desc_fr: if d.at("Explicatif") != none { d.at("Explicatif") } else { "" },
  Desc_en: if d.at("Traduction") != none { d.at("Traduction") } else { "" },
  Credit: if d.at("Credit") != none { d.at("Credit") } else { "" },
  Inventaire: if d.at("Inventaire") != none { d.at("Inventaire") } else { "" },
  Preteur: if d.at("Preteur") != none { d.at("Preteur") } else { "" },
  Technique: if d.at("Technique") != none { d.at("Technique") } else { "" },
)
