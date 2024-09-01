use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flashlog::lazy_string::LazyString;

fn bench_lazy_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_string");

    group.bench_function("3 interpotation", |b| {
        b.iter(|| {
            let _ = black_box(LazyString::new(|| format!("format {} {} {}", 1, 2, 3)));
        })
    });

    group.bench_function("6 interpotation", |b| {
        b.iter(|| {
            let _ = black_box(LazyString::new(|| format!("format {} {} {} {} {} {}", 1, 2, 3, 4, 5, 6)));
        })
    });

    group.bench_function("10 interpotation", |b| {
        b.iter(|| {
            let _ = black_box(LazyString::new(|| format!("format {} {} {} {} {} {} {} {} {} {}", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)));
        })
    });

    group.bench_function("20 interpotation", |b| {
        b.iter(|| {
            let _ = black_box(LazyString::new(|| format!(
                "format {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", 
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
            )));
        })
    });

    group.finish();
}

fn bench_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("format");
    
    group.bench_function("3 interp", |b| {
        b.iter(|| {
            let _ = black_box(format!("format {} {} {}", 1, 2, 3));
        })
    });

    group.bench_function("6 interp", |b| {
        b.iter(|| {
            let _ = black_box(format!("format {} {} {} {} {} {}", 1, 2, 3, 4, 5, 6));
        })
    });


    group.bench_function("10 interp", |b| {
        b.iter(|| {
            let _ = black_box(format!("format {} {} {} {} {} {} {} {} {} {}", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
        })
    });

    group.bench_function("20 interp", |b| {
        b.iter(|| {
            let _ = black_box(format!(
                "format {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
            ));
        })
    });

    group.finish();

}

criterion_group!(benches, bench_lazy_string, bench_format);
criterion_main!(benches);