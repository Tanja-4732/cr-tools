use yew::prelude::*;

/// The root component of cr-tools
pub struct CardInfo {}

pub enum Msg {}
#[derive(Properties, Clone)]
pub struct Props {}

impl Component for CardInfo {
    type Message = Msg;
    type Properties = Props;

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
            <p>{ "card details here" }</p>
        }
    }
}
