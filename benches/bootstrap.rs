use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock_crest::style::Stylesheet;
use std::fs::read_to_string;

fn benchmark_bootstrap(c: &mut Criterion) {
    {
        let mut bootstrap = c.benchmark_group("bootstrap1");
        bootstrap.measurement_time(std::time::Duration::from_secs(10));

        let bootstrap_data = read_to_string("static/css/bootstrap-1.0.0.css").unwrap();

        bootstrap.bench_function("selectors", |b| {
            b.iter(|| black_box(&bootstrap_data).parse::<Stylesheet>().unwrap())
        });

        let bootstrap_data: Stylesheet = bootstrap_data.parse::<Stylesheet>().unwrap();

        bootstrap.bench_function("selectors_display", |b| {
            b.iter(|| black_box(&bootstrap_data).to_string())
        });

        let bootstrap_data = read_to_string("static/css/acid1.css").unwrap();

        bootstrap.bench_function("acid1", |b| {
            b.iter(|| black_box(&bootstrap_data).parse::<Stylesheet>().unwrap())
        });

        let bootstrap_data: Stylesheet = bootstrap_data.parse::<Stylesheet>().unwrap();

        bootstrap.bench_function("display", |b| {
            b.iter(|| black_box(&bootstrap_data).to_string())
        });

        bootstrap.finish();
    }
}

criterion_group!(benches, benchmark_bootstrap);
criterion_main!(benches);
