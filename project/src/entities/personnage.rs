use crate::entities::inventaire::Inventaire;
use crate::entities::objet::Objet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassePersonnage {
    Kael,
    Seraph,
    Rook,
}

#[derive(Debug, Clone)]
pub struct Personnage {
    pub nom: String,
    pub classe: ClassePersonnage,
    pub pv: i32,
    pub pv_max: i32,
    pub points_action: i32,
    pub points_action_max: i32,
    pub inventaire: Inventaire,
    pub capacite_restante: i32,
    pub chambre_actuelle: usize,
    pub zone_actuelle: usize,
    pub est_vivant: bool,
}

impl Personnage {
    pub fn nouveau(nom: &str, classe: ClassePersonnage, pv: i32, capacite_max: i32) -> Self {
        Personnage {
            nom: nom.to_string(),
            classe,
            pv,
            pv_max: pv,
            points_action: 12,
            points_action_max: 12,
            inventaire: Inventaire::nouveau(10),
            capacite_restante: capacite_max,
            chambre_actuelle: 0,
            zone_actuelle: 0,
            est_vivant: true,
        }
    }

    pub fn kael() -> Self {
        Personnage::nouveau("Kael", ClassePersonnage::Kael, 15, 3)
    }

    pub fn seraph() -> Self {
        Personnage::nouveau("Seraph", ClassePersonnage::Seraph, 18, 2)
    }

    pub fn rook() -> Self {
        Personnage::nouveau("Rook", ClassePersonnage::Rook, 14, 3)
    }

    pub fn subir_degats(&mut self, degats: i32) -> bool {
        self.pv -= degats;
        if self.pv <= 0 {
            self.pv = 0;
            self.est_vivant = false;
            return true;
        }
        false
    }

    pub fn soigner(&mut self, montant: i32) {
        self.pv = (self.pv + montant).min(self.pv_max);
        println!("PV restaurés : {}/{}", self.pv, self.pv_max);
    }

    pub fn utiliser_capacite(&mut self) -> bool {
        if self.capacite_restante > 0 {
            self.capacite_restante -= 1;
            true
        } else {
            println!("Capacité épuisée !");
            false
        }
    }

    pub fn ajouter_objet(&mut self, objet: Objet) -> bool {
        self.inventaire.ajouter(objet)
    }

    pub fn afficher_statut(&self) {
        println!("\n----------------------------------");
        println!(" Nom: {}", self.nom);
        println!(" Classe: {:?}", self.classe);
        println!("----------------------------------");
        println!(" PV: {}/{}", self.pv, self.pv_max);
        println!(" PA: {}/{}", self.points_action, self.points_action_max);
        println!(" Capacité: {} utilisations", self.capacite_restante);
        println!(" Position: Chambre {}, Zone {}", self.chambre_actuelle + 1, self.zone_actuelle + 1);
        println!("----------------------------------\n");
    }

    #[allow(dead_code)]
    pub fn deplacer_zone(&mut self, nouvelle_zone: usize) {
    // ✅ Déplacement entre zones : NE CONSOMME PAS DE PA
    self.zone_actuelle = nouvelle_zone;
    println!("🚶 Vous vous déplacez vers la zone {}...", nouvelle_zone + 1);
}

pub fn traverser_porte(&mut self, cout_pa: i32, cout_pv: i32) -> bool {
    // ✅ Traverser une porte : CONSOMME PA et PV
    if self.points_action >= cout_pa && self.pv > cout_pv {
        self.points_action -= cout_pa;
        self.pv -= cout_pv;
        println!("🚪 Vous traversez la porte (-{} PA, -{} PV)", cout_pa, cout_pv);
        true
    } else {
        println!("❌ Impossible de traverser ! PA: {}/{}, PV: {}/{}", 
                 self.points_action, cout_pa, self.pv, cout_pv);
        false
    }
}
}
