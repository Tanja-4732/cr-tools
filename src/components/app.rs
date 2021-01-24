use super::cards_listing::CardsListing;
use yew::prelude::*;

/// The root component of cr-tools
pub struct App;

impl Component for App {
    type Message = ();
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
            <br />
            <em>{ "Changes are not yet supported" }</em>
            </p>

            <CardsListing />

            </>
        }
    }
}
