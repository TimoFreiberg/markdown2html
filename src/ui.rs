use yew::{html, Component, ComponentLink, Html};

mod header;
mod input;
mod lints;
mod output;

use crate::ui::header::Header;
use crate::{
    convert,
    convert_html::ConvertLint,
    ui::{input::Input, lints::Lints, output::Output},
};

pub struct Model {
    link: ComponentLink<Self>,
    markdown_text: String,
    html_text: String,
    lints: Vec<ConvertLint>,
}

pub enum Msg {
    TextChanged { text: String },
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            markdown_text: String::new(),
            html_text: String::new(),
            lints: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::TextChanged { text } => {
                self.markdown_text = text;
                let (converted, lints) = convert(&self.markdown_text);
                self.html_text = converted;
                self.lints = lints;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <div class="input-output-container">
                    <Input oninput=self.link.callback(|e: Msg| e) />
                    <Output text=&self.html_text />
                </div>
                <Lints lints=&self.lints />
            </>
        }
    }
}
