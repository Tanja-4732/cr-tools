use super::{card_info::CardInfo, card_input::CardInput};
use crate::logic::{
    events::EventSourcingService,
    types::{gold_string, Arena, CardEntry, CardEntryV1},
};
use libocc::Event;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const CARDS_KEY_V1: &str = "cr-tools.state.cards";
const CARD_EVENTS_KEY: &str = "cr-tools.events.cards";
const ARENA_KEY: &str = "cr-tools.state.arena";

/// The listing of the cards to keep track of
pub struct CardsListing {
    link: ComponentLink<Self>,
    storage: StorageService,
    events: EventSourcingService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    cards: Vec<CardEntry>,
    arena: Arena,
}

pub enum Msg {
    Create(CardEntry),
    Update(usize, CardEntry),
    Delete(usize, CardEntry),
    SetArena(Arena),
}

impl Component for CardsListing {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Get a reference to localStorage
        let mut storage = StorageService::new(Area::Local).expect("Cannot use localStorage");

        // Load the cards from localStorage
        let events = {
            if let Json(Ok(events)) = storage.restore(CARD_EVENTS_KEY) {
                // Load the event log from localStorage
                EventSourcingService::load(events)
            } else if let Json(Ok(old_cards)) = storage.restore(CARDS_KEY_V1) {
                // Tell the compiler about the type
                // TODO improve or report bug/suggestion to rust lang
                let old_cards: Vec<CardEntryV1> = old_cards;

                // Convert the old format to the new one
                let events = EventSourcingService::migrate_from_v1(old_cards).unwrap();

                // Persist the data (including the new UUIDs)
                storage.store(CARD_EVENTS_KEY, Json(&events.borrow()));

                // Use the converted events
                events
            } else {
                // If no such entry exists, create a new one
                EventSourcingService::new()
            }
        };

        // Load the arena from localStorage
        let arena = {
            if let Ok(loaded_arena) = storage.restore(ARENA_KEY) {
                // Remove the quotes & restore the arena from localStorage
                Arena::from_str(&loaded_arena[1..&loaded_arena.len() - 1]).unwrap()
            } else {
                // If no such entry exists, default to the LegendaryArena
                Arena::LegendaryArena
            }
        };

        // Create a mutable copy of the current projection for sorting
        let mut cards = events.borrow().get_projection().clone();

        // Compute the calculated values of all cards
        CardEntry::compute_all(&mut cards, Some(&arena));

        // Sort by remaining time
        cards.sort_by(CardEntry::sort_by_remaining(Some(&arena)));

        // Compute the in_order values
        CardEntry::sum_all(&mut cards).unwrap();

        // The state of the application
        let state = State { cards, arena };

        Self {
            link,
            events,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create(mut card) => {
                // Make a create event
                self.events
                    .borrow_mut()
                    .push(Event::create(card.clone()))
                    .unwrap();

                // Generate the computed values of the card
                card.computed = card.calc_remaining(Some(&self.state.arena));

                // Add the card to the list
                self.state.cards.push(card);

                // Handle the state change
                self.handle_state_change();
            }
            Msg::Update(index, mut card) => {
                // Make an update event
                self.events
                    .borrow_mut()
                    .push(Event::update(card.clone()))
                    .unwrap();

                // Generate the computed values of the card
                card.computed = card.calc_remaining(Some(&self.state.arena));

                // Replace the outdated card entry
                self.state.cards[index] = card;

                // Handle the state change
                self.handle_state_change();
            }
            Msg::Delete(index, card) => {
                // Make a create event
                self.events.borrow_mut().push(Event::delete(card)).unwrap();

                // Remove the card
                self.state.cards.remove(index);

                // Handle the state change
                self.handle_state_change();
            }
            Msg::SetArena(arena) => {
                // Persist the data
                self.state.arena = arena;
                self.storage.store(ARENA_KEY, Json(&self.state.arena));

                // Compute the calculated values of all cards
                CardEntry::compute_all(&mut self.state.cards, Some(&self.state.arena));

                // Sort by remaining time
                self.state
                    .cards
                    .sort_by(CardEntry::sort_by_remaining(Some(&self.state.arena)));

                // Compute the in_order values
                CardEntry::sum_all(&mut self.state.cards).unwrap();
            }
        }

        // Re-render
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let total_gold = gold_string(
            self.state
                .cards
                .iter()
                .fold(0, |acc, card| acc + card.get_needed_gold()),
        );

        html! {
            <>

            <div style=BOTTOM_PADDING>
                { "Selected arena: " }
                <select onchange=self.link.callback(|event: ChangeData| {
                    if let yew::events::ChangeData::Select(data) = event {
                        Msg::SetArena(Arena::from_str(&data.value()).unwrap())
                    } else {
                        panic!("Big oof");
                    }
                }) >
                    { self.get_arenas() }
                </select>
            </div>

            <div style=GRID>

                // Render all cards
                {
                    for self.state.cards.iter().enumerate().map(|(i, card)| html!{
                        <CardInfo
                            card=card.clone()
                            on_update=self.link.callback(move |c: CardEntry| Msg::Update(i, c))
                            on_delete=self.link.callback(move |c: CardEntry| Msg::Delete(i, c),)
                        />
                    })
                }

                // Show a field for card input
                <CardInput
                    on_create=self.link.callback(|card: CardEntry| Msg::Create(card))
                    total_gold=total_gold
                />

           </div>

           </>
        }
    }
}

const GRID: &str = "
    display: grid;
    grid-template-columns: auto 4em 4em repeat(10, auto);
    gap: 5px;
";

const BOTTOM_PADDING: &str = "
    padding-bottom: 1em;
";

impl CardsListing {
    fn handle_state_change(&mut self) {
        // Sort by remaining time
        self.state
            .cards
            .sort_by(CardEntry::sort_by_remaining(Some(&self.state.arena)));

        // Compute the in_order values
        CardEntry::sum_all(&mut self.state.cards).unwrap();

        // Persist the data
        self.storage
            .store(CARD_EVENTS_KEY, Json(&self.events.borrow()));
    }

    fn get_arenas(&self) -> Html {
        Arena::iter()
            .map(|arena| {
                let name = format!("{:?}", arena);

                // Skip the training camp
                if arena == Arena::TrainingCamp {
                    return html! {};
                }

                html! {
                    <option
                        value=name
                        selected={self.state.arena == arena}
                    >
                        {name}
                    </option>
                }
            })
            .collect::<Html>()
    }
}
