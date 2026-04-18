use crate::entities::personnage::Personnage;
use crate::entities::personnage::ClassePersonnage;
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
            // On garde juste un message minimal, l'illustration matrice gère le visuel.
            zone.afficher_contenu(false);
            self.afficher_chambre_matrice();
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
    println!("│  [u] Capacité        │  [objet] Utiliser     │");
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
        "u" => self.utiliser_capacite_speciale(),
        "objet" => self.utiliser_objet(),
        "c" => self.combattre(),
        "f" => self.fuir(),
        "ramasser" => self.ramasser_objet(),
        "q" => self.partie_terminee = true,
        _ => println!("Commande inconnue."),
    }

    self.verifier_conditions_fin();
}

fn combattre(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    let est_spectre = self.labyrinthe
        .get_chambre(chambre_id)
        .and_then(|chambre| chambre.get_zone(zone_id))
        .and_then(|zone| zone.ennemi.as_ref())
        .map_or(false, |ennemi| {
            !ennemi.est_vaincu
                && ennemi.tipe == crate::entities::ennemi::TypeEnnemi::SpectreEnigmatique
        });

    if est_spectre {
        self.combattre_spectre();
        return;
    }

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

fn utiliser_capacite_speciale(&mut self) {
    match self.personnage.classe {
        ClassePersonnage::Kael => self.utiliser_vision(),
        ClassePersonnage::Seraph => self.utiliser_elimination(),
        ClassePersonnage::Rook => self.utiliser_crochetage(),
    }
}

fn utiliser_vision(&mut self) {
    if !self.personnage.utiliser_capacite() {
        return;
    }

    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) {
        println!("\nVision de Kael :");
        let colonne = zone_id % 4;
        let ligne = zone_id / 4;
        let voisins = [
            ("Nord", ligne > 0, zone_id.wrapping_sub(4)),
            ("Sud", ligne < 2, zone_id + 4),
            ("Ouest", colonne > 0, zone_id.wrapping_sub(1)),
            ("Est", colonne < 3, zone_id + 1),
        ];

        for (direction, possible, voisin_id) in voisins {
            if !possible {
                continue;
            }

            if let Some(zone) = chambre.get_zone(voisin_id) {
                if let Some(ennemi) = &zone.ennemi {
                    if !ennemi.est_vaincu {
                        println!("   {} -> Zone {} avec {}", direction, voisin_id + 1, ennemi.nom);
                        continue;
                    }
                }

                if let Some(objet) = &zone.objet {
                    println!("   {} -> Zone {} avec {}", direction, voisin_id + 1, objet.nom);
                } else {
                    println!("   {} -> Zone {} vide", direction, voisin_id + 1);
                }
            }
        }
    }
}

fn utiliser_elimination(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;
    let ennemi_present = self.labyrinthe
        .get_chambre(chambre_id)
        .and_then(|chambre| chambre.get_zone(zone_id))
        .and_then(|zone| zone.ennemi.as_ref())
        .map_or(false, |ennemi| !ennemi.est_vaincu);

    if !ennemi_present {
        println!("Aucun ennemi à éliminer ici.");
        return;
    }

    if !self.personnage.utiliser_capacite() {
        return;
    }

    if let Some(chambre) = self.labyrinthe.get_chambre_mut(chambre_id) {
        if let Some(zone) = chambre.get_zone_mut(zone_id) {
            if let Some(ennemi) = &mut zone.ennemi {
                ennemi.est_vaincu = true;
                ennemi.pv = 0;
                zone.est_occupee = false;
                println!("Seraph élimine {} sans perdre de PV.", ennemi.nom);
            }
        }
    }
}

fn utiliser_crochetage(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let porte_idx = self.choisir_porte_index(chambre_id);

    if let Some(porte_idx) = porte_idx {
        if !self.personnage.utiliser_capacite() {
            return;
        }
        self.traverser_porte_index(porte_idx, true);
    }
}

fn utiliser_objet(&mut self) {
    if self.personnage.inventaire.objets.is_empty() {
        println!("Inventaire vide.");
        return;
    }

    self.personnage.inventaire.afficher();
    println!("Choisis un objet à utiliser :");
    print!("> ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choix = input.trim().parse::<usize>().ok();

    let Some(index) = choix.and_then(|n| n.checked_sub(1)) else {
        println!("Choix invalide.");
        return;
    };

    if index >= self.personnage.inventaire.objets.len() {
        println!("Objet introuvable.");
        return;
    }

    let tipe = self.personnage.inventaire.objets[index].tipe;

    match tipe {
        crate::entities::objet::TypeObjet::PotionDeVie => {
            let soin = self.personnage.inventaire.objets[index].utiliser().unwrap_or(0);
            self.personnage.soigner(soin);
            println!("Potion de Vie utilisée.");
        }
        crate::entities::objet::TypeObjet::CleMystique => {
            let chambre_id = self.personnage.chambre_actuelle;
            if let Some(porte_idx) = self.choisir_porte_index(chambre_id) {
                self.personnage.inventaire.objets[index].utiliser();
                self.traverser_porte_index(porte_idx, true);
                println!("Clé Mystique utilisée.");
            }
        }
        crate::entities::objet::TypeObjet::BouclierSpectral => {
            println!("Le Bouclier Spectral s'active automatiquement lors de la prochaine attaque.");
        }
    }

    self.personnage.inventaire.objets.retain(|objet| !objet.est_utilise);
}

fn combattre_spectre(&mut self) {
    let chambre_id = self.personnage.chambre_actuelle;
    let zone_id = self.personnage.zone_actuelle;

    if let Some(spectre) = self.labyrinthe
        .get_chambre(chambre_id)
        .and_then(|chambre| chambre.get_zone(zone_id))
        .and_then(|zone| zone.ennemi.as_ref())
    {
        spectre.afficher_enigme();
    }

    loop {
        print!("\nRéponse > ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choix = input.trim().parse::<usize>().unwrap_or(0);

        let bonne_reponse = self.labyrinthe
            .get_chambre(chambre_id)
            .and_then(|chambre| chambre.get_zone(zone_id))
            .and_then(|zone| zone.ennemi.as_ref())
            .map_or(false, |spectre| spectre.verifier_reponse(choix.saturating_sub(1)));

        if bonne_reponse {
            println!("L'énigme se dissipe. Le Spectre passe au combat final.");
            break;
        }

        self.personnage.subir_degats(8);
        println!("Mauvaise réponse : -8 PV.");
        if !self.personnage.est_vivant {
            return;
        }
    }

    println!("\nPhase 2 :");
    println!("1 - Attaque directe (-6 PV)");
    println!("2 - Défense puis riposte (-3 PV)");
    println!("3 - Feinte puis frappe fatale (-2 PV)");
    print!("> ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let degats = match input.trim() {
        "1" => 6,
        "2" => 3,
        "3" => 2,
        _ => {
            println!("Hésitation fatale : le Spectre frappe violemment.");
            6
        }
    };

    self.personnage.subir_degats(degats);
    if !self.personnage.est_vivant {
        return;
    }

    if let Some(chambre) = self.labyrinthe.get_chambre_mut(chambre_id) {
        if let Some(zone) = chambre.get_zone_mut(zone_id) {
            if let Some(ennemi) = &mut zone.ennemi {
                ennemi.est_vaincu = true;
                ennemi.pv = 0;
                zone.est_occupee = false;
            }
        }
    }

    println!("Le Spectre est vaincu.");
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
    if let Some(porte_idx) = self.choisir_porte_index(chambre_id) {
        self.traverser_porte_index(porte_idx, false);
    }
}

fn choisir_porte_index(&self, chambre_id: usize) -> Option<usize> {
    let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) else {
        return None;
    };

    if chambre.portes.is_empty() {
        println!("🚫 Il n'y a pas de porte sur les murs de cette chambre.");
        return None;
    }

    if chambre.portes.len() == 1 {
        return Some(0);
    }

    println!("\n🚪 Choisis une porte par direction :");
    for porte in &chambre.portes {
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

    let porte_idx = cote.and_then(|cote| {
        chambre.portes.iter().position(|porte| porte.cote_mur == cote)
    });

    if porte_idx.is_none() {
        println!("🚫 Direction invalide ou porte introuvable.");
    }

    porte_idx
}

fn traverser_porte_index(&mut self, porte_idx: usize, gratuit: bool) {
    let chambre_id = self.personnage.chambre_actuelle;
    let Some(chambre) = self.labyrinthe.get_chambre(chambre_id) else {
        return;
    };

    let Some(porte) = chambre.portes.get(porte_idx) else {
        println!("🚫 Porte introuvable.");
        return;
    };

    let chambre_dest = porte.chambre_destination;
    let zone_dest = porte.zone_destination;
    let cout_pa = if gratuit { 0 } else { porte.cout_pa };
    let cout_pv = if gratuit { 0 } else { porte.cout_pv };
    let cote = porte.cote_mur;

    if self.personnage.traverser_porte(cout_pa, cout_pv) {
        self.personnage.chambre_actuelle = chambre_dest;
        self.personnage.zone_actuelle = zone_dest;

        println!("✅ Vous passez par la porte {} vers la chambre {} !", cote.etiquette(), chambre_dest + 1);
        self.afficher_zone_actuelle();
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

    fn verifier_conditions_fin(&mut self) {
        if !self.personnage.est_vivant || self.personnage.pv <= 0 {
            self.partie_terminee = true;
            return;
        }

        let derniere_chambre = self.labyrinthe.chambres.len().saturating_sub(1);
        if self.personnage.chambre_actuelle != derniere_chambre {
            return;
        }

        let chambre_id = self.personnage.chambre_actuelle;
        let zone_id = self.personnage.zone_actuelle;
        let ennemi_bloquant = self.labyrinthe
            .get_chambre(chambre_id)
            .and_then(|chambre| chambre.get_zone(zone_id))
            .and_then(|zone| zone.ennemi.as_ref())
            .map_or(false, |ennemi| !ennemi.est_vaincu);

        if !ennemi_bloquant {
            println!("Vous trouvez enfin la sortie du labyrinthe.");
            self.partie_terminee = true;
        }
    }

    fn fin_partie(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        let derniere_chambre = self.labyrinthe.chambres.len().saturating_sub(1);
        let victoire = self.personnage.est_vivant
            && self.personnage.chambre_actuelle == derniere_chambre;

        if victoire {
            let pa_utilises = self.personnage.points_action_max - self.personnage.points_action;
            if self.personnage.pv > 12 && pa_utilises < 8 {
                println!("║   VICTOIRE PARFAITE                     ║");
                println!("║   Le trésor ancestral t'appartient.     ║");
            } else if self.personnage.pv >= 6 {
                println!("║   VICTOIRE STANDARD                     ║");
                println!("║   Tu as survécu, mais à quel prix...    ║");
            } else {
                println!("║   VICTOIRE DIFFICILE                    ║");
                println!("║   Tu t'es échappé de justesse.          ║");
            }
        } else if self.personnage.est_vivant {
            println!("║   PARTIE TERMINÉE                       ║");
        } else {
            println!("║   GAME OVER                               ║");
            println!("║   Ton âme rejoint le labyrinthe.        ║");
        }
        println!("║   Tours joués: {:<24}  ║", self.tour);
        println!("║   PV restants: {:<23}  ║", self.personnage.pv);
        println!("║   PA restants: {:<23}  ║", self.personnage.points_action);
        println!("╚════════════════════════════════════════════╝\n");
    }
}
