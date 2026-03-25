use crate::world::chambre::Chambre;

#[derive(Debug, Clone)]
pub struct Labyrinthe {
    pub nom: String,
    pub chambres: Vec<Chambre>,
    pub largeur: usize,
    pub hauteur: usize,
}

impl Labyrinthe {
    pub fn nouveau(nom: &str, largeur: usize, hauteur: usize) -> Self {
        let total_chambres = largeur * hauteur;
        let mut chambres = Vec::with_capacity(total_chambres);
        
        for i in 0..total_chambres {
            chambres.push(Chambre::nouveau(i));
        }

        Labyrinthe {
            nom: nom.to_string(),
            chambres,
            largeur,
            hauteur,
        }
    }

    pub fn generer(&mut self) {
        for chambre in &mut self.chambres {
            chambre.generer_contenu();
        }
    }

    pub fn get_chambre(&self, id: usize) -> Option<&Chambre> {
        self.chambres.get(id)
    }

    pub fn get_chambre_mut(&mut self, id: usize) -> Option<&mut Chambre> {
        self.chambres.get_mut(id)
    }

    pub fn afficher_carte(&self) {
        println!("\n  ────────────────────────────");
        println!("  {}", self.nom);
        println!("  Dimensions: {}x{}", self.largeur, self.hauteur);
        println!("  Total chambres: {}", self.chambres.len());
        println!("  ────────────────────────────\n");
    }
}