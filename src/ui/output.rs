use web_sys::window;
use yew::{html, Component, ComponentLink, Properties};

pub struct Output {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

impl Component for Output {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if props != self.props {
            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                // FIXME deal with promise somehow
                let _ = clipboard.write_text(&props.text);
            }
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <div class="html-output text-container">
                <label>
                    <h2>
                    {
                        if self.props.text.is_empty() {
                            "Found nothing to generate yet"
                        } else {
                            "We copied this HTML to your clipboard"
                        }
                    }
                    </h2>
                    <textarea disabled=true>
                        { &self.props.text }
                    </textarea>
                </label>
            </div>
        }
    }
}
