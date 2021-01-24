use crate::logic::types::{CardEntry, CardType, Rarity};
use chrono::{DateTime, Local};
use float_pretty_print::PrettyPrintFloat;
use serde_derive::{Deserialize, Serialize};
use std::cmp;
use std::iter::Filter;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use wasm_bindgen::prelude::*;
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{html, Component, ComponentLink, Href, Html, InputData, NodeRef, ShouldRender};

/// The root component of cr-tools
pub struct CardInfo {
    pub props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub card: CardEntry,
}

impl Component for CardInfo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let data = &self.props.card.computed;

        if let Some(data) = data {
            let get_date = |date: DateTime<Local>| date.date().format("%F");

            // Handle non-legendary cards
            html! {
                <>

                // The input fields for new cards
                <input type="text" placeholder="name" value={self.props.card.name.to_owned()}/>
                <input type="number" placeholder="level" value={self.props.card.level}/>
                <input type="number" placeholder="have" value={self.props.card.have}/>
                <select>
                    { self.get_rarities(Some(&self.props.card)) }
                </select>

                // The calculated outputs for the card
                <span>{"Need: "} {self.props.card.get_needed()}</span>
                <span>{"Remaining: "} {data.cards_remaining}</span>
                <span>{"Requests: "} {data.requests_remaining}</span>
                <span>{"Weeks: "} {Self::simple_round(data.weeks_remaining.clone())}</span>
                <span>{"Days: "} {Self::simple_round(data.days_remaining.clone())}</span>
                <span>{"Days in order: "} {Self::simple_round(data.days_in_order.unwrap().clone())}</span>
                <span>{"Done on: "} {get_date(data.done_on)}</span>
                <span>{"Done in order: "} {get_date(data.done_in_order_on.unwrap())}</span>

                </>
            }
        } else {
            // Handle legendary cards

            let cards_remaining = if self.props.card.get_needed() < self.props.card.have {
                0
            } else {
                self.props.card.get_needed() - self.props.card.have
            };

            html! {
                <>

                // The input fields for new cards
                <input type="text" placeholder="name" value={&self.props.card.name.to_owned()}/>
                <input type="number" placeholder="level" value={&self.props.card.level}/>
                <input type="number" placeholder="have" value={&self.props.card.have}/>
                <select>
                    { self.get_rarities(Some(&self.props.card)) }
                </select>

                // The calculated outputs for the card
                <span>{"Need: "} {self.props.card.get_needed()}</span>
                <span>{"Remaining: "} { cards_remaining }</span>
                <span>{"Requests: n/a"}</span>
                <span>{"Weeks: n/a"}</span>
                <span>{"Days: n/a"}</span>
                <span>{"Days in order: n/a"}</span>
                <span>{"Done on: n/a"}</span>
                <span>{"Done in order: n/a"}</span>

                </>
            }
        }
    }
}

impl CardInfo {
    fn simple_round(number: f64) -> String {
        format!("{:.3}", PrettyPrintFloat(number))
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
