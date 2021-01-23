use std::cmp;

use super::types::{get_request_size, Arena, CardEntry, Rarity, REQUEST_FREQUENCY};

pub struct CardData {
    pub cards_remaining: usize,
    pub requests_remaining: usize,
    pub weeks_remaining: f64,
    pub days_remaining: f64,
    // TODO implement the "in order" fields
    // pub days_in_order: usize,
    // pub done_at: Date,
    // pub done_in_order_at: Date,
}

impl CardEntry {
    pub fn calc_remaining(&self, arena: Option<Arena>) -> Option<CardData> {
        // Cannot request legendary cards
        if self.rarity == Rarity::Legendary {
            return None;
        }

        // The arena the user is in (default to the LegendaryArena)
        let request_size = get_request_size(&arena.unwrap_or(Arena::LegendaryArena));

        let cards_remaining = cmp::max(self.need as isize - self.have as isize, 0) as usize;

        let requests_remaining = (cards_remaining as f64
            / if self.rarity == Rarity::Common {
                request_size.common as f64
            } else {
                request_size.rare as f64
            })
        .ceil() as usize;

        let weeks_remaining = requests_remaining as f64
            / match self.rarity {
                Rarity::Common => REQUEST_FREQUENCY.common,
                Rarity::Rare => REQUEST_FREQUENCY.rare,
                Rarity::Epic => REQUEST_FREQUENCY.epic,
                Rarity::Legendary => REQUEST_FREQUENCY.legendary, // Unreachable
            } as f64;

        Some(CardData {
            cards_remaining,
            requests_remaining,
            weeks_remaining,
            days_remaining: weeks_remaining * 7 as f64,
        })
    }
}
