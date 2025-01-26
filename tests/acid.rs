use std::fs::read_to_string;

use peacock_crest::Stylesheet;

#[test]
fn acid1_selectors() {
    let acid = read_to_string("static/css/acid1-selectors.css").unwrap();
    let _ = Stylesheet::from_source(&acid).expect("Failed to parse css");
}

#[test]
fn acid1_selectors_display() {
    let acid = read_to_string("static/css/acid1-selectors.css").unwrap();
    let stylesheet = Stylesheet::from_source(&acid).expect("Failed to parse css");
    stylesheet.to_string();
}

#[test]
fn acid1() {
    let acid = read_to_string("static/css/acid1.css").unwrap();
    let _ = Stylesheet::from_source(&acid).expect("Failed to parse css");
}

#[test]
fn acid1_display() {
    let acid = read_to_string("static/css/acid1.css").unwrap();
    let stylesheet = Stylesheet::from_source(&acid).expect("Failed to parse css");
    stylesheet.to_string();
}

#[test]
fn acid2_selectors() {
    let acid = read_to_string("static/css/acid2-selectors.css").unwrap();
    let _ = Stylesheet::from_source(&acid).expect("Failed to parse css");
}

#[test]
fn acid2_selectors_display() {
    let acid = read_to_string("static/css/acid2-selectors.css").unwrap();
    let stylesheet = Stylesheet::from_source(&acid).expect("Failed to parse css");
    stylesheet.to_string();
}

#[test]
fn acid2() {
    let acid = read_to_string("static/css/acid2.css").unwrap();
    let _ = Stylesheet::from_source(&acid).expect("Failed to parse css");
}

#[test]
fn acid2_display() {
    let acid = read_to_string("static/css/acid2.css").unwrap();
    let stylesheet = Stylesheet::from_source(&acid).expect("Failed to parse css");
    stylesheet.to_string();
}
