use convert_html::push_html;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, App, Component, ComponentLink, Html, InputData};

mod convert_html;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

struct Model {
    link: ComponentLink<Self>,
    markdown_text: String,
    html_text: String,
    msgs: Vec<String>,
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
            msgs: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::TextChanged { text } => {
                self.markdown_text = text;
                match convert(&self.markdown_text) {
                    Ok((converted, msgs)) => {
                        self.html_text = converted;
                        self.msgs = msgs;
                    }
                    Err(e) => {
                        self.html_text = format!("Invalid Markdown: {}", e);
                        self.msgs.clear();
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let header = html! {
            <>
            <h1>{"md2html"}</h1>
            <div>{"Converts your markdown to HTML that will be rendered properly in the NT blog!"}</div>
            </>
        };
        let input = html! {
            <div>
            <label>{"Input markdown"}
                <textarea oninput=self.link.callback(|e: InputData| Msg::TextChanged {text: e.value})>
                    { &self.markdown_text }
                </textarea>
            </label>
            </div>
        };
        let output = if !self.html_text.is_empty() {
            html! {<div>
                <label>{"Generated HTML"}
                    <textarea readonly=true>
                        { &self.html_text }
                    </textarea>
                </label>
            </div>}
        } else {
            html! {}
        };
        let msgs = if !self.msgs.is_empty() {
            html! {
                <div>
                    <p>{"You'll need to manually fix these issues:"}</p>
                    <ul>
                        { self.msgs.iter().map(|li| html!{ <li> { li } </li>} ).collect::<Html>() }
                    </ul>
                </div>
            }
        } else {
            html! {}
        };
        html! {
            <>
                { header }
                { input }
                { output }
                { msgs }
            </>
        }
    }
}

pub fn convert(markdown: &str) -> eyre::Result<(String, Vec<String>)> {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut output = String::new();
    let msgs = push_html(&mut output, parser);
    Ok((output, msgs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_blocks_are_adjusted() {
        let md = r"\
```rust
fn foo() {}
"
        .trim();
        let html = r#"<pre class="lang:rust decode:true">fn foo() {}</pre>
"#;
        let (converted, _) = convert(md).unwrap();
        assert!(
            converted.contains(html),
            "Expected {:?} to contain {:?}",
            converted,
            html
        );
    }

    #[test]
    fn message_for_image() {
        let md = r#"
![image alt text](path/to/image.svg "image title")
"#
        .trim();
        let html = r#"
<img src="path/to/image.svg" alt="image alt text" title="image title" />
"#
        .trim();
        let (converted, msgs) = convert(md).unwrap();
        assert!(
            converted.contains(html),
            "Expected {:?} to contain {:?}",
            converted,
            html
        );

        let expected_msg =
            r#"Image titled "image title" with link "path/to/image.svg""#.to_string();

        assert!(
            msgs.contains(&expected_msg),
            "Expected {:?} to contain {:?}",
            msgs,
            expected_msg
        );
    }
}
