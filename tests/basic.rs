use peacock_crest::syntax::{CssParser, CssRule};
use peacock_crest::source::parse_source;
use peacock_crest::SourceInfo;

#[test]
fn token_gen() {
    let css = "\nmain {}";
    let source_info = SourceInfo::new(css.into());
    parse_source::<CssRule, CssParser>(source_info, CssRule::CSS).unwrap();
}
