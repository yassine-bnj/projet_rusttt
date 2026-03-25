use crate::entities::objet::Objet;

#[derive(Debug, Clone)]
pub struct Inventaire {
    pub objets: Vec<Objet>,
    pub capacite_max: usize,
}

impl Inventaire {
    pub fn nouveau(capacite_max: usize) -> Self {
        Inventaire {
            objets: Vec::new(),
            capacite_max,
        }
    }

    pub fn ajouter(&mut self, objet: Objet) -> bool {
        if self.objets.len() < self.capacite_max {
            println!("{} ajouté à l'inventaire", objet.nom);
            self.objets.push(objet);
            true
        } else {
            println!(" Inventaire plein !");
            false
        }
    }

    pub fn afficher(&self) {
        println!("\n INVENTAIRE ({} / {}):", self.objets.len(), self.capacite_max);
        if self.objets.is_empty() {
            println!("   (vide)");
        } else {
            for (i, objet) in self.objets.iter().enumerate() {
                println!("   [{}] {} - {}", i + 1, objet.nom, objet.description);
            }
        }
        println!();
    }
}