
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock_crest::syntax::{CssRule, CssParser};
use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;
use std::fs::read_to_string;

fn benchmark_parse(c: &mut Criterion) {
    let css = r#"
div.main {
    width: 50px;
    height: 100px;
}
"#;

    c.bench_function("gen_tokens", |b| {
        b.iter(|| {
            parse_source::<CssRule, CssParser>(black_box(css), CssRule::CSS).unwrap()
        })
    });

    c.bench_function("gen_stylesheet", |b| {
        b.iter(|| {
            black_box(css).parse::<Stylesheet>().unwrap()
        })
    });

    let acid1 = read_to_string("static/css/acid1-bare.css").unwrap();
    let acid2 = read_to_string("static/css/acid2-bare.css").unwrap();

    c.bench_function("acid1_selectors", |b| {
        b.iter(|| {
            parse_source::<CssRule, CssParser>(black_box(&acid1), CssRule::CSS).unwrap()
        })
    });

    c.bench_function("acid2_selectors", |b| {
        b.iter(|| {
            parse_source::<CssRule, CssParser>(black_box(&acid2), CssRule::CSS).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
