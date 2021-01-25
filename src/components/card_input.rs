use crate::logic::types::{CardEntry, Rarity};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;

/// Enter information about a card
pub struct CardInput {
    link: ComponentLink<Self>,
    card: CardEntry,
    props: Props,
}

pub enum Msg {
    Create,
    UpdateName(String),
    UpdateLevel(usize),
    UpdateHave(usize),
    UpdateRarity(Rarity),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub on_create: Callback<CardEntry>,
}

impl Component for CardInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            card: CardEntry::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => self.card.name = name,
            Msg::UpdateLevel(level) => self.card.level = level,
            Msg::UpdateHave(have) => self.card.have = have,
            Msg::UpdateRarity(rarity) => self.card.rarity = rarity,
            Msg::Create => {
                // Give the new card to the listing component
                self.props.on_create.emit(self.card.clone());

                // Reset this component
                self.card = CardEntry::new();

                // Re-render
                return true;
            }
        }

        // Don't re-render
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>

                // The input fields for new cards
                <input
                    type="text"
                    placeholder="name"
                    oninput=self.link.callback(|i: InputData| Msg::UpdateName(i.value))
                />

                <input
                    type="number"
                    placeholder="level"
                    oninput=self.link.callback(|i: InputData| Msg::UpdateLevel(i.value.parse::<usize>().unwrap()))
                />

                <input
                    type="number"
                    placeholder="have"
                    oninput=self.link.callback(|i: InputData| Msg::UpdateHave(i.value.parse::<usize>().unwrap()))
                />

                <select onchange=self.link.callback(|event: ChangeData| {
                    if let yew::events::ChangeData::Select(data) = event {
                        Msg::UpdateRarity(Rarity::from_str(&data.value()).unwrap())
                    } else {
                        panic!("Big oof");
                    }
                }) >
                    { self.get_rarities(None) }
                </select>

                // Save changes button
                <button
                    onclick=self.link.callback(|_| Msg::Create)
                    disabled={self.card.name.is_empty()}
                >
                    {"Add"}
                </button>

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
