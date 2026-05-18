#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeObjet {
    PotionDeVie,
    CleMystique,
    BouclierSpectral,
}

#[derive(Debug, Clone)]
pub struct Objet {
    pub nom: String,
    pub type_objet: TypeObjet,
    pub description: String,
    pub valeur: i32,
    pub utilisations_restantes: i32,
    pub est_utilise: bool,
}

impl Objet {
    pub fn nouveau(nom: &str, type_objet: TypeObjet, description: &str, valeur: i32) -> Self {
        Objet {
            nom: nom.to_string(),
            type_objet,
            description: description.to_string(),
            valeur,
            utilisations_restantes: 1,
            est_utilise: false,
        }
    }

    pub fn potion_de_vie() -> Self {
        Objet::nouveau(
            "Potion de Vie",
            TypeObjet::PotionDeVie,
            "Restaure 5 points de vie",
            10,
        )
    }

    pub fn cle_mystique() -> Self {
        Objet::nouveau(
            "Clé Mystique",
            TypeObjet::CleMystique,
            "Ouvre une porte sans dépenser de points d'action",
            50,
        )
    }

    pub fn bouclier_spectral() -> Self {
        Objet::nouveau(
            "Bouclier Spectral",
            TypeObjet::BouclierSpectral,
            "Annule complètement 1 attaque ennemie",
            75,
        )
    }

    pub fn utiliser(&mut self) -> Option<i32> {
        if self.utilisations_restantes <= 0 || self.est_utilise {
            return None;
        }
        self.utilisations_restantes -= 1;
        if self.utilisations_restantes <= 0 {
            self.est_utilise = true;
        }

        match self.type_objet {
            TypeObjet::PotionDeVie => Some(5),
            TypeObjet::CleMystique => Some(0),
            TypeObjet::BouclierSpectral => Some(0),
        }
    }
}