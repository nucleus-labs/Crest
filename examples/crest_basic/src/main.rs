// use std::sync::{Arc, Weak, RwLock};
// use std::collections::HashMap;

use peacock_crest as crest;

// struct TreeNode {
//     attributes: HashMap<&str, (&str, Vec<&str>)>,

//     inline_props: crest::style::CssStyleProperties,
//     applied_properties: crest::style::CssStyleProperties,

//     handle: Arc<RwLock<Self>>,
//     parent: Weak<RwLock<Self>>,
//     children: Vec<Arc<RwLock<Self>>>,
// }

fn main() {
    let css = r#"main {}"#;

    println!("====================================\n{css}\n====================================");
    let sheet: crest::style::Stylesheet = css.parse().expect("Failed to parse css");
    println!("{sheet}\n====================================");

    let css = r#"div.main {
    width: 50pt;
    height: 100px;
    test-prop: auto;
}"#;

    println!("{css}\n====================================");
    let sheet: crest::style::Stylesheet = css.parse().expect("Failed to parse css");
    println!("{sheet}\n====================================");
}
