use yew::{html, Component, Html, Properties};

use crate::convert_html::{self, ConvertLint};

pub(crate) struct Lints {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Props {
    pub lints: Vec<ConvertLint>,
}

impl convert_html::ConvertLint {
    fn render(&self) -> Html {
        match self {
            convert_html::ConvertLint::Image { title, dest } => {
                let title = if title.is_empty() {
                    String::new()
                } else {
                    format!("titled {title:?}")
                };
                let msg = format!("Image {title} with link {dest:?}");
                html! {
                    { msg }
                }
            }
        }
    }
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
                    <p>{"You'll need to manually fix these issues:"}</p>
                    <ul>
                        {
                            self.props.lints
                                .iter()
                                .map(|li| html!{ <li> { li.render() } </li>} )
                                .collect::<Html>()
                        }
                    </ul>
                </div>
            }
        } else {
            html! {}
        }
    }
}
