use super::cards_listing::CardsListing;
use crate::constants;
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

            <h1>{constants::meta::NAME}</h1>
            {constants::meta::ABOUT}
            <p>
            { "This app is a work in progress, and my first Yew project." }
            <br />
            { "Some functionality might not be implemented yet. Please be patient." }
            <br />
            <em>{ "Only edit one element at a time - clear name to delete" }</em>
            </p>

            <CardsListing />


            <p>
                { "Get the source code "}
                <a href="https://github.com/Bernd-L/cr-tools">{ "here" }</a>
            </p>

            <div style=BOXED>
                <h3>{constants::license::license_notice_title()}</h3>

                <p>
                    {constants::license::LICENSE_SHORT} <br />
                    {"The license: "} <a href=constants::license::LICENSE_URL>{constants::license::LICENSE_URL}</a>
                </p>

                {constants::license::license_notice_body()}
            </div>

            <br />
            { "\"Clash Royale\" may be a trademark of its owner, with which I'm not affiliated with at all."}

            </>
        }
    }
}

const BOXED: &str = "
    max-width: 700px;
";
