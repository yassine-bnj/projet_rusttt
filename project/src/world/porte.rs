#[derive(Debug, Clone)]
pub struct Porte {
    pub chambre_destination: usize,
    pub zone_destination: usize,
    pub cout_pa: i32,
    pub cout_pv: i32,
    pub est_verrouillee: bool,
    pub necessite_cle: bool,
}

impl Porte {
    pub fn nouveau(chambre_dest: usize, zone_dest: usize) -> Self {
        Porte {
            chambre_destination: chambre_dest,
            zone_destination: zone_dest,
            cout_pa: 2,
            cout_pv: 2,
            est_verrouillee: false,
            necessite_cle: false,
        }
    }
}