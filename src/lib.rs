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

    #[test]
    fn code_blocks_are_adjusted() {
        let md = r"\
```rust
fn foo() {}
"
        .trim();
        let html = r#"<pre class="lang:rust decode:true">fn foo() {}</pre>
"#;
        let (converted, _) = convert(md);
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
        let (converted, msgs) = convert(md);
        assert!(
            converted.contains(html),
            "Expected {:?} to contain {:?}",
            converted,
            html
        );

        let expected_lint = ConvertLint::Image {
            title: "image title".to_string(),
            dest: "path/to/image.svg".to_string(),
        };

        assert!(
            msgs.contains(&expected_lint),
            "Expected {:?} to contain {:?}",
            msgs,
            expected_lint
        );
    }
}
