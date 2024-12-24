use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock_crest::source::parse_source;
use peacock_crest::style::Stylesheet;
use peacock_crest::syntax::{CssParser, CssRule};
use peacock_crest::SourceInfo;

fn benchmark_gen(c: &mut Criterion) {
    let css = "\nmain {}";
    let source_info = black_box(SourceInfo::new(css.into()));

    c.bench_function("gen_tokens", |b| {
        b.iter(|| parse_source::<CssRule, CssParser>(source_info.clone(), CssRule::CSS).unwrap())
    });

    c.bench_function("reference_selector", |b| {
        b.iter(|| black_box(css).parse::<Stylesheet>().unwrap())
    });
}

criterion_group!(benches, benchmark_gen);
criterion_main!(benches);
