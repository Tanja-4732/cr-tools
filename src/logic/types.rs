use serde_derive::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};
use uuid::Uuid;

use super::calc::CardData;

#[derive(Deserialize, Clone)]
pub struct CardEntryV1 {
    /// The name of the card
    pub name: String,

    /// The amount of cards in posession
    pub have: usize,

    /// The current level of the card
    pub level: usize,

    /// The rarity of the card
    pub card_type: CardType,

    /// The rarity of the card
    pub rarity: Rarity,

    /// Computed values
    #[serde(skip)]
    pub computed: Option<CardData>,
}

impl CardEntryV1 {
    pub fn retrofit_uuid(self) -> CardEntry {
        CardEntry {
            uuid: Uuid::new_v4(),
            name: self.name,
            have: self.have,
            level: self.level,
            card_type: self.card_type,
            rarity: self.rarity,
            computed: self.computed,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct CardEntry {
    /// The UUID of the card  
    /// Used for tracking renames
    pub uuid: Uuid,

    /// The name of the card
    pub name: String,

    /// The amount of cards in posession
    pub have: usize,

    /// The current level of the card
    pub level: usize,

    /// The rarity of the card
    pub card_type: CardType,

    /// The rarity of the card
    pub rarity: Rarity,

    /// Computed values
    #[serde(skip)]
    pub computed: Option<CardData>,
}

impl CardEntry {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::new(),
            have: 0,
            level: 9,
            card_type: CardType::Building,
            rarity: Rarity::Common,
            computed: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum CardType {
    Troop,
    Spell,
    Building,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, EnumString, PartialEq, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, EnumString, PartialEq, Clone)]
pub enum Arena {
    TrainingCamp,
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
    match arena {
        Arena::TrainingCamp => RequestSize { common: 0, rare: 0 },
        Arena::GoblinStadium | Arena::BonePit | Arena::BarbarianBowl => RequestSize {
            common: 10,
            rare: 1,
        },
        Arena::PekkasPlayhouse | Arena::SpellValley | Arena::BuildersWorkshop => RequestSize {
            common: 20,
            rare: 2,
        },
        Arena::RoyalArena | Arena::FrozenPeak | Arena::JungleArena => RequestSize {
            common: 30,
            rare: 3,
        },
        Arena::HogMountain | Arena::ElectroValley | Arena::SpookyTown | Arena::LegendaryArena => {
            RequestSize {
                common: 40,
                rare: 4,
            }
        }
    }
}

pub struct RequestFrequency {
    pub common: f64,
    pub rare: f64,
    pub epic: f64,
    pub legendary: f64,
}

/**
The amount of requests to place per week

- Each day can fit three requests
- Epic cards can (and get) only be requested once per week
- Common or rare cards get requested otherwise
*/
pub const REQUEST_FREQUENCY: RequestFrequency = RequestFrequency {
    common: (6 * 3 + 2) as f64,
    rare: (6 * 3 + 2) as f64,
    epic: (1) as f64,
    legendary: (0) as f64,
};

const NEEDED_CARDS: [usize; 13] = [1, 2, 4, 10, 20, 50, 100, 200, 400, 800, 1000, 2000, 5000];

const COMMON_OFFSET: usize = 0;
const RARE_OFFSET: usize = 2;
const EPIC_OFFSET: usize = 5;
const LEGENDARY_OFFSET: usize = 8;

const NEEDED_GOLD: [usize; 13] = [
    0, 5, 20, 50, 150, 400, 1000, 2000, 4000, 8000, 20000, 50000, 100000,
];

impl CardEntry {
    /// Calculates the amount of required cards to upgrade to the next level (or 0 when on 13)
    pub fn get_needed_cards(&self) -> usize {
        if self.level == 13 {
            return 0;
        };

        let use_offset = |offset: usize| {
            if self.level < 1 {
                panic!("Invalid level")
            } else {
                NEEDED_CARDS[self.level - offset]
            }
        };

        match self.rarity {
            Rarity::Common => use_offset(COMMON_OFFSET),
            Rarity::Rare => use_offset(RARE_OFFSET),
            Rarity::Epic => use_offset(EPIC_OFFSET),
            Rarity::Legendary => use_offset(LEGENDARY_OFFSET),
        }
    }

    /// Calculates the amount of required gold to upgrade to the next level (or 0 when on 13)
    pub fn get_needed_gold(&self) -> usize {
        if self.level == 13 {
            return 0;
        }

        match self.rarity {
            Rarity::Common => NEEDED_GOLD[self.level],
            Rarity::Rare => NEEDED_GOLD[self.level],
            Rarity::Epic if self.level == 6 => 400,
            Rarity::Epic => NEEDED_GOLD[self.level],
            Rarity::Legendary if self.level == 9 => 5000,
            Rarity::Legendary => NEEDED_GOLD[self.level],
        }
    }

    /// Calculates the amount of required gold to upgrade to the next level (or 0 when on 13)
    /// as a String, formatting a `usize` not below 1000 as `{:.3}K`, with the actual number divided by 1000
    pub fn get_needed_gold_string(&self) -> String {
        gold_string(self.get_needed_gold())
    }
}

pub fn gold_string(number: usize) -> String {
    if number < 1000 {
        number.to_string()
    } else {
        format!("{:.3}K", number / 1000)
    }
}
