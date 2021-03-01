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
            <>
            <h1>{"md2html"}</h1>
            <div>{"Converts your markdown to HTML that will be rendered properly in the NT blog!"}</div>
            </>
        }
    }
}
