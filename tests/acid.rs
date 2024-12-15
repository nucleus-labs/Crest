
use std::fs::read_to_string;

use peacock_crest::style::Stylesheet;

#[test]
fn acid1_bare() {
    let acid = read_to_string("static/css/acid1-bare.css").unwrap();
    acid.parse::<Stylesheet>().expect("Failed to parse css");
}

// #[test]
// fn acid1() {
//     let acid = read_to_string("static/css/acid1.css").unwrap();
//     acid.parse::<Stylesheet>().expect("Failed to parse css");
// }

#[test]
fn acid2_bare() {
    let acid = read_to_string("static/css/acid2-bare.css").unwrap();
    acid.parse::<Stylesheet>().expect("Failed to parse css");
}

// #[test]
// fn acid2() {
//     let acid = read_to_string("static/css/acid2.css").unwrap();
//     acid.parse::<Stylesheet>().expect("Failed to parse css");
// }
