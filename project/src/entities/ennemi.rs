#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeEnnemi {
    OmbreErrante,
    GardienDePierre,
    SpectreEnigmatique,
}

#[derive(Debug, Clone)]
pub struct Ennemi {
    pub nom: String,
    pub type_ennemi: TypeEnnemi,
    pub pv: i32,
    pub pv_max: i32,
    pub degats: i32,
    pub est_vaincu: bool,
    pub question: Option<String>,
    pub reponses: Option<Vec<String>>,
    pub reponse_correcte: Option<usize>,
}

impl Ennemi {
    pub fn nouveau(type_ennemi: TypeEnnemi) -> Self {
        match type_ennemi {
            TypeEnnemi::OmbreErrante => Ennemi {
                nom: "Ombre Errante".to_string(),
                type_ennemi,
                pv: 10,
                pv_max: 10,
                degats: 3,
                est_vaincu: false,
                question: None,
                reponses: None,
                reponse_correcte: None,
            },
            TypeEnnemi::GardienDePierre => Ennemi {
                nom: "Gardien de Pierre".to_string(),
                type_ennemi,
                pv: 20,
                pv_max: 20,
                degats: 5,
                est_vaincu: false,
                question: None,
                reponses: None,
                reponse_correcte: None,
            },
            TypeEnnemi::SpectreEnigmatique => Ennemi {
                nom: "Spectre Énigmatique".to_string(),
                type_ennemi,
                pv: 999,
                pv_max: 999,
                degats: 8,
                est_vaincu: false,
                question: Some("Quelle est la clé de l'âme perdue ?".to_string()),
                reponses: Some(vec![
                    "L'oubli".to_string(),
                    "Le regret".to_string(),
                    "La peur".to_string(),
                ]),
                reponse_correcte: Some(1),
            },
        }
    }

    pub fn attaquer(&self) -> i32 {
        self.degats
    }

    pub fn subir_degats(&mut self, degats: i32) -> bool {
        if self.type_ennemi == TypeEnnemi::SpectreEnigmatique {
            return false;
        }
        
        self.pv -= degats;
        if self.pv <= 0 {
            self.pv = 0;
            self.est_vaincu = true;
            return true;
        }
        false
    }

    pub fn afficher(&self) {
        println!("\n  ────────────────────────────");
        println!("  ENNEMI: {}", self.nom);
        if self.type_ennemi != TypeEnnemi::SpectreEnigmatique {
            println!("  PV: {}/{}", self.pv, self.pv_max);
        }
        println!("  Dégâts: -{} PV", self.degats);
        println!("  ────────────────────────────\n");
    }

    pub fn afficher_enigme(&self) {
        if let Some(question) = &self.question {
            println!("\n LE SPECTRE PARLE :");
            println!("   \"{}\"", question);
            println!("\n   Choisissez votre réponse :");
            if let Some(reponses) = &self.reponses {
                for (i, reponse) in reponses.iter().enumerate() {
                    println!("   [{}] {}", i + 1, reponse);
                }
            }
        }
    }

    pub fn verifier_reponse(&self, choix: usize) -> bool {
        if let Some(correct) = self.reponse_correcte {
            choix == correct
        } else {
            false
        }
    }
}