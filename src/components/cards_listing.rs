use crate::logic::types::{CardEntry, CardType, Rarity};
use std::cmp;
use std::iter::Filter;

use super::card_info::CardInfo;
use yew::prelude::*;

use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{html, Component, ComponentLink, Href, Html, InputData, NodeRef, ShouldRender};

const KEY: &str = "cr-tools.state.cards";

/// The listing of the cards to keep track of
pub struct CardsListing {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    focus_ref: NodeRef,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    cards: Vec<CardEntry>,
    value: String,
    edit_value: String,
}

pub enum Msg {
    Create(MouseEvent),
    Update,
    Delete,
    NoOp,
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

        let state = State {
            cards,
            value: "".into(),
            edit_value: "".into(),
        };

        // TODO maybe remove
        let focus_ref = NodeRef::default();

        Self {
            link,
            storage,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

                 // Render all cards
                { for self.state.cards.iter().map(|card| self.view_card(card)) }

            </>
        }
    }
}

impl CardsListing {
    fn view_card(&self, card: &CardEntry) -> Html {
        let data = &card.computed;

        if let Some(data) = data {
            // Handle non-legendary cards
            html! {
                <>

                // The input fields for new cards
                <input type="text" placeholder="name" value={card.name.to_owned()}/>
                <input type="number" placeholder="level" value={card.level}/>
                <input type="number" placeholder="have" value={card.have}/>
                <select>
                    { self.get_rarities(Some(&card)) }
                </select>

                // The calculated outputs for the card
                <span>{"Need: "} {card.get_needed()}</span>
                <span>{"Remaining: "} {data.cards_remaining}</span>
                <span>{"Requests: "} {data.requests_remaining}</span>
                <span>{"Weeks: "} {data.weeks_remaining}</span>
                <span>{"Days: "} {data.days_remaining}</span>
                <span>{"Days in order: "}</span> // TODO implement days_in_order
                <span>{"Done at: "}</span> // TODO implement done_at
                <span>{"Done in order: "}</span> // TODO implement done_in_order_at

                </>
            }
        } else {
            // Handle legendary cards
            html! {
                <>

                // The input fields for new cards
                <input type="text" placeholder="name" value={card.name.to_owned()}/>
                <input type="number" placeholder="level" value={card.level}/>
                <input type="number" placeholder="have" value={card.have}/>
                <select>
                    { self.get_rarities(Some(&card)) }
                </select>

                // The calculated outputs for the card
                <span>{"Need: "} {card.get_needed()}</span>
                <span>{"Remaining: "} { cmp::max(card.get_needed() - card.have, 0)}</span>
                <span>{"Requests: n/a"}</span>
                <span>{"Weeks: n/a"}</span>
                <span>{"Days: n/a"}</span>
                <span>{"Days in order: n/a"}</span>
                <span>{"Done at: n/a"}</span>
                <span>{"Done in order: n/a"}</span>

                </>
            }
        }
    }

    fn get_rarities(&self, card: Option<&CardEntry>) -> Html {
        // TODO cache/memoize this

        Rarity::iter()
            .map(|rarity| {
                let name = format!("{:?}", rarity);

                let should_select = if let Some(c) = card {
                    c.rarity == rarity
                } else {
                    false
                };

                html! {<option value=name selected={should_select}> {name} </option>}
            })
            .collect::<Html>()
    }
}
