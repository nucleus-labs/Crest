use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock_crest::style::Stylesheet;
use std::fs::read_to_string;

fn benchmark_acid(c: &mut Criterion) {
    {
        let mut acid1 = c.benchmark_group("acid1");
        acid1.measurement_time(std::time::Duration::from_secs(10));

        let acid1_data = read_to_string("static/css/acid1-selectors.css").unwrap();

        acid1.bench_function("selectors", |b| {
            b.iter(|| black_box(&acid1_data).parse::<Stylesheet>().unwrap())
        });

        let acid1_data: Stylesheet = acid1_data.parse::<Stylesheet>().unwrap();

        acid1.bench_function("selectors_display", |b| {
            b.iter(|| black_box(&acid1_data).to_string())
        });

        let acid1_data = read_to_string("static/css/acid1.css").unwrap();

        acid1.bench_function("acid1", |b| {
            b.iter(|| black_box(&acid1_data).parse::<Stylesheet>().unwrap())
        });

        let acid1_data: Stylesheet = acid1_data.parse::<Stylesheet>().unwrap();

        acid1.bench_function("display", |b| b.iter(|| black_box(&acid1_data).to_string()));

        acid1.finish();
    }
    {
        let mut acid2 = c.benchmark_group("acid2");
        acid2.measurement_time(std::time::Duration::from_secs(10));

        let acid2_data = read_to_string("static/css/acid2-selectors.css").unwrap();

        acid2.bench_function("selectors", |b| {
            b.iter(|| black_box(&acid2_data).parse::<Stylesheet>().unwrap())
        });

        let acid2_data: Stylesheet = acid2_data.parse::<Stylesheet>().unwrap();

        acid2.bench_function("selectors_display", |b| {
            b.iter(|| black_box(&acid2_data).to_string())
        });

        let acid2_data = read_to_string("static/css/acid2.css").unwrap();

        acid2.bench_function("acid2", |b| {
            b.iter(|| black_box(&acid2_data).parse::<Stylesheet>().unwrap())
        });

        let acid2_data: Stylesheet = acid2_data.parse::<Stylesheet>().unwrap();

        acid2.bench_function("display", |b| b.iter(|| black_box(&acid2_data).to_string()));

        acid2.finish();
    }
}

criterion_group!(benches, benchmark_acid);
criterion_main!(benches);
