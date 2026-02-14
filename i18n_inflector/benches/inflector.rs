use criterion::{Criterion, criterion_group, criterion_main};
use i18n_inflector::{pluralize, singularize};
use std::hint::black_box;

fn bench_singularize_no_change(c: &mut Criterion) {
    c.bench_function("singularize_no_change", |b| {
        b.iter(|| singularize(black_box("en"), black_box("user")).expect("supported locale"));
    });
}

fn bench_pluralize_no_change(c: &mut Criterion) {
    c.bench_function("pluralize_no_change", |b| {
        b.iter(|| pluralize(black_box("en"), black_box("users")).expect("supported locale"));
    });
}

fn bench_singularize_en(c: &mut Criterion) {
    c.bench_function("singularize_en", |b| {
        b.iter(|| singularize(black_box("en"), black_box("categories")).expect("supported locale"));
    });
}

fn bench_pluralize_en(c: &mut Criterion) {
    c.bench_function("pluralize_en", |b| {
        b.iter(|| pluralize(black_box("en"), black_box("category")).expect("supported locale"));
    });
}

fn bench_singularize_es(c: &mut Criterion) {
    c.bench_function("singularize_es", |b| {
        b.iter(|| singularize(black_box("es"), black_box("ciudades")).expect("supported locale"));
    });
}

fn bench_pluralize_es(c: &mut Criterion) {
    c.bench_function("pluralize_es", |b| {
        b.iter(|| pluralize(black_box("es"), black_box("ciudad")).expect("supported locale"));
    });
}

fn bench_singularize_de(c: &mut Criterion) {
    c.bench_function("singularize_de", |b| {
        b.iter(|| singularize(black_box("de"), black_box("produkte")).expect("supported locale"));
    });
}

fn bench_pluralize_de(c: &mut Criterion) {
    c.bench_function("pluralize_de", |b| {
        b.iter(|| pluralize(black_box("de"), black_box("produkt")).expect("supported locale"));
    });
}

fn bench_singularize_fr(c: &mut Criterion) {
    c.bench_function("singularize_fr", |b| {
        b.iter(|| singularize(black_box("fr"), black_box("journaux")).expect("supported locale"));
    });
}

fn bench_singularize_invariant(c: &mut Criterion) {
    c.bench_function("singularize_invariant", |b| {
        b.iter(|| singularize(black_box("ja"), black_box("user")).expect("supported locale"));
    });
}

fn bench_singularize_unknown_locale(c: &mut Criterion) {
    c.bench_function("singularize_unknown_locale", |b| {
        b.iter(|| {
            let _ = singularize(black_box("xx"), black_box("users"));
        });
    });
}

fn bench_pluralize_tr(c: &mut Criterion) {
    c.bench_function("pluralize_tr", |b| {
        b.iter(|| pluralize(black_box("tr"), black_box("kullanici")).expect("supported locale"));
    });
}

criterion_group!(
    benches,
    bench_singularize_no_change,
    bench_pluralize_no_change,
    bench_singularize_en,
    bench_pluralize_en,
    bench_singularize_es,
    bench_pluralize_es,
    bench_singularize_de,
    bench_pluralize_de,
    bench_singularize_fr,
    bench_singularize_invariant,
    bench_singularize_unknown_locale,
    bench_pluralize_tr,
);
criterion_main!(benches);
