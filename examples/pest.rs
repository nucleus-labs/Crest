
use peacock_crest::source::parse_source;
use peacock_crest::syntax::{CssRule, CssParser};

fn main() {
    let css = r#"
div.main {
    width: 50px;
    height: 100px;
}
"#;

    parse_source::<CssRule, CssParser>(css, CssRule::CSS).unwrap();
}
