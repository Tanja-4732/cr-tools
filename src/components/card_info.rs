use crate::logic::types::{CardEntry, Rarity};
use chrono::{DateTime, Local};
use float_pretty_print::PrettyPrintFloat;
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;

/// The root component of cr-tools
pub struct CardInfo {
    pub props: Props,
    link: ComponentLink<Self>,
    card_backup: CardEntry,
    state: State,
}

pub enum Msg {
    Update,
    Cancel,
    Delete,
    UpdateName(String),
    UpdateLevel(usize),
    UpdateHave(usize),
    UpdateRarity(Rarity),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub card: CardEntry,
    pub on_update: Callback<CardEntry>,
    pub on_delete: Callback<()>,
}

enum State {
    Clean,
    Dirty,
    Empty,
}

impl Component for CardInfo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            card_backup: props.card.clone(),
            props,
            link,
            state: State::Clean,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = State::Dirty;

        match msg {
            Msg::UpdateName(name) => {
                if name.is_empty() {
                    self.state = State::Empty;
                }

                self.props.card.name = name;
            }
            Msg::UpdateLevel(level) => self.props.card.level = level,
            Msg::UpdateHave(have) => self.props.card.have = have,
            Msg::UpdateRarity(rarity) => self.props.card.rarity = rarity,
            Msg::Update => {
                // Give the new card to the listing component
                self.props.on_update.emit(self.props.card.clone());

                // Set as clean
                self.state = State::Clean;
            }
            Msg::Delete => {
                // Give the new card to the listing component
                self.props.on_delete.emit(());

                // Set as clean
                self.state = State::Clean;
            }
            Msg::Cancel => {
                // Restore the card from the backup
                self.props.card = self.card_backup.clone();

                // Set as clean
                self.state = State::Clean;
            }
        }

        // Re-render
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Apply the new state
        self.props = props;

        // TODO Handle non-clean states to avoid losing data during edits

        // Re-render with the new state
        true
    }

    fn view(&self) -> Html {
        match self.state {
            State::Clean => {
                if let Some(data) = &self.props.card.computed {
                    // Handle non-legendary cards

                    let get_date = |date: DateTime<Local>| date.date().format("%F");

                    html! {
                        <>

                        // The input fields for new cards
                        { self.view_inputs() }

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
                        { self.view_inputs() }

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
            State::Dirty => {
                // Handle editing
                html! {
                    <>

                    // The input fields for new cards
                    { self.view_inputs() }

                    // Save edits button
                    <button onclick=self.link.callback(|_| Msg::Update)> {"Save"} </button>

                    // Cancel button
                    <button onclick=self.link.callback(|_| Msg::Cancel)> {"Cancel"} </button>

                    // Padding
                    <span/>
                    <span/>
                    <span/>
                    <span/>
                    <span/>
                    <span/>

                    </>
                }
            }
            State::Empty => {
                // Handle empty
                html! {
                    <>

                    // The input fields for new cards
                    { self.view_inputs() }

                    // Save edits button
                    <button onclick=self.link.callback(|_| Msg::Delete)> {"Delete"} </button>

                    // Cancel button
                    <button onclick=self.link.callback(|_| Msg::Cancel)> {"Cancel"} </button>

                    // Padding
                    <span/>
                    <span/>
                    <span/>
                    <span/>
                    <span/>
                    <span/>

                    </>
                }
            }
        }
    }
}

impl CardInfo {
    fn simple_round(number: f64) -> String {
        format!("{:.3}", PrettyPrintFloat(number))
    }

    fn get_rarities(&self, card: Option<&CardEntry>) -> Html {
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

    /// Renders the input elements
    fn view_inputs(&self) -> Html {
        html! {
            <>

            <input
                type="text"
                placeholder="name"
                value={self.props.card.name.to_owned()}
                oninput=self.link.callback(|i: InputData| Msg::UpdateName(i.value))
            />

            <input
                type="number"
                placeholder="level"
                value={self.props.card.level}
                oninput=self.link.callback(|i: InputData| Msg::UpdateLevel(i.value.parse::<usize>().unwrap()))
            />

            <input
                type="number"
                placeholder="have"
                value={self.props.card.have}
                oninput=self.link.callback(|i: InputData| Msg::UpdateHave(i.value.parse::<usize>().unwrap()))
            />

            <select onchange=self.link.callback(|event: ChangeData| {
                if let yew::events::ChangeData::Select(data) = event {
                    Msg::UpdateRarity(Rarity::from_str(&data.value()).unwrap())
                } else {
                    panic!("Big oof");
                }
            }) >
                { self.get_rarities(Some(&self.props.card)) }
            </select>

            </>
        }
    }
}
