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
    cards: Vec<Entry>,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Create,
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style=MY_STYLE>

                <input placeholder="name" />
                <input placeholder="need" />
                <input placeholder="have" />

                // { for self.props.items.iter().map(renderItem) }


            </div>
        }
    }
}

const MY_STYLE: &str = "
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 5px;
";
