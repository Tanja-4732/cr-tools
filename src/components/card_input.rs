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

/// Enter information about a card
pub struct CardInput {
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

impl Component for CardInput {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Get a reference to localStorage
        let storage = StorageService::new(Area::Local).expect("Cannot use localStorage");

        // Load the cards from localStorage
        let cards = {
            if let Json(Ok(loaded_cards)) = storage.restore(KEY) {
                loaded_cards
            } else {
                // If no such entry exists, create a new one
                Vec::new()
            }
        };

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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create(c) => self.state.cards.push(CardEntry {
                card_type: CardType::Building,
                have: 42,
                need: 100,
                name: format!("{:?}", c),
                rarity: Rarity::Rare,
            }),
            Msg::Update => {}
            Msg::Delete => {}
            Msg::NoOp => {}
        }

        // Persist the data
        self.storage.store(KEY, Json(&self.state.cards));

        // Re-render
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style=MY_STYLE>
                // Render all cards
                { for self.state.cards.iter().map(|card| self.view_card(card)) }

                // The input fields for new cards
                <input placeholder="name" />
                <input placeholder="need" />
                <input placeholder="have" />
                <select>
                    { self.get_rarities(None) }
                </select>
                <button onclick=self.link.callback(|c| Msg::Create(c))> {"Add"} </button>

            </div>
        }
    }
}

impl CardInput {
    fn view_card(&self, card: &CardEntry) -> Html {
        html! {
            <>

            // The input fields for new cards
            <input placeholder="name" value={card.name.to_owned()}/>
            <input placeholder="need" value={card.need}/>
            <input placeholder="have" value={card.have}/>
            <select>
                { self.get_rarities(Some(&card)) }
            </select>

            // The calculated outputs for the card
            <p>{"Remaining: "} { cmp::max(card.need - card.have, 0)}</p>
            <p>{"Requests: "}</p>
            <p>{"Weeks: "}</p>
            <p>{"Days: "}</p>
            <p>{"Days in order: "}</p>
            <p>{"Done at: "}</p>
            <p>{"Done in order: "}</p>

            </>
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

const MY_STYLE: &str = "
    display: grid;
    grid-template-columns: repeat(11, auto);
    gap: 5px;
";
