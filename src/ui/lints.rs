use yew::{html, Component, Properties};

use crate::convert_html::ConvertLint;

pub(crate) struct Lints {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub lints: Vec<ConvertLint>,
}

impl Component for Lints {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        if !self.props.lints.is_empty() {
            html! {
                <div>
                    <h2 class="lint">
                        {format!("You'll need to manually fix {} image links", self.props.lints.len())}
                    </h2>
                </div>
            }
        } else {
            html! {}
        }
    }
}
