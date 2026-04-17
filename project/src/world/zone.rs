use crate::entities::ennemi::Ennemi;
use crate::entities::objet::Objet;

#[derive(Debug, Clone)]
pub struct Zone {
    pub id: usize,
    pub nom: String,
    pub description: String,
    pub est_visitee: bool,
    pub est_occupee: bool,
    pub ennemi: Option<Ennemi>,
    pub objet: Option<Objet>,
}

impl Zone {
    pub fn nouveau(id: usize) -> Self {
        Zone {
            id,
            nom: format!("Zone {}", id + 1),
            description: "Une zone sombre du labyrinthe...".to_string(),
            est_visitee: false,
            est_occupee: false,
            ennemi: None,
            objet: None,
        }
    }

    pub fn ajouter_ennemi(&mut self, ennemi: Ennemi) {
        self.ennemi = Some(ennemi);
        self.est_occupee = true;
    }

    pub fn ajouter_objet(&mut self, objet: Objet) {
        self.objet = Some(objet);
    }

    pub fn afficher_contenu(&self, detaille: bool) {
        println!("\n {}", self.nom);
        println!("   {}\n", self.description);

        if let Some(ennemi) = &self.ennemi {
            if !ennemi.est_vaincu {
                println!("  Un {} vous fait face !", ennemi.nom);
                if detaille {
                    ennemi.afficher();
                }
            }
        }

        if let Some(objet) = &self.objet {
            if detaille {
                println!("  {} est ici : {}", objet.nom, objet.description);
            } else {
                println!("  Quelque chose brille dans un coin...");
            }
        }

        if self.ennemi.is_none() && self.objet.is_none() {
            println!("   La zone semble vide...");
        }

        println!();
    }
}