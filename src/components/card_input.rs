use crate::logic::types::{CardEntry, CardType, Rarity};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::events::ChangeData;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};

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
    UpdateLevel(usize),
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
                level: 9,
                card_type: CardType::Building,
                rarity: Rarity::Common,
                computed: None,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NoOp => {}
            Msg::UpdateName(name) => self.card.name = name,
            Msg::UpdateLevel(level) => self.card.level = level,
            Msg::UpdateHave(have) => self.card.have = have,
            Msg::UpdateRarity(rarity) => self.card.rarity = rarity,
            Msg::Create => {
                // Push the card onto the list
                self.cards.push(self.card.clone());

                // Persist the data
                self.storage.store(KEY, Json(&self.cards));
            }
        }

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
                <input type="text" placeholder="name" oninput=self.link.callback(|i: InputData| Msg::UpdateName(i.value)) />
                <input type="number" placeholder="level" oninput=self.link.callback(|i: InputData| Msg::UpdateLevel(i.value.parse::<usize>().unwrap())) />
                <input type="number" placeholder="have" oninput=self.link.callback(|i: InputData| Msg::UpdateHave(i.value.parse::<usize>().unwrap())) />
                <select onchange=self.link.callback(|i: ChangeData| {
                    if let yew::events::ChangeData::Select(data) = i {
                        Msg::UpdateRarity(Rarity::from_str(&data.value()).unwrap())
                    } else {
                        panic!("Big oof");
                    }
                }) >
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
