use peacock_crest::Stylesheet;
use std::fs::read_to_string;

fn main() {
    let acid1_selectors = read_to_string("static/css/acid1-selectors.css").unwrap();
    let acid2_selectors = read_to_string("static/css/acid2-selectors.css").unwrap();
    let acid1 = read_to_string("static/css/acid1.css").unwrap();
    let acid2 = read_to_string("static/css/acid2.css").unwrap();

    {
        let _ = Stylesheet::from_source(&acid1_selectors).expect("Failed to parse css");
    }
    {
        let _ = Stylesheet::from_source(&acid2_selectors).expect("Failed to parse css");
    }
    {
        println!("{acid1}");
        let acid = Stylesheet::from_source(&acid1).expect("Failed to parse css");
        println!("{acid}");
    }
    {
        let acid = Stylesheet::from_source(&acid2).expect("Failed to parse css");
        println!("{acid}");
    }
}
