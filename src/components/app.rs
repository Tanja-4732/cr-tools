use super::{card_info::CardInfo, card_input::CardInput, cards_listing::CardsListing};
use yew::prelude::*;

/// The root component of cr-tools
pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
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

            <h1>{ "cr-tools" }</h1>
            <p>
            { "This app is a work in progress, and my first Yew project." }
            <br />
            { "Some functionality might not be implemented yet. Please be patient." }
            </p>

            <div style=MY_STYLE>
                <CardsListing />
                <CardInput />
            </div>

            </>
        }
    }
}

const MY_STYLE: &str = "
    display: grid;
    grid-template-columns: repeat(11, auto);
    gap: 5px;
";
