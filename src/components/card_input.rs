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
    focus_ref: NodeRef,
    cards: Vec<CardEntry>,
    card: CardEntry,
    storage: StorageService,
}

pub enum Msg {
    Create,
    UpdateName(String),
    UpdateNeed(usize),
    UpdateHave(usize),
    UpdateRarity(Rarity),
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

        // TODO maybe remove
        let focus_ref = NodeRef::default();

        Self {
            link,
            focus_ref,
            cards,
            storage,
            card: CardEntry {
                name: String::new(),
                have: 0,
                need: 0,
                card_type: CardType::Building,
                rarity: Rarity::Rare,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // Re-render
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>

                // The input fields for new cards
                <input placeholder="name" />
                <input placeholder="need" />
                <input placeholder="have" />
                <select>
                    { self.get_rarities(None) }
                </select>
                <button onclick=self.link.callback(|c| Msg::Create)> {"Add"} </button>

            </>
        }
    }
}

impl CardInput {
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
