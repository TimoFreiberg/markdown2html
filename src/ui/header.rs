use yew::{html, Component, ComponentLink, Html};

pub struct Header;

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="header">
                <h1>{"markdown2html"}</h1>
                <subtitle>{"Converts markdown to HTML that will be rendered properly in a Wordpress blog!"}</subtitle>
                <hr/>
            </div>
        }
    }
}
