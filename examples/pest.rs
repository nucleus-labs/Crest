use peacock_crest::source::{parse_source, SourceInfo};
use peacock_crest::syntax::{CssParser, CssRule};

fn main() {
    let css = r#"
div.main {
    width: 50px;
    height: 100px;
}
"#;

    let info = SourceInfo::new(css.into());
    parse_source::<CssRule, CssParser>(info, CssRule::CSS).unwrap();
}
