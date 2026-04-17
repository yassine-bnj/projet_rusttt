use crate::world::chambre::Chambre;
use crate::world::porte::Porte;
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

        // Générer des portes entre chambres adjacentes (droite et bas)
        let mut rng = rand::thread_rng();
        let total = self.chambres.len();

        for i in 0..total {
            let row = i / self.largeur;
            let col = i % self.largeur;

            // Voisin à droite
            if col + 1 < self.largeur {
                let dest = i + 1;
                let src_zone = Self::zone_libre_pour_porte(&self.chambres[i], &mut rng);
                let dst_zone = Self::zone_libre_pour_porte(&self.chambres[dest], &mut rng);

                let (src_zone, dst_zone) = match (src_zone, dst_zone) {
                    (Some(src_zone), Some(dst_zone)) => (src_zone, dst_zone),
                    _ => continue,
                };

                let porte = Porte::nouveau(dest, dst_zone);

                // Mutably borrow distinct slices to modify both chambres
                let (left, right) = self.chambres.split_at_mut(dest);
                let src_ch = &mut left[i];
                let dst_ch = &mut right[0];

                src_ch.portes.push(porte.clone());
                dst_ch.portes.push(Porte::nouveau(i, src_zone));

                if let Some(zone) = src_ch.get_zone_mut(src_zone) {
                    zone.porte = Some(porte.clone());
                }
                if let Some(zone) = dst_ch.get_zone_mut(dst_zone) {
                    zone.porte = Some(Porte::nouveau(i, src_zone));
                }
            }

            // Voisin en bas
            if row + 1 < self.hauteur {
                let dest = i + self.largeur;
                let src_zone = Self::zone_libre_pour_porte(&self.chambres[i], &mut rng);
                let dst_zone = Self::zone_libre_pour_porte(&self.chambres[dest], &mut rng);

                let (src_zone, dst_zone) = match (src_zone, dst_zone) {
                    (Some(src_zone), Some(dst_zone)) => (src_zone, dst_zone),
                    _ => continue,
                };

                let porte = Porte::nouveau(dest, dst_zone);

                let (left, right) = self.chambres.split_at_mut(dest);
                let src_ch = &mut left[i];
                let dst_ch = &mut right[0];

                src_ch.portes.push(porte.clone());
                dst_ch.portes.push(Porte::nouveau(i, src_zone));

                if let Some(zone) = src_ch.get_zone_mut(src_zone) {
                    zone.porte = Some(porte.clone());
                }
                if let Some(zone) = dst_ch.get_zone_mut(dst_zone) {
                    zone.porte = Some(Porte::nouveau(i, src_zone));
                }
            }
        }
    }

    fn zone_libre_pour_porte(chambre: &Chambre, rng: &mut impl Rng) -> Option<usize> {
        let zones_libres: Vec<usize> = chambre
            .zones
            .iter()
            .enumerate()
            .filter_map(|(index, zone)| if zone.porte.is_none() { Some(index) } else { None })
            .collect();

        if zones_libres.is_empty() {
            None
        } else {
            Some(zones_libres[rng.gen_range(0..zones_libres.len())])
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