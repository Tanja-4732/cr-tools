pub enum CardType {
    Troop,
    Spell,
    Building,
}

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
    common: usize,
    rare: usize,
}

pub struct DonationSize {
    common: usize,
    rare: usize,
}

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
