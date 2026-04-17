#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoteMur {
    Nord,
    Sud,
    Est,
    Ouest,
}

impl CoteMur {
    pub fn oppose(self) -> Self {
        match self {
            CoteMur::Nord => CoteMur::Sud,
            CoteMur::Sud => CoteMur::Nord,
            CoteMur::Est => CoteMur::Ouest,
            CoteMur::Ouest => CoteMur::Est,
        }
    }

    pub fn etiquette(self) -> &'static str {
        match self {
            CoteMur::Nord => "Nord",
            CoteMur::Sud => "Sud",
            CoteMur::Est => "Est",
            CoteMur::Ouest => "Ouest",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Porte {
    pub chambre_destination: usize,
    pub zone_destination: usize,
    pub cote_mur: CoteMur,
    pub cout_pa: i32,
    pub cout_pv: i32,
    pub est_verrouillee: bool,
    pub necessite_cle: bool,
}

impl Porte {
    pub fn nouveau(chambre_dest: usize, zone_dest: usize, cote_mur: CoteMur) -> Self {
        Porte {
            chambre_destination: chambre_dest,
            zone_destination: zone_dest,
            cote_mur,
            cout_pa: 2,
            cout_pv: 2,
            est_verrouillee: false,
            necessite_cle: false,
        }
    }
}