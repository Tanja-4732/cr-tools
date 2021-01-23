use super::types::{get_request_size, Arena, CardEntry, Rarity, REQUEST_FREQUENCY};
use chrono::{DateTime, Duration, Local};
use serde_derive::{Deserialize, Serialize};
use std::cmp;
use strum_macros::{EnumIter, EnumString};

#[derive(PartialEq, Clone)]
pub struct CardData {
    pub cards_remaining: usize,
    pub requests_remaining: usize,
    pub weeks_remaining: f64,
    pub days_remaining: f64,
    pub done_on: DateTime<Local>,
    pub days_in_order: Option<f64>,
    pub done_in_order_on: Option<DateTime<Local>>,
}

impl CardEntry {
    pub fn calc_remaining(&self, arena: Option<Arena>) -> Option<CardData> {
        // Cannot request legendary cards
        if self.rarity == Rarity::Legendary {
            return None;
        }

        // The arena the user is in (default to the LegendaryArena)
        let request_size = get_request_size(&arena.unwrap_or(Arena::LegendaryArena));

        let cards_remaining = cmp::max(self.get_needed() as isize - self.have as isize, 0) as usize;

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

        let days_remaining = weeks_remaining * 7 as f64;

        let done_on =
            Local::now().checked_add_signed(Duration::days(days_remaining.ceil() as i64))?;

        Some(CardData {
            cards_remaining,
            requests_remaining,
            weeks_remaining,
            days_remaining,
            done_on,
            days_in_order: None,
            done_in_order_on: None,
        })
    }

    pub fn compute_all(list: &mut Vec<Self>) {
        for card in list {
            card.computed = card.calc_remaining(None);
        }
    }

    /// Custom order algorithm for sorting CardEntries by days
    pub fn sort_remaining(a: &Self, b: &Self) -> cmp::Ordering {
        // Handle legendary cards
        if a.rarity == Rarity::Legendary {
            if b.rarity == Rarity::Legendary {
                return cmp::Ordering::Equal;
            } else {
                return cmp::Ordering::Greater;
            }
        }
        if b.rarity == Rarity::Legendary {
            return cmp::Ordering::Less;
        }

        let get_remaining = |card: &CardEntry| {
            card.computed
                .clone()
                .unwrap_or_else(|| card.calc_remaining(None).unwrap())
                .days_remaining
        };

        // Compare the cards
        get_remaining(a).partial_cmp(&get_remaining(b)).unwrap()
    }
}
