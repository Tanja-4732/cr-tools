use super::{card_info::CardInfo, card_input::CardInput};
use crate::logic::types::{CardEntry, CardType, Rarity};
use chrono::{DateTime, Local};
use float_pretty_print::PrettyPrintFloat;
use serde_derive::{Deserialize, Serialize};
use std::iter::Filter;
use std::{cmp, mem};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{html, Component, ComponentLink, Href, Html, InputData, NodeRef, ShouldRender};

const KEY: &str = "cr-tools.state.cards";

/// The listing of the cards to keep track of
pub struct CardsListing {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    cards: Vec<CardEntry>,
    real_pass: bool,
}

pub enum Msg {
    Create(CardEntry),
    Update(usize, CardEntry),
    Delete(usize),
    RealPass,
}

impl Component for CardsListing {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Get a reference to localStorage
        let storage = StorageService::new(Area::Local).expect("Cannot use localStorage");

        // Load the cards from localStorage
        let mut cards = {
            if let Json(Ok(loaded_cards)) = storage.restore(KEY) {
                loaded_cards
            } else {
                // If no such entry exists, create a new one
                Vec::new()
            }
        };

        // Compute the calculated values of all cards
        CardEntry::compute_all(&mut cards);

        // Sort by remaining time
        cards.sort_by(CardEntry::sort_remaining);

        // Compute the in_order values
        CardEntry::sum_all(&mut cards).unwrap();

        let state = State {
            cards,
            real_pass: true,
        };

        Self {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create(mut card) => {
                // Generate the computed values of the card
                card.computed = card.calc_remaining(None);

                // Add the card to the list
                self.state.cards.push(card);

                // Sort by remaining time
                self.state.cards.sort_by(CardEntry::sort_remaining);

                // Compute the in_order values
                CardEntry::sum_all(&mut self.state.cards).unwrap();

                // Persist the data
                self.storage.store(KEY, Json(&self.state.cards));

                // Do a fake render first
                self.state.real_pass = false;
            }
            Msg::Update(index, mut card) => {
                // TODO Support arenas other than the LegendaryArena
                // Generate the computed values of the card
                card.computed = card.calc_remaining(None);

                // Replace the outdated card entry
                self.state.cards[index] = card;

                // Sort by remaining time
                self.state.cards.sort_by(CardEntry::sort_remaining);

                // Compute the in_order values
                CardEntry::sum_all(&mut self.state.cards).unwrap();

                // Persist the data
                self.storage.store(KEY, Json(&self.state.cards));

                // Do a fake render first
                self.state.real_pass = false;
            }
            Msg::Delete(index) => {
                // TODO implement deletion
            }
            Msg::RealPass => self.state.real_pass = true,
        }

        // Re-render
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if self.state.real_pass {
            html! {
                <div style=MY_STYLE>

                    // Render all cards
                    {
                        for self.state.cards.iter().enumerate().map(|(i, card)| html!{
                            <CardInfo
                                card=card.clone()
                                on_update=self.link.callback(move |c: CardEntry| Msg::Update(i, c))
                            />
                        })
                    }

                    // Show a field for card input
                    <CardInput on_create=self.link.callback(|card: CardEntry| Msg::Create(card)) />

               </div>
            }
        } else {
            // Force a legit re-render (avoids memoization)
            html! { <img src="" onerror=self.link.callback(|_| Msg::RealPass) />}
        }
    }
}

const MY_STYLE: &str = "
    display: grid;
    grid-template-columns: auto 4em 4em repeat(9, auto);
    gap: 5px;
";
