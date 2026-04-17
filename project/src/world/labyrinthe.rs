use crate::world::chambre::Chambre;
use crate::world::porte::{CoteMur, Porte};
use rand::Rng;

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
        // Générer contenu de chaque chambre
        for chambre in &mut self.chambres {
            chambre.generer_contenu();
        }

        // Générer des portes entre chambres adjacentes, attachées aux murs
        let mut rng = rand::thread_rng();
        let total = self.chambres.len();

        for i in 0..total {
            let row = i / self.largeur;
            let col = i % self.largeur;

            // Voisin à droite
            if col + 1 < self.largeur {
                let dest = i + 1;
                let src_zone = rng.gen_range(0..self.chambres[i].zones.len());
                let dst_zone = rng.gen_range(0..self.chambres[dest].zones.len());

                let porte = Porte::nouveau(dest, dst_zone, CoteMur::Est);
                let porte_retour = Porte::nouveau(i, src_zone, CoteMur::Est.oppose());

                // Mutably borrow distinct slices to modify both chambres
                let (left, right) = self.chambres.split_at_mut(dest);
                let src_ch = &mut left[i];
                let dst_ch = &mut right[0];

                src_ch.ajouter_porte(porte);
                dst_ch.ajouter_porte(porte_retour);
            }

            // Voisin en bas
            if row + 1 < self.hauteur {
                let dest = i + self.largeur;
                let src_zone = rng.gen_range(0..self.chambres[i].zones.len());
                let dst_zone = rng.gen_range(0..self.chambres[dest].zones.len());

                let porte = Porte::nouveau(dest, dst_zone, CoteMur::Sud);
                let porte_retour = Porte::nouveau(i, src_zone, CoteMur::Sud.oppose());

                let (left, right) = self.chambres.split_at_mut(dest);
                let src_ch = &mut left[i];
                let dst_ch = &mut right[0];

                src_ch.ajouter_porte(porte);
                dst_ch.ajouter_porte(porte_retour);
            }
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