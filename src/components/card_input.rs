use super::card_info::CardInfo;
use yew::prelude::*;

/// Enter information about a card
pub struct CardInput {}

pub enum Msg {}

pub struct InputProps {
    #[prop_or(2)]
    active: u64,
}

impl Component for CardInput {
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
