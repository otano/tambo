
// Définition de la fonction modèle
#let cartel(
  Auteur: "",
  Titre_fr: "",
  Titre_en: "",
  Date: "",
  Desc_fr: [],
  Desc_en: [],
  Credit: [],
  Inventaire: ""
) = {
  // Configuration globale de la page
  set page(
    width: 15cm,
    height: 25cm,
    margin: 1.27cm,
    fill: rgb("16162C")
  )

  // Police et couleur par défaut
  set text(
    font: "Inter",
    size: 10pt,
    lang: "fr",
    fill: white
  )

  // Auteur
  text(size: 12pt)[#Auteur]
  v(0.01em)

  // Titre français
  text(
    size: 14pt,
    weight: 800,
    style: "italic",
    lang: "fr"
  )[#Titre_fr]
  v(0.3em)

  // Titre anglais
  text(
    size: 14pt,
    style: "italic",
    lang: "en"
  )[#Titre_en]
  v(0.4em)

  // Année
  text(size: 14pt)[#Date]
  v(1em)

  // Description française
  set par(
    justify: true,
    leading: 0.65em
  )

  text(size: 12pt)[#Desc_fr]
  v(1em)

  // Description anglaise
  text(
    size: 12pt,
    tracking: -0.2pt,
    style: "italic",
    lang: "en"
  )[#Desc_en]

  // Le bloc est placé au bas de la page
  place(
    bottom + left,
    
      [
        #text(size: 9pt)[#Credit],#text(font: "Inter",
          size: 9pt
        )[#Inventaire]
      ]
    )
  
}

/////////
//appel externe
////////

#import sys: inputs
#let d = inputs.data

#cartel(
  Auteur: d.at("Auteur"),
  Titre_fr: d.at("Titre"),
  Titre_en: d.at("Titre"),
  Date: d.at("Date"),
  Desc_fr: if d.at("Explicatif") != none { d.at("Explicatif") } else { "" },
  Desc_en: if d.at("Traduction") != none { d.at("Traduction") } else { "" },
  Credit: if d.at("Credit") != none { d.at("Credit") } else { "" },
  Inventaire: if d.at("Inventaire") != none { d.at("Inventaire") } else { "" },
)


/////////
//Appel du modèle
/////////
/*
#cartel(
  Auteur: "Fernand Cuville",

  Titre_fr: [
    Le cénotaphe dédié aux morts pour la patrie sous l'Arc de triomphe
    pendant la veillée funèbre du 13 juillet 1919
  ],

  Titre_en: [
    The cenotaph dedicated to those who died for their country,
    under the Arc de Triomphe during the funeral vigil of July 13, 1919
  ],

  Date: "1919",

  Desc_fr: [
    Une veillée funèbre devant un cénotaphe — un grand cercueil vide en
    plâtre doré dédié « À nos morts » placé sous l'Arc de triomphe —
    précède la journée du 14 juillet 1919, qui voit le défilé militaire
    de la Victoire passer sous l'Arc. Un système d'illuminations sous la
    voûte et à son pourtour n'empêche pas l'édifice d'être plongé dans
    une pénombre générale d'où seul émerge le cénotaphe.

    La fonction funéraire du monument semble prendre le pas sur sa
    fonction triomphale, en raison de la mort des millions de combattants
    pendant la Première Guerre mondiale qu'on honore à l'Arc de triomphe.
    La tombe du Soldat inconnu n'étant installée qu'en 1920.
  ],

  Desc_en: [
    A funeral vigil in front of a cenotaph—a large, empty, gilded plaster
    coffin dedicated "To Our Dead" placed beneath the Arc de Triomphe—
    preceded July 14, 1919, the day of the Victory military parade.
    Despite a lighting system installed under the vault and around its
    perimeter, the monument remained plunged in a general gloom from which
    only the cenotaph emerged.

    The funerary function of the monument seemed to take precedence over
    its triumphal purpose, honoring the millions of soldiers killed during
    the First World War. The Tomb of the Unknown Soldier was not installed
    until 1920.
  ],

 Credit: [
    Autochrome positif verre, Reproduction, Musée départemental Albert-Kahn \
    Département des Hauts-de-Seine
  ],

  Inventaire: [
    A17902
  ]
)
 */
