use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct CardEntry {
    /// The name of the card
    pub name: String,

    /// The amount of cards in posession
    pub have: usize,

    /// The amount of cards required to upgrade
    pub need: usize,

    /// The rarity of the card
    pub card_type: CardType,

    /// The rarity of the card
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum CardType {
    Troop,
    Spell,
    Building,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

pub enum Arena {
    TrainingCamp = 0,
    GoblinStadium,
    BonePit,
    BarbarianBowl,
    PekkasPlayhouse,
    SpellValley,
    BuildersWorkshop,
    RoyalArena,
    FrozenPeak,
    JungleArena,
    HogMountain,
    ElectroValley,
    SpookyTown,
    LegendaryArena,
}

pub struct RequestSize {
    pub common: usize,
    pub rare: usize,
}

pub struct DonationSize {
    pub common: usize,
    pub rare: usize,
}

/// Returns the daily donation limit for a given arena
pub fn get_donation_limit(arena: &Arena) -> usize {
    match arena {
        Arena::TrainingCamp => 0,
        Arena::GoblinStadium => 90,
        Arena::BonePit => 90,
        Arena::BarbarianBowl => 90,
        Arena::PekkasPlayhouse => 180,
        Arena::SpellValley => 180,
        Arena::BuildersWorkshop => 180,
        Arena::RoyalArena => 270,
        Arena::FrozenPeak => 270,
        Arena::JungleArena => 270,
        Arena::HogMountain => 360,
        Arena::ElectroValley => 360,
        Arena::SpookyTown => 360,
        Arena::LegendaryArena => 360,
    }
}

/// Returns the donation size limit for a given arena
pub fn get_donation_size(arena: &Arena) -> DonationSize {
    match arena {
        Arena::TrainingCamp => DonationSize { common: 0, rare: 0 },
        Arena::GoblinStadium => DonationSize { common: 1, rare: 1 },
        Arena::BonePit => DonationSize { common: 2, rare: 1 },
        Arena::BarbarianBowl => DonationSize { common: 2, rare: 1 },
        Arena::PekkasPlayhouse => DonationSize { common: 4, rare: 1 },
        Arena::SpellValley => DonationSize { common: 4, rare: 1 },
        Arena::BuildersWorkshop => DonationSize { common: 4, rare: 1 },
        Arena::RoyalArena => DonationSize { common: 6, rare: 1 },
        Arena::FrozenPeak => DonationSize { common: 6, rare: 1 },
        Arena::JungleArena => DonationSize { common: 6, rare: 1 },
        Arena::HogMountain => DonationSize { common: 8, rare: 1 },
        Arena::ElectroValley => DonationSize { common: 8, rare: 1 },
        Arena::SpookyTown => DonationSize { common: 8, rare: 1 },
        Arena::LegendaryArena => DonationSize { common: 8, rare: 1 },
    }
}

/// Returns the request size limit for a given arena
pub fn get_request_size(arena: &Arena) -> RequestSize {
    use RequestSize as Rs;

    match arena {
        Arena::TrainingCamp => Rs { common: 0, rare: 0 },
        Arena::GoblinStadium => Rs {
            common: 10,
            rare: 1,
        },
        Arena::BonePit => Rs {
            common: 10,
            rare: 1,
        },
        Arena::BarbarianBowl => Rs {
            common: 10,
            rare: 1,
        },
        Arena::PekkasPlayhouse => Rs {
            common: 20,
            rare: 2,
        },
        Arena::SpellValley => Rs {
            common: 20,
            rare: 2,
        },
        Arena::BuildersWorkshop => Rs {
            common: 20,
            rare: 2,
        },
        Arena::RoyalArena => Rs {
            common: 30,
            rare: 3,
        },
        Arena::FrozenPeak => Rs {
            common: 30,
            rare: 3,
        },
        Arena::JungleArena => Rs {
            common: 30,
            rare: 3,
        },
        Arena::HogMountain => Rs {
            common: 40,
            rare: 4,
        },
        Arena::ElectroValley => Rs {
            common: 40,
            rare: 4,
        },
        Arena::SpookyTown => Rs {
            common: 40,
            rare: 4,
        },
        Arena::LegendaryArena => Rs {
            common: 40,
            rare: 4,
        },
    }
}
