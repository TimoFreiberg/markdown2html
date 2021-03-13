use yew::{html, Callback, Component, ComponentLink, InputData, Properties};

use crate::ui::Msg;

pub struct Input {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub oninput: Callback<Msg>,
}

impl Component for Input {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        self.props.oninput.emit(msg);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <div class="markdown-input text-container">
                <label>
                    <h2>{"Input markdown"}</h2>
                    <textarea oninput=self.link.callback(|e: InputData| Msg::TextChanged { text: e.value }) />
                </label>
            </div>
        }
    }
}
