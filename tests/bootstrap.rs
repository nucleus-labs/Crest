use std::fs::read_to_string;

use peacock_crest::style::Stylesheet;

#[test]
fn bootstrap3() {
    let bootstrap = read_to_string("static/css/bootstrap-3.4.1.css").unwrap();
    bootstrap.parse::<Stylesheet>().expect("Failed to parse css");
}

#[test]
fn bootstrap4() {
    let bootstrap = read_to_string("static/css/bootstrap-4.6.2.css").unwrap();
    bootstrap.parse::<Stylesheet>().expect("Failed to parse css");
}

// #[test]
// fn bootstrap5() {
//     let bootstrap = read_to_string("static/css/bootstrap-5.0.0.css").unwrap();
//     bootstrap.parse::<Stylesheet>().expect("Failed to parse css");
// }
