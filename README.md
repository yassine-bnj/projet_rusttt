
# LE LABYRINTHE DES ÂMES PERDUES

Un jeu de rôle textuel stratégique développé en Rust, où l'intelligence et la planification priment sur le hasard. Le joueur incarne un personnage piégé dans un donjon maudit et doit trouver la sortie avant que ses points de vie ne s'épuisent.

**Genre :** RPG Stratégique / Dungeon Crawler  
**Langage :** Rust  
**Plateforme :** Terminal / Console  
**Statut :** En développement

---

## Table des matières

- [Description](#description)
- [Caractéristiques](#caractéristiques)
- [Mécaniques de jeu](#mécaniques-de-jeu)
- [Personnages](#personnages)
- [Ennemis](#ennemis)
- [Objets](#objets)
- [Système de fins](#système-de-fins)
- [Architecture](#architecture)
- [Documentation technique](#documentation-technique)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Contribuer](#contribuer)
- [Licence](#licence)

---

## Description

Le Labyrinthe des Âmes Perdues est un dungeon crawler où chaque décision compte. Contrairement aux jeux basés sur le hasard, chaque action a des conséquences prévisibles et mesurables. Le joueur doit gérer ses ressources limitées (Points de Vie et Points d'Action) tout en naviguant dans un labyrinthe rempli d'ennemis, d'énigmes et de pièges.

---

## Caractéristiques

- **Gameplay stratégique** : Aucun élément aléatoire, chaque décision impacte le résultat
- **Trois personnages uniques** avec des capacités spéciales distinctes
- **Système de ressources** : Gestion des Points d'Action (PA) et Points de Vie (PV)
- **Combat tactique** : Choix stratégiques avec conséquences variables
- **Boss final** : Combat en deux phases avec énigme et affrontement tactique
- **Fins multiples** : Résultat basé sur la performance du joueur

---

## Mécaniques de jeu

### Système de Points d'Action (PA)

Chaque personnage dispose de **12 PA** pour toute la partie. Ouvrir une porte consomme des PA selon sa difficulté :

- Porte niveau 3 : 3 PA
- Porte niveau 5 : 5 PA
- Porte niveau 7 : 7 PA

Le joueur doit optimiser son chemin à travers le labyrinthe en gérant ces ressources limitées.

### Système de Points de Vie (PV)

Les PV varient selon le personnage choisi (14 à 18 PV). Chaque combat, piège ou mauvaise décision peut réduire les PV. Si les PV atteignent zéro, la partie est perdue.

### Système de combat

Trois approches possibles face à un ennemi :
- **Combattre** : Éliminer l'ennemi en perdant des PV
- **Négocier** : Donner un objet ou payer un tribut
- **Contourner** : Trouver une alternative (coût variable en PV/PA)

---

## Personnages

### KAEL - Le Chercheur
- **Points de Vie :** 15
- **Capacité spéciale :** Vision (3 utilisations)
  - Révèle le contenu des chambres adjacentes avant d'y entrer
  - Permet une planification optimale du parcours
- **Style de jeu :** Exploration méthodique et anticipation

### SERAPH - La Guerrière
- **Points de Vie :** 18
- **Capacité spéciale :** Élimination gratuite (2 utilisations)
  - Tue un ennemi sans perdre de PV
  - Idéal pour les ennemis dangereux
- **Style de jeu :** Combat direct et agressif

### ROOK - Le Voleur
- **Points de Vie :** 14
- **Capacité spéciale :** Crochetage (3 utilisations)
  - Ouvre une porte sans dépenser de PA
  - Maximise l'exploration avec ressources limitées
- **Style de jeu :** Économie de ressources et contournement

---

## Ennemis

### Ombre Errante (Niveau faible)
- **Dégâts :** 3 PV
- **Stratégies disponibles :**
  - Combattre (-3 PV) : Élimination directe
  - Fuir (-1 PV) : Évitement partiel, ennemi reste présent
  - Donner un objet : Passage libre sans dégâts

### Gardien de Pierre (Niveau moyen)
- **Dégâts :** 5 PV
- **Stratégies disponibles :**
  - Combattre (-5 PV) : Affrontement frontal coûteux
  - Payer tribut : Donner un objet pour passage libre
  - Contourner (-3 PV) : Trouver un passage secret

### Spectre Maudit (Boss final)
Apparaît uniquement devant la porte de sortie. Combat en deux phases obligatoires :

**Phase 1 - Énigme mortelle :**
- Question avec 3 réponses possibles
- Mauvaise réponse : -8 PV et nouvelle tentative obligatoire
- Bonne réponse : Passage automatique en Phase 2

**Phase 2 - Combat tactique :**
Trois actions disponibles avec différents coûts :
- **Attaque Directe** : -6 PV (le Spectre contre-attaque)
- **Défense puis Riposte** : -3 PV (efficace mais coûteux)
- **Feinte puis Frappe Fatale** : -2 PV (solution optimale, nécessite réflexion)

---

## Objets

Trois objets à usage unique disponibles dans le labyrinthe :

| Objet | Effet | Usage | Stratégie |
|-------|-------|-------|-----------|
| **Potion de Vie** | Restaure 5 PV | 1 fois | À conserver pour le boss ou situations critiques |
| **Clé Mystique** | Ouvre une porte sans PA | 1 fois | Utile pour portes coûteuses en fin de partie |
| **Bouclier Spectral** | Annule complètement 1 attaque | 1 fois | Protection contre le boss ou ennemis dangereux |

---

## Système de fins

Le jeu se termine par une des quatre fins possibles, déterminée par la performance du joueur :

### Victoire Parfaite (3 étoiles)
- **Conditions :** PV > 12 ET PA utilisés < 8
- **Récompense :** Découverte du trésor secret du labyrinthe
- **Message :** "Tu as triomphé avec brio ! Le trésor ancestral t'appartient."

### Victoire Standard (2 étoiles)
- **Conditions :** 6 ≤ PV ≤ 12
- **Message :** "Tu as survécu, mais à quel prix... Les cicatrices resteront."

### Victoire Difficile (1 étoile)
- **Conditions :** 1 ≤ PV ≤ 5
- **Message :** "Tu t'es échappé de justesse, brisé mais vivant."

### Défaite
- **Condition :** PV = 0
- **Message :** "Ton âme rejoint les autres dans le labyrinthe éternel..."

---

## Architecture

### Structure du projet

```
labyrinth_game/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── docs/
│   ├── diagrammes/
│   │   ├── Diagramme_de_class.png
│   │   ├── Diagramme_de_seq_DEMARRAGE_DU_JEU.png
│   │   ├── Diagramme_de_seq_TOUR_DE_JEU.png
│   │   ├── Diagramme_de_seq_BOSS_SPECTRE.png
│   │   ├── Diagramme_de_seq_COMBAT_SIMPLE.png
│   │   ├── Diagramme_de_seq_OUVRIR_PORTE.png
│   │   ├── Diagramme_de_seq_UTILISATION_OBJET.png
│   │   └── Diagramme_de_seq_UTILISATION_CAPACITE.png
│   └── specifications/
│       └── game_design.md
└── src/
    ├── main.rs
    ├── game_state.rs
    ├── player.rs
    ├── character.rs
    ├── enemy.rs
    ├── item.rs
    ├── room.rs
    └── door.rs
```

### Modules principaux

#### GameState
Gestion de l'état global du jeu, orchestration des tours, vérification des conditions de victoire/défaite.

**Responsabilités :**
- Initialisation du labyrinthe
- Boucle de jeu principale
- Gestion des transitions entre chambres
- Vérification de l'état de victoire/défaite

#### Player
Représentation du joueur avec ses statistiques, inventaire et capacités.

**Attributs principaux :**
- `character: Character` - Type de personnage
- `hp: i32` - Points de vie actuels
- `action_points: i32` - Points d'action restants
- `inventory: Vec<Item>` - Objets possédés

#### Character
Enum définissant les trois personnages jouables avec leurs capacités uniques.

**Variants :**
- `Kael { vision_uses: i32 }` - Capacité de vision
- `Seraph { free_kills: i32 }` - Capacité d'élimination
- `Rook { free_doors: i32 }` - Capacité de crochetage

#### Enemy
Enum définissant les trois types d'ennemis avec leurs comportements.

**Variants :**
- `Shadow { damage: i32 }` - Ennemi basique
- `Guardian { damage: i32 }` - Ennemi intermédiaire
- `Specter { phase: SpecterPhase }` - Boss final

#### Item
Enum définissant les objets collectables.

**Variants :**
- `HealthPotion(i32)` - Restaure des PV
- `MysticKey` - Ouvre une porte
- `SpectralShield` - Bloque une attaque

#### Room
Structure représentant une chambre du labyrinthe.

**Attributs :**
- `id: usize` - Identifiant unique
- `room_type: RoomType` - Type de chambre
- `doors: Vec<Door>` - Portes disponibles
- `enemy: Option<Enemy>` - Ennemi présent
- `item: Option<Item>` - Objet présent
- `is_exit: bool` - Chambre de sortie

#### Door
Structure représentant une porte entre deux chambres.

**Attributs :**
- `direction: Direction` - Direction (Nord, Sud, Est, Ouest)
- `cost: i32` - Coût en PA pour ouvrir
- `target_room: usize` - Chambre cible
- `locked: bool` - État de verrouillage

---

## Documentation technique

### Diagrammes UML disponibles

La conception du jeu est documentée par les diagrammes suivants (format PNG dans `docs/diagrammes/`) :

#### Diagramme de classes
Visualisation complète de la structure du système avec :
- Toutes les classes/structures principales
- Relations entre entités (composition, agrégation, héritage)
- Interfaces (traits) implémentées
- Méthodes publiques de chaque classe

#### Diagrammes de séquence

1. **Démarrage du jeu**
   - Initialisation du GameState
   - Création du joueur
   - Génération du labyrinthe
   - Configuration initiale

2. **Tour de jeu**
   - Boucle principale de gameplay
   - Affichage de l'état actuel
   - Traitement des actions du joueur
   - Vérification des conditions de fin

3. **Combat simple**
   - Rencontre avec un ennemi standard
   - Choix du joueur (combattre/fuir/négocier)
   - Application des dégâts
   - Résolution du combat

4. **Boss Spectre (2 phases)**
   - Phase 1 : Énigme avec pénalités
   - Transition entre phases
   - Phase 2 : Combat tactique
   - Résolution finale

5. **Ouvrir porte**
   - Sélection d'une porte
   - Vérification des PA disponibles
   - Déplacement vers nouvelle chambre
   - Mise à jour de l'état

6. **Utilisation d'objet**
   - Sélection dans l'inventaire
   - Application de l'effet
   - Retrait de l'inventaire

7. **Utilisation de capacité**
   - Vérification de la disponibilité
   - Activation selon le personnage
   - Décompte des utilisations

Tous les diagrammes sont disponibles dans le dossier `docs/diagrammes/`.

---

## Installation

### Prérequis

- **Rust** : Version 1.70 ou supérieure
- **Cargo** : Gestionnaire de paquets Rust (inclus avec Rust)
- **Système d'exploitation** : Windows, macOS, ou Linux

### Installation de Rust

Si Rust n'est pas installé sur votre système :

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Télécharger et exécuter rustup-init.exe depuis https://rustup.rs/
```

### Étapes d'installation du projet

```bash
# 1. Cloner le dépôt
git clone https://github.com/votre-username/labyrinth-game.git
cd labyrinth-game

# 2. Vérifier l'installation de Rust
rustc --version
cargo --version

# 3. Compiler le projet en mode debug
cargo build

# 4. Compiler le projet en mode release (optimisé)
cargo build --release

# 5. Exécuter le jeu
cargo run --release
```

### Installation rapide

```bash
git clone https://github.com/votre-username/labyrinth-game.git && cd labyrinth-game && cargo run --release
```

---

## Utilisation

### Lancer le jeu

```bash
# Mode développement (compilation rapide)
cargo run

# Mode release (exécution optimisée)
cargo run --release
```

### Commandes en jeu

Le jeu utilise une interface textuelle avec saisie de commandes numériques :

#### 1. Sélection du personnage

Au démarrage, choisir un personnage en entrant le numéro correspondant :
```
1 - Kael (15 PV, Vision x3)
2 - Seraph (18 PV, Kill x2)
3 - Rook (14 PV, Open Door x3)
```

#### 2. Actions dans une chambre

En fonction de la situation, différentes options sont proposées :
- **Fouiller** : Chercher des objets cachés
- **Examiner** : Observer l'environnement
- **Utiliser capacité** : Activer le pouvoir spécial du personnage
- **Choisir une porte** : Se déplacer vers une autre chambre

#### 3. Combat

Face à un ennemi, sélectionner une stratégie :
```
1 - Combattre
2 - Fuir / Négocier / Contourner (selon l'ennemi)
3 - Utiliser un objet
4 - Utiliser capacité spéciale
```

#### 4. Gestion de l'inventaire

```
i - Afficher l'inventaire
1-3 - Utiliser l'objet correspondant
```

### Exemple de partie

```
=== LE LABYRINTHE DES ÂMES PERDUES ===

Choisis ton personnage:
1. Kael (15 PV, Vision x3)
2. Seraph (18 PV, Kill x2)
3. Rook (14 PV, Open Door x3)
> 1

=== CHAMBRE 1 - Salle d'Éveil ===
PV: 15/15 | PA: 12/12

Description: Une petite pièce de pierre froide...

Portes disponibles:
1. Porte Nord [5 PA]
2. Porte Est [3 PA]

Actions:
1. Utiliser capacité (Vision)
2. Choisir une porte
> 1

Vision activée! (2 utilisations restantes)
Porte Nord → Chambre avec Gardien de Pierre
Porte Est → Chambre vide avec Potion

> 2
Choisir porte: 2

Vous entrez dans la chambre 2...
```

---

## Contribuer

Les contributions sont les bienvenues ! Voici comment participer au projet.

### Workflow de contribution

1. **Fork** le projet
2. Créer une branche pour votre fonctionnalité
   ```bash
   git checkout -b feature/NouvelleFonctionnalite
   ```
3. Commiter vos changements
   ```bash
   git commit -m 'Ajout de NouvelleFonctionnalite'
   ```
4. Push vers la branche
   ```bash
   git push origin feature/NouvelleFonctionnalite
   ```
5. Ouvrir une **Pull Request**

### Standards de code

#### Formatage
```bash
# Formater le code selon les conventions Rust
cargo fmt

# Vérifier le formatage sans modifier
cargo fmt --check
```

#### Linting
```bash
# Exécuter Clippy pour les warnings
cargo clippy

# Clippy strict
cargo clippy -- -D warnings
```

#### Tests
```bash
# Exécuter tous les tests
cargo test

# Tests avec affichage verbeux
cargo test -- --nocapture

# Couverture de code (nécessite tarpaulin)
cargo tarpaulin --out Html
```

### Guidelines

- **Documenter** les fonctions publiques avec `///`
- **Tester** les nouvelles fonctionnalités
- Respecter les **conventions Rust** (naming, ownership, etc.)
- Maintenir la **couverture de tests** au-dessus de 80%
- Mettre à jour la **documentation** si nécessaire

### Types de contributions acceptées

- Correction de bugs
- Nouvelles fonctionnalités
- Amélioration de la documentation
- Optimisation du code
- Ajout de tests
- Traductions

---

## Roadmap

### Version 0.1.0 (En cours)
- [ ] Implémentation des structures de base (GameState, Player, Room)
- [ ] Système de combat avec les 3 ennemis
- [ ] Génération statique du labyrinthe
- [ ] Interface utilisateur en terminal
- [ ] Boss final avec 2 phases
- [ ] Système de fins multiples

### Version 0.2.0 (Planifiée)
- [ ] Sauvegarde et chargement de partie (format JSON)
- [ ] Génération procédurale de labyrinthes
- [ ] Plus d'énigmes variées pour le boss
- [ ] Mode de difficulté (Facile/Normal/Difficile)
- [ ] Statistiques de partie (temps, décisions, etc.)

### Version 0.3.0 (Future)
- [ ] Plus de personnages jouables (2-3 nouveaux)
- [ ] Nouveaux types d'ennemis et pièges
- [ ] Système de compétences évolutif
- [ ] Événements aléatoires narratifs

### Version 1.0.0 (Long terme)
- [ ] Interface graphique (TUI avec ratatui ou GUI)
- [ ] Musique et effets sonores
- [ ] Système d'achievements
- [ ] Mode multijoueur (tour par tour)
- [ ] Éditeur de labyrinthes personnalisés

---

## Tests

### Exécuter les tests

```bash
# Tous les tests
cargo test

# Tests spécifiques
cargo test test_player_creation
cargo test combat

# Tests avec output
cargo test -- --show-output
```

### Structure des tests

```
tests/
├── integration_tests.rs    # Tests d'intégration
└── unit_tests/
    ├── player_tests.rs     # Tests unitaires Player
    ├── character_tests.rs  # Tests unitaires Character
    ├── enemy_tests.rs      # Tests unitaires Enemy
    └── game_state_tests.rs # Tests unitaires GameState
```

### Couverture de code

```bash
# Installer tarpaulin
cargo install cargo-tarpaulin

# Générer le rapport de couverture
cargo tarpaulin --out Html --output-dir coverage
```

---

## Technologies utilisées

- **Rust** : Langage principal (edition 2021)
- **Cargo** : Build system et package manager
- **std::io** : Entrées/sorties console
- **serde** (futur) : Sérialisation pour sauvegardes

### Dépendances prévues

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## FAQ

### Le jeu sera-t-il disponible avec une interface graphique ?

Oui, une interface graphique est prévue pour la version 1.0.0. La version actuelle se concentre sur la logique de jeu et l'interface terminal.

### Puis-je créer mes propres labyrinthes ?

Un éditeur de labyrinthes est prévu pour une version future. Pour l'instant, les labyrinthes sont générés par le code.

### Le jeu est-il vraiment sans hasard ?

Oui, absolument. Tous les résultats sont déterministes et basés uniquement sur les choix du joueur. Deux parties avec les mêmes décisions auront le même résultat.

### Combien de temps dure une partie ?

Une partie complète dure entre 10 et 20 minutes selon les choix et le personnage.

---

## Auteurs

- **Votre Nom** - Développeur principal - [GitHub](https://github.com/votre-username)

### Contributeurs

Voir la liste des [contributeurs](https://github.com/votre-username/labyrinth-game/contributors) qui ont participé à ce projet.

---

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

### Résumé de la licence MIT

Vous êtes libre de :
- Utiliser le code commercialement
- Modifier le code
- Distribuer le code
- Utiliser le code en privé

Sous conditions de :
- Inclure la licence et le copyright
- Ne pas tenir l'auteur responsable

---

## Remerciements

- Inspiration tirée des jeux rogue-like classiques (NetHack, Dungeon Crawl)
- Conception basée sur des principes de game design stratégique
- Communauté Rust pour le support et les outils excellents
- Contributeurs et testeurs du projet

---

## Ressources

### Documentation Rust
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Game Design
- [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [Roguelike Development](http://rogueliketutorials.com/)

---

## Contact

Pour toute question, suggestion ou signalement de bug :

- **Email** : votre.email@example.com
- **Issues** : [GitHub Issues](https://github.com/votre-username/labyrinth-game/issues)
- **Discussions** : [GitHub Discussions](https://github.com/votre-username/labyrinth-game/discussions)

---

## Changelog

### [Unreleased]
- Implémentation initiale des structures
- Diagrammes UML complets
- Documentation technique

---

**Note** : Ce projet est en développement actif. Les fonctionnalités et l'API peuvent changer sans préavis avant la version 1.0.0.
```
