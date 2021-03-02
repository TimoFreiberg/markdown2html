#![feature(format_args_capture)]

use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

mod convert_html;
mod ui;

use crate::convert_html::{push_html, ConvertLint};
use crate::ui::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

pub(crate) fn convert(markdown: &str) -> (String, Vec<ConvertLint>) {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut output = String::new();
    let msgs = push_html(&mut output, parser);
    (output, msgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Assertions {
        fn html_should_contain(self, expected_html: &str) -> Self;
        fn lints_should_contain(self, expected_lint: &ConvertLint) -> Self;
    }

    impl Assertions for (String, Vec<ConvertLint>) {
        fn html_should_contain(self, expected_html: &str) -> Self {
            let actual_html = &self.0;
            assert!(
                actual_html.contains(expected_html),
                "Expected {:?} to contain {:?}",
                actual_html,
                expected_html
            );
            self
        }

        fn lints_should_contain(self, expected_lint: &ConvertLint) -> Self {
            let lints = &self.1;
            assert!(
                lints.contains(&expected_lint),
                "Expected {:?} to contain {:?}",
                lints,
                expected_lint
            );
            self
        }
    }

    #[test]
    fn code_blocks_are_adjusted() {
        let md = r"\
```rust
fn foo() {}
"
        .trim();
        let html = r#"<pre class="lang:rust decode:true">fn foo() {}</pre>
"#;

        convert(md).html_should_contain(html);
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

        let expected_lint = ConvertLint::Image {
            title: "image title".to_string(),
            dest: "path/to/image.svg".to_string(),
        };

        convert(md)
            .html_should_contain(html)
            .lints_should_contain(&expected_lint);
    }

    #[test]
    fn single_linebreaks_in_md_are_ignored() {
        let md = r#"
lorem ipsum
dolor sit
amet
"#;
        let html = r#"
<p>lorem ipsum dolor sit amet</p>
"#
        .trim();

        convert(&md).html_should_contain(&html);
    }
}
