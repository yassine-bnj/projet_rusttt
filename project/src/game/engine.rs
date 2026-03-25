use crate::entities::personnage::Personnage;
use crate::world::labyrinthe::Labyrinthe;

pub struct GameEngine {
    pub personnage: Personnage,
    pub labyrinthe: Labyrinthe,
    pub tour: i32,
    pub partie_terminee: bool,
}

impl GameEngine {
    pub fn nouveau(personnage: Personnage, labyrinthe: Labyrinthe) -> Self {
        GameEngine {
            personnage,
            labyrinthe,
            tour: 1,
            partie_terminee: false,
        }
    }

    pub fn demarrer(&mut self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║   LE LABYRINTHE DES ÂMES PERDUES          ║");
        println!("╚════════════════════════════════════════════╝\n");

        self.labyrinthe.afficher_carte();
        self.personnage.afficher_statut();

        while !self.partie_terminee && self.personnage.est_vivant {
            self.tour_actuel();
        }

        self.fin_partie();
    }

    fn tour_actuel(&mut self) {
        println!("\n━━━ TOUR {} ━━━\n", self.tour);
        
        let chambre_id = self.personnage.chambre_actuelle;
        let zone_id = self.personnage.zone_actuelle;

        if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
            if let Some(zone) = chambre.get_zone(zone_id) {
                zone.afficher_contenu(false);
            }
        }

        self.personnage.afficher_statut();
        self.gerer_input();
        self.tour += 1;
    }

    fn gerer_input(&mut self) {
        println!("Commands: [n/s/e/o] déplacer | [r] regarder | [i] inventaire | [q] quitter");
        print!("> ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "n" | "s" | "e" | "o" => self.deplacer(),
            "r" => self.regarder(),
            "i" => self.afficher_inventaire(),
            "q" => self.partie_terminee = true,
            _ => println!("Commande inconnue."),
        }
    }

    fn deplacer(&mut self) {
        if self.personnage.points_action <= 0 {
            println!(" Pas assez de points d'action !");
            return;
        }

        println!(" Vous vous déplacez...");
        self.personnage.points_action -= 1;
        self.personnage.zone_actuelle = (self.personnage.zone_actuelle + 1) % 12;
    }

    fn regarder(&mut self) {
        println!(" Vous examinez les alentours...");
        let chambre_id = self.personnage.chambre_actuelle;
        let zone_id = self.personnage.zone_actuelle;

        if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
            if let Some(zone) = chambre.get_zone(zone_id) {
                zone.afficher_contenu(true);
            }
        }
    }

    fn afficher_inventaire(&self) {
        self.personnage.inventaire.afficher();
    }

    fn fin_partie(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        if self.personnage.est_vivant {
            println!("║   PARTIE TERMINÉE                       ║");
        } else {
            println!("║   GAME OVER                               ║");
        }
        println!("║   Tours joués: {:<24}  ║", self.tour);
        println!("╚════════════════════════════════════════════╝\n");
    }
}