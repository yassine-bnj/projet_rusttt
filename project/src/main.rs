mod entities;
mod world;
mod game;

use entities::personnage::Personnage;
use world::labyrinthe::Labyrinthe;
use game::engine::GameEngine;

fn main() {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║   LE LABYRINTHE DES ÂMES PERDUES          ║");
    println!("║                                           ║");
    println!("║   Choisis ton personnage :                ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║   [1] KAEL - Le Chercheur                 ║");
    println!("║       Capacité: Voir chambres adjacentes  ║");
    println!("║                                           ║");
    println!("║   [2] SERAPH - La Guerrière               ║");
    println!("║       Capacité: Tuer sans perdre de PV    ║");
    println!("║                                           ║");
    println!("║   [3] ROOK - Le Voleur                    ║");
    println!("║       Capacité: Ouvrir porte gratuitement ║");
    println!("╚════════════════════════════════════════════╝\n");

    let mut choix = String::new();
    std::io::stdin().read_line(&mut choix).unwrap();

    let personnage = match choix.trim() {
        "1" => {
            println!(" Tu as choisi KAEL - Le Chercheur\n");
            Personnage::kael()
        },
        "2" => {
            println!(" Tu as choisi SERAPH - La Guerrière\n");
            Personnage::seraph()
        },
        "3" => {
            println!(" Tu as choisi ROOK - Le Voleur\n");
            Personnage::rook()
        },
        _ => {
            println!("Choix invalide, personnage par défaut: Kael\n");
            Personnage::kael()
        }
    };

    let mut labyrinthe = Labyrinthe::nouveau("Appartement Maudit", 3, 3);
    labyrinthe.generer();

    let mut engine = GameEngine::nouveau(personnage, labyrinthe);
    engine.demarrer();
}