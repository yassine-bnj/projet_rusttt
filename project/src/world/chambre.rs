use crate::world::zone::Zone;
use crate::world::porte::Porte;
use crate::entities::ennemi::{Ennemi, TypeEnnemi};
use crate::entities::objet::Objet;

#[derive(Debug, Clone)]
pub struct Chambre {
    pub id: usize,
    pub nom: String,
    pub description: String,
    pub zones: Vec<Zone>,
    pub portes: Vec<Porte>,
    pub est_visitee: bool,
}

impl Chambre {
    pub fn nouveau(id: usize) -> Self {
        let noms = vec![
            "L'Entrée Oubliée",
            "Le Couloir des Murmures",
            "La Salle des Échos",
            "Le Hall des Gardiens",
            "La Chambre Noire",
            "Le Passage Secret",
            "L'Antre du Spectre",
            "Le Sanctuaire",
            "La Sortie Espérée",
        ];

               let nom = if id < noms.len() {
            noms[id].to_string()
        } else {
            format!("Chambre {}", id + 1)
        };

        let mut zones = Vec::new();
        for i in 0..12 {
            zones.push(Zone::nouveau(i));
        }

        Chambre {
            id,
            nom,
            description: "Une pièce mystérieuse du labyrinthe...".to_string(),
            zones,
            portes: Vec::new(),
            est_visitee: false,
        }
    }

    pub fn generer_contenu(&mut self, autoriser_boss: bool) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if autoriser_boss {
            let zone_index = rng.gen_range(0..self.zones.len());
            self.zones[zone_index].ajouter_ennemi(Ennemi::nouveau(TypeEnnemi::SpectreEnigmatique));
            // La dernière chambre a aussi toujours un objet
            let zone_objet = rng.gen_range(0..self.zones.len());
            self.zones[zone_objet].ajouter_objet(Self::objet_aleatoire(&mut rng));
            return;
        }

        // Probabilités augmentées pour éviter les chambres vides
        let a_ennemi = rng.gen_bool(0.70);
        let a_objet  = rng.gen_bool(0.60);

        // Garantir au moins un contenu par chambre
        let forcer_objet = !a_ennemi && !a_objet;

        if a_ennemi {
            let zone_index = rng.gen_range(0..self.zones.len());
            let ennemi_type = if rng.gen_bool(0.5) {
                TypeEnnemi::OmbreErrante
            } else {
                TypeEnnemi::GardienDePierre
            };
            self.zones[zone_index].ajouter_ennemi(Ennemi::nouveau(ennemi_type));
        }

        if a_objet || forcer_objet {
            let zone_index = rng.gen_range(0..self.zones.len());
            self.zones[zone_index].ajouter_objet(Self::objet_aleatoire(&mut rng));
        }
    }

    fn objet_aleatoire(rng: &mut impl rand::Rng) -> Objet {
        match rng.gen_range(0..3) {
            0 => Objet::potion_de_vie(),
            1 => Objet::cle_mystique(),
            _ => Objet::bouclier_spectral(),
        }
    }

    pub fn get_zone(&self, id: usize) -> Option<&Zone> {
        self.zones.get(id)
    }

    pub fn get_zone_mut(&mut self, id: usize) -> Option<&mut Zone> {
        self.zones.get_mut(id)
    }

    pub fn ajouter_porte(&mut self, porte: Porte) {
        self.portes.push(porte);
    }
}
