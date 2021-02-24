use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, App, Component, ComponentLink, InputData};

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

struct Model {
    link: ComponentLink<Self>,
    markdown_text: String,
    html_text: String,
}

enum Msg {
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::TextChanged { text } => {
                self.markdown_text = text;
                self.html_text = match convert(&self.markdown_text) {
                    Ok(it) => it,
                    Err(e) => format!("Invalid Markdown: {}", e),
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <textarea oninput=self.link.callback(|e: InputData| Msg::TextChanged {text: e.value})>
                { &self.markdown_text }
                </textarea>
                <textarea readonly=true>
                { &self.html_text }
                </textarea>
            </div>
        }
    }
}

pub fn convert(markdown: &str) -> eyre::Result<String> {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);
    Ok(output)
}
