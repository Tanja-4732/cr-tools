use super::card_info::CardInfo;
use super::card_input::CardInput;
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
            <div>

                <h1>{ "cr-tools" }</h1>

                <CardInput />

            </div>
        }
    }
}
