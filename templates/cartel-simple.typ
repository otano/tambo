// ============================================================
// CARTEL — 150 × 110 mm
// Police : Inter
// ============================================================

#set page(
  width: 150mm,
  height: 110mm,
  margin: 5mm,
  fill: rgb("#050A2D"),
)

// ------------------------------------------------------------
// PARAMÈTRES
// ------------------------------------------------------------

#let blanc = rgb("#FFFFFF")
#let gris = rgb("#77798B")

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
    size: 14pt,
    weight: "regular",
    fill: blanc,
  )[ #Auteur ]

  v(5.2mm)

  // ----------------------------------------------------------
  // TITRE FRANÇAIS
  // ----------------------------------------------------------
 set par(leading:0.92em,)
  text(
    font: "Inter",
    size: 14pt,
    weight: "bold",
    style: "italic",

    fill: blanc,
  )[ #titre_fr ]

  v(5.2mm)

  // ----------------------------------------------------------
  // TITRE ANGLAIS
  // ----------------------------------------------------------
 set par(leading:0.98em,)
  text(
    font: "Inter",
    size: 14pt,
    weight: "regular",
    style: "italic",
    fill: blanc,
  )[ #titre_en ]

  v(5.2mm)

  // ----------------------------------------------------------
  // ANNÉE
  // ----------------------------------------------------------

  text(
    font: "Inter",
    size: 5mm,
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
      [
         set par(leading:0.98em,)
         
        #text(
          font: "Inter",
          size: 3.1mm,
          weight: "regular",
          
          fill: blanc,
        )[
          #Technique \
          #Preteur \
          #Inventaire
        ]
      ],
    ),
  )

  // ----------------------------------------------------------
  // CRÉDIT — BAS DROITE
  // ----------------------------------------------------------

  place(
    bottom + right,
    dx: 0mm,
    dy: 0mm,
    text(
      font: "Inter",
      size: 2.8mm,
      weight: "regular",
      fill: gris,
    )[
      #Credit
    ],
  )
}

 // ----------------------------------------------------------
  // contenu externe attention le bloc Exemple doit etre commenté
  // ----------------------------------------------------------



#import sys: inputs
#let d = inputs.data

#cartel(
  Auteur: d.at("Auteur"),
  Date: d.at("Date"),
  titre_fr: d.at("Titre"),
  titre_en: d.at("Titre"),
  Technique: if d.at("Technique") != none { d.at("Technique") } else { "" },
  Preteur: if d.at("Preteur") != none { d.at("Preteur") } else { "" },
  Inventaire: if d.at("Inventaire") != none { d.at("Inventaire") } else { "" },
  Credit: if d.at("Credit") != none { d.at("Credit") } else { "" },
)



/*

// ============================================================
// EXEMPLE Attention le bloc Contenu externe doit etre commenté
// ============================================================

#cartel(
  Auteur: "Fernand Cuville",
  Date: "1919",

  titre_fr: [
    Le cénotaphe dédié aux morts pour la
    patrie sous l'Arc de triomphe pendant
    la veillée funèbre du 13 juillet 1919
  ],

  titre_en: [
    The cenotaph dedicated to those who died for
    their country, under the Arc de Triomphe during
    the funeral vigil of July13, 1919
  ],

  Technique: "Autochrome positif verre, Reproduction",
  Preteur: "Musée départemental Albert-Kahn Département des Hauts-de-Seine",
  Inventaire: "A17902",
  Credit: "© Roger Schall - Schall Collection",
)
*/