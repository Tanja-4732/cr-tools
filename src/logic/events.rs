use super::types::{CardEntry, CardEntryV1};
use anyhow::Result;
use libocc::{Event, Projector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventSourcingService {
    projector: Projector<CardEntry>,
}

impl EventSourcingService {
    pub fn new() -> Self {
        Self {
            projector: Projector::new(),
        }
    }

    pub fn migrate_from_v1(old_cards: Vec<CardEntryV1>) -> Result<Self> {
        let mut projector = Projector::new();

        // Add all cards
        for card in old_cards {
            projector.push(Event::create(card.retrofit_uuid()))?;
        }

        // Return the projector inside of a new EventSourcingService
        Ok(Self { projector })
    }

    pub fn borrow(&self) -> &Projector<CardEntry> {
        &self.projector
    }

    pub fn borrow_mut(&mut self) -> &Projector<CardEntry> {
        &mut self.projector
    }
}
