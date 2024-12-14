
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock_crest::syntax::{CssRule, CssParser};
use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;

fn benchmark_parse(c: &mut Criterion) {
    let css = r#"
div.main {
    width: 50px;
    height: 100px;
}
"#;

    c.bench_function("gen_tokens", |b| {
        b.iter(|| {
            parse_source::<CssRule, CssParser>(black_box(css), CssRule::CSS)
        })
    });

    c.bench_function("gen_stylesheet", |b| {
        b.iter(|| {
            black_box(css).parse::<Stylesheet>().unwrap()
        })
    });
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
