use crate::entities::personnage::Personnage;
use crate::world::labyrinthe::Labyrinthe;
use crate::world::chambre::Chambre;
use crate::world::porte::CoteMur;

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

    // pub fn demarrer(&mut self) {
    //     println!("\n╔════════════════════════════════════════════╗");
    //     println!("║   LE LABYRINTHE DES ÂMES PERDUES          ║");
    //     println!("╚════════════════════════════════════════════╝\n");

    //     self.labyrinthe.afficher_carte();
    //     self.personnage.afficher_statut();

    //     while !self.partie_terminee && self.personnage.est_vivant {
    //         self.tour_actuel();
    //     }

    //     self.fin_partie();
    // }
    pub fn demarrer(&mut self) {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║   LE LABYRINTHE DES ÂMES PERDUES          ║");
    println!("╚════════════════════════════════════════════╝\n");

    // ✅ Initialiser la position du joueur (Chambre 0, Zone 0)
    self.personnage.chambre_actuelle = 0;
    self.personnage.zone_actuelle = 0;

    self.labyrinthe.afficher_carte();
    self.personnage.afficher_statut();
    
    // ✅ Afficher la zone de départ et les déplacements possibles
    self.afficher_zone_actuelle();

    while !self.partie_terminee && self.personnage.est_vivant {
        self.tour_actuel();
    }

    self.fin_partie();
}

fn afficher_zone_actuelle(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        if let Some(zone) = chambre.get_zone(zone_id) {
            zone.afficher_contenu(false);
            
            // ✅ Afficher les déplacements possibles
            self.afficher_deplacements_possibles(chambre, zone_id);
        }
    }
}

fn afficher_chambre_matrice(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        println!("\n🏰 ──────────────────────────────────────────────");
        println!("🏰  CHAMBRE {}: {}", chambre_id + 1, chambre.nom);
        println!("🏰 ───────────────────────────────────────────────\n");
        
        // Grille 4 colonnes x 3 rangées = 12 zones
        // [0]  [1]  [2]  [3]
        // [4]  [5]  [6]  [7]
        // [8]  [9]  [10] [11]
        
        println!("     ┌──────┬────────────┬──────");
        
        for ligne in 0..3 {
            print!("     │");
            
            for colonne in 0..4 {
                let zone_id_calc = ligne * 4 + colonne;
                
                if let Some(zone) = chambre.get_zone(zone_id_calc) {
                    // Déterminer le symbole à afficher
                    let symbole = if zone_id_calc == zone_id {
                        "  👤  "  // Position du joueur
                    } else if let Some(ennemi) = &zone.ennemi {
                        if !ennemi.est_vaincu {
                            "  ⚔️  "  // Ennemi
                        } else {
                            "  💀  "  // Ennemi vaincu
                        }
                    } else if let Some(objet) = &zone.objet {
                        match objet.tipe {
                            crate::entities::objet::TypeObjet::PotionDeVie => "  🧪  ",
                            crate::entities::objet::TypeObjet::CleMystique => "  🔑  ",
                            crate::entities::objet::TypeObjet::BouclierSpectral => "  🛡️  ",
                        }
                    } else if zone.est_visitee {
                        "  ··  "  // Zone visitée (vide)
                    } else {
                        "  ░░  "  // Zone non explorée
                    };
                    
                    print!("{}│", symbole);
                }
            }
            
            println!();
            
            if ligne < 2 {
                println!("     ├──────┼────────────┼──────");
            }
        }
        
        println!("     └──────┴──────┴──────┴──────┘");
        
        // Légende
        println!("\n📍 LÉGENDE :");
        println!("   👤 = Votre position  │  ⚔️ = Ennemi  │  💀 = Ennemi vaincu");
        println!("   🧪 = Potion  │  🔑 = Clé  │  🛡️ = Bouclier");
        println!("   ·· = Visitée  │  ░░ = Non explorée");
        
        // Directions possibles
        println!("\n🚶 DÉPLACEMENTS :");
        let colonne = zone_id % 4;
        let ligne = zone_id / 4;
        
        let mut directions = Vec::new();
        
        if ligne > 0 { directions.push("[N] Nord"); }
        if ligne < 2 { directions.push("[S] Sud"); }
        if colonne > 0 { directions.push("[O] Ouest"); }
        if colonne < 3 { directions.push("[E] Est"); }
        
        // Vérifier porte
        if !chambre.portes.is_empty() {
            directions.push("[P] Porte");
        }
        
        println!("   {}", directions.join("  │  "));
        println!();
    }
}

// fn afficher_deplacements_possibles(&self, chambre: &Chambre, zone_actuelle: usize) {
fn afficher_deplacements_possibles(&self, chambre: &Chambre, zone_actuelle: usize) {
    println!("\n🚶 DÉPLACEMENTS POSSIBLES :");
    println!("   ┌─────────────────────────────────┐");

    let colonne = zone_actuelle % 4;
    let ligne = zone_actuelle / 4;

    // Nord
    if ligne > 0 {
        let zone_nord = zone_actuelle - 4;
        if let Some(z) = chambre.get_zone(zone_nord) {
            if z.est_occupee && z.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                println!("   │  [N] Nord  → Zone {} ⚔️  (Ennemi)  │", zone_nord + 1);
            } else {
                println!("   │  [N] Nord  → Zone {}              │", zone_nord + 1);
            }
        }
    } else {
        println!("   │  [N] Nord  → 🚫 Mur                   │");
    }

    // Sud
    if ligne < 2 {
        let zone_sud = zone_actuelle + 4;
        if let Some(z) = chambre.get_zone(zone_sud) {
            if z.est_occupee && z.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                println!("   │  [S] Sud   → Zone {} ⚔️  (Ennemi)  │", zone_sud + 1);
            } else {
                println!("   │  [S] Sud   → Zone {}              │", zone_sud + 1);
            }
        }
    } else {
        println!("   │  [S] Sud   → 🚫 Mur                   │");
    }

    // Ouest
    if colonne > 0 {
        let zone_ouest = zone_actuelle - 1;
        if let Some(z) = chambre.get_zone(zone_ouest) {
            if z.est_occupee && z.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                println!("   │  [O] Ouest → Zone {} ⚔️  (Ennemi)  │", zone_ouest + 1);
            } else {
                println!("   │  [O] Ouest → Zone {}              │", zone_ouest + 1);
            }
        }
    } else {
        println!("   │  [O] Ouest → 🚫 Mur                   │");
    }

    // Est
    if colonne < 3 {
        let zone_est = zone_actuelle + 1;
        if let Some(z) = chambre.get_zone(zone_est) {
            if z.est_occupee && z.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                println!("   │  [E] Est   → Zone {} ⚔️  (Ennemi)  │", zone_est + 1);
            } else {
                println!("   │  [E] Est   → Zone {}              │", zone_est + 1);
            }
        }
    } else {
        println!("   │  [E] Est   → 🚫 Mur                   │");
    }

    // Afficher les portes vers autres chambres
    for porte in &chambre.portes {
        println!("   │  [P] Porte → Chambre {} (Coût: {} PA, {} PV) │", 
                 porte.chambre_destination + 1, porte.cout_pa, porte.cout_pv);
    }

    println!("   └─────────────────────────────────┘\n");
}

    // fn tour_actuel(&mut self) {
    //     println!("\n━━━ TOUR {} ━━━\n", self.tour);
        
    //     let chambre_id = self.personnage.chambre_actuelle;
    //     let zone_id = self.personnage.zone_actuelle;

    //     if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
    //         if let Some(zone) = chambre.get_zone(zone_id) {
    //             zone.afficher_contenu(false);
    //         }
    //     }

    //     self.personnage.afficher_statut();
    //     self.gerer_input();
    //     self.tour += 1;
    // }

    fn tour_actuel(&mut self) {
    println!("\n━━━ TOUR {} ━━━\n", self.tour);
    
    // ✅ Afficher la matrice de la chambre
    self.afficher_chambre_matrice();
    
    // ✅ Afficher le statut du joueur
    self.personnage.afficher_statut();
    
    // ✅ Afficher le contenu détaillé de la zone actuelle
    self.afficher_contenu_zone();
    
    self.gerer_input();
    self.tour += 1;
}

fn afficher_contenu_zone(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        if let Some(zone) = chambre.get_zone(zone_id) {
            // Si ennemi présent et non vaincu
            if let Some(ennemi) = &zone.ennemi {
                if !ennemi.est_vaincu {
                    println!("\n⚠️  ATTENTION ! Un {} vous fait face !", ennemi.nom);
                    println!("   Tapez [c] pour combattre ou [f] pour fuir");
                }
            }
            
            // Si objet présent
            if let Some(objet) = &zone.objet {
                println!("\n✨  Vous voyez : {} - {}", objet.nom, objet.description);
                println!("   Tapez [ramasser] pour le prendre");
            }

                if !chambre.portes.is_empty() {
                    println!("\n🚪  Portes disponibles sur les murs :");
                    for porte in &chambre.portes {
                        println!(
                            "   - {} → Chambre {} (Coût: {} PA, {} PV)",
                            porte.cote_mur.etiquette(),
                            porte.chambre_destination + 1,
                            porte.cout_pa,
                            porte.cout_pv
                        );
                    }
                    println!("   Tapez [p] pour choisir une porte");
                }
        }
    }
}


fn gerer_input(&mut self) {
    println!("\n┌────────────────────────────────────────────────┐");
    println!("│  [n/s/e/o] Déplacer  │  [p] Porte            │");
    println!("│  [r] Regarder        │  [i] Inventaire       │");
    println!("│  [c] Combattre       │  [f] Fuir             │");
    println!("│  [ramasser] Objet    │  [q] Quitter          │");
    println!("└────────────────────────────────────────────────┘");
    print!("\n> ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "n" | "s" | "e" | "o" => self.deplacer(input.as_str()),
        "p" => self.traverser_porte(),
        "r" => self.regarder(),
        "i" => self.afficher_inventaire(),
        "c" => self.combattre(),
        "f" => self.fuir(),
        "ramasser" => self.ramasser_objet(),
        "q" => self.partie_terminee = true,
        _ => println!("Commande inconnue."),
    }
}

fn combattre(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre_mut(chambre_id) {
        if let Some(zone) = chambre.get_zone_mut(zone_id) {
            if let Some(ennemi) = &mut zone.ennemi {
                if !ennemi.est_vaincu {
                    println!("\n⚔️  COMBAT CONTRE {}", ennemi.nom.to_uppercase());
                    println!("─────────────────────────────────────");
                    
                    // Joueur attaque
                    let degats_joueur = 5; // À ajuster selon tes règles
                    ennemi.subir_degats(degats_joueur);
                    println!("💥 Vous infligez {} dégâts !", degats_joueur);
                    
                    if ennemi.est_vaincu {
                        println!("✅ Ennemi vaincu !");
                        zone.est_occupee = false;
                    } else {
                        // Ennemi contre-attaque
                        let degats_ennemi = ennemi.attaquer();
                        
                        // Vérifier si bouclier spectral
                        let bouclier_actif = self.personnage.inventaire.objets
                            .iter_mut()
                            .find(|o| o.tipe == crate::entities::objet::TypeObjet::BouclierSpectral 
                                     && !o.est_utilise);
                        
                        if let Some(bouclier) = bouclier_actif {
                            bouclier.utiliser();
                            println!("🛡️  Bouclier Spectral activé ! Attaque annulée !");
                        } else {
                            self.personnage.subir_degats(degats_ennemi);
                            println!("💥 {} vous inflige {} dégâts !", ennemi.nom, degats_ennemi);
                        }
                    }
                    
                    println!("─────────────────────────────────────");
                    println!("❤️  Vos PV: {}/{}", self.personnage.pv, self.personnage.pv_max);
                    if ennemi.tipe != crate::entities::ennemi::TypeEnnemi::SpectreEnigmatique {
                        println!("🗡️  PV Ennemi: {}/{}", ennemi.pv, ennemi.pv_max);
                    }
                }
            } else {
                println!("❌ Il n'y a pas d'ennemi dans cette zone !");
            }
        }
    }
}

fn fuir(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        if let Some(zone) = chambre.get_zone(zone_id) {
            if zone.ennemi.is_some() {
                // 50% de chance de fuir
                use rand::Rng;
                let mut rng = rand::thread_rng();
                
                if rng.gen_bool(0.5) {
                    println!("✅ Vous avez réussi à fuir !");
                    // Reculer d'une zone
                    if zone_id > 0 {
                        self.personnage.zone_actuelle = zone_id - 1;
                    }
                } else {
                    println!("❌ Fuite échouée ! L'ennemi vous attaque !");
                    if let Some(ennemi) = &zone.ennemi {
                        let degats = ennemi.attaquer();
                        self.personnage.subir_degats(degats);
                        println!("💥 Vous subissez {} dégâts !", degats);
                    }
                }
            } else {
                println!("❌ Il n'y a pas d'ennemi à fuir !");
            }
        }
    }
}

fn ramasser_objet(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre_mut(chambre_id) {
        if let Some(zone) = chambre.get_zone_mut(zone_id) {
            if let Some(objet) = zone.objet.take() {
                if self.personnage.ajouter_objet(objet.clone()) {
                    println!("✅ {} ramassé !", objet.nom);
                } else {
                    zone.objet = Some(objet); // Remettre l'objet si inventaire plein
                }
            } else {
                println!("❌ Il n'y a pas d'objet à ramasser ici !");
            }
        }
    }
}

    // fn gerer_input(&mut self) {
    //     println!("Commands: [n/s/e/o] déplacer | [r] regarder | [i] inventaire | [q] quitter");
    //     print!("> ");
        
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let input = input.trim().to_lowercase();

    //     match input.as_str() {
    //         "n" | "s" | "e" | "o" => self.deplacer(),
    //         "r" => self.regarder(),
    //         "i" => self.afficher_inventaire(),
    //         "q" => self.partie_terminee = true,
    //         _ => println!("Commande inconnue."),
    //     }
    // }

    // fn deplacer(&mut self) {
    //     if self.personnage.points_action <= 0 {
    //         println!(" Pas assez de points d'action !");
    //         return;
    //     }

    //     println!(" Vous vous déplacez...");
    //     self.personnage.points_action -= 1;
    //     self.personnage.zone_actuelle = (self.personnage.zone_actuelle + 1) % 12;
    // }

    fn deplacer(&mut self, direction: &str) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        let colonne = zone_id % 4;
        let ligne = zone_id / 4;
        
        let nouvelle_zone = match direction {
            "n" => {
                if ligne > 0 { Some(zone_id - 4) } else { None }
            },
            "s" => {
                if ligne < 2 { Some(zone_id + 4) } else { None }
            },
            "o" => {
                if colonne > 0 { Some(zone_id - 1) } else { None }
            },
            "e" => {
                if colonne < 3 { Some(zone_id + 1) } else { None }
            },
            _ => None,
        };

        if let Some(zone_dest) = nouvelle_zone {
            // ✅ Vérifier si la zone est bloquée par un ennemi
            if let Some(z) = chambre.get_zone(zone_dest) {
                if z.est_occupee && z.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                    println!("⚔️  Un ennemi bloque le passage ! Vous devez le combattre.");
                    self.lancer_combat(zone_dest);
                    return;
                }
            }

            // ✅ Déplacement vers zone (SANS consommer de PA)
            self.personnage.zone_actuelle = zone_dest;
            println!("🚶 Vous vous déplacez vers la zone {}...", zone_dest + 1);
            
            // Marquer la zone comme visitée
            if let Some(chambre_mut) = self.labyrinthe.get_chambre_mut(chambre_id) {
                if let Some(zone_mut) = chambre_mut.get_zone_mut(zone_dest) {
                    zone_mut.est_visitee = true;
                }
            }
            
            // Afficher la nouvelle zone
            self.afficher_zone_actuelle();
        } else {
            println!("🚫 Vous ne pouvez pas aller dans cette direction (mur).");
        }
    }
}


fn lancer_combat(&mut self, zone_dest: usize) {
    let chambre_id = self.personnage.chambre_actuelle;

    println!("⚔️  Un combat est lancé contre la zone {}...", zone_dest + 1);

    // Se déplacer temporairement vers la zone et lancer le combat
    let ancienne_zone = self.personnage.zone_actuelle;
    self.personnage.zone_actuelle = zone_dest;
    self.combattre();

    // Si l'ennemi est toujours vivant, revenir en arrière
    if let Some(chambre_mut) = self.labyrinthe.get_chambre_mut(chambre_id) {
        if let Some(zone_mut) = chambre_mut.get_zone_mut(zone_dest) {
            if zone_mut.ennemi.as_ref().map_or(false, |e| !e.est_vaincu) {
                println!("🚶 Vous reculez vers la zone {}.", ancienne_zone + 1);
                self.personnage.zone_actuelle = ancienne_zone;
            } else {
                zone_mut.est_visitee = true;
            }
        }
    }
}

fn traverser_porte(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    
    // Récupérer la chambre et vérifier les portes disponibles
    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        if chambre.portes.is_empty() {
            println!("🚫 Il n'y a pas de porte sur les murs de cette chambre.");
            return;
        }

        // Déterminer quelle porte emprunter
        let porte_destination: Option<usize> = if chambre.portes.len() == 1 {
            // Seule une porte : l'utiliser directement
            Some(0)
        } else {
            // Plusieurs portes : laisser choisir le joueur
            println!("\n🚪 Choisis une porte par direction :");
            for (idx, porte) in chambre.portes.iter().enumerate() {
                println!(
                    "   [{}] {} → Chambre {}",
                    match porte.cote_mur {
                        CoteMur::Nord => "n",
                        CoteMur::Sud => "s",
                        CoteMur::Est => "e",
                        CoteMur::Ouest => "o",
                    },
                    porte.cote_mur.etiquette(),
                    porte.chambre_destination + 1
                );
            }
            print!("\n> ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            let cote = match input.as_str() {
                "n" => Some(CoteMur::Nord),
                "s" => Some(CoteMur::Sud),
                "e" => Some(CoteMur::Est),
                "o" => Some(CoteMur::Ouest),
                _ => None,
            };

            cote.and_then(|cote| {
                chambre.portes.iter().position(|porte| porte.cote_mur == cote)
            })
        };

        if let Some(porte_idx) = porte_destination {
            if porte_idx < chambre.portes.len() {
                let porte = &chambre.portes[porte_idx];
                let chambre_dest = porte.chambre_destination;
                let zone_dest = porte.zone_destination;
                let cout_pa = porte.cout_pa;
                let cout_pv = porte.cout_pv;
                let cote = porte.cote_mur;

                if self.personnage.traverser_porte(cout_pa, cout_pv) {
                    self.personnage.chambre_actuelle = chambre_dest;
                    self.personnage.zone_actuelle = zone_dest;
                    self.personnage.points_action = self.personnage.points_action_max;

                    println!("✅ Vous passez par la porte {} vers la chambre {} !", cote.etiquette(), chambre_dest + 1);
                    self.afficher_zone_actuelle();
                }
            }
        } else {
            println!("🚫 Direction invalide ou porte introuvable.");
        }
    }
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