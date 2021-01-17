use super::card_info::CardInfo;
use yew::prelude::*;

/// The listing of the cards to keep track of
pub struct CardsListing {}

pub enum Msg {}

impl Component for CardsListing {
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

                <p>{ "Hello world! cr-tools" }</p>

                <CardInfo />
                <CardInfo />
                <CardInfo />

            </>
        }
    }
}
