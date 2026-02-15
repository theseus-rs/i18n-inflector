use criterion::{Criterion, criterion_group, criterion_main};
use i18n_inflector::{LanguageRules, language_rules};
use std::hint::black_box;

fn bench_singularize_no_change(c: &mut Criterion) {
    let rules = language_rules("en").expect("supported locale");
    c.bench_function("singularize_no_change", |b| {
        b.iter(|| rules.singularize(black_box("user")));
    });
}

fn bench_pluralize_no_change(c: &mut Criterion) {
    let rules = language_rules("en").expect("supported locale");
    c.bench_function("pluralize_no_change", |b| {
        b.iter(|| rules.pluralize(black_box("users")));
    });
}

fn bench_singularize_en(c: &mut Criterion) {
    let rules = language_rules("en").expect("supported locale");
    c.bench_function("singularize_en", |b| {
        b.iter(|| rules.singularize(black_box("categories")));
    });
}

fn bench_pluralize_en(c: &mut Criterion) {
    let rules = language_rules("en").expect("supported locale");
    c.bench_function("pluralize_en", |b| {
        b.iter(|| rules.pluralize(black_box("category")));
    });
}

fn bench_singularize_es(c: &mut Criterion) {
    let rules = language_rules("es").expect("supported locale");
    c.bench_function("singularize_es", |b| {
        b.iter(|| rules.singularize(black_box("ciudades")));
    });
}

fn bench_pluralize_es(c: &mut Criterion) {
    let rules = language_rules("es").expect("supported locale");
    c.bench_function("pluralize_es", |b| {
        b.iter(|| rules.pluralize(black_box("ciudad")));
    });
}

fn bench_singularize_de(c: &mut Criterion) {
    let rules = language_rules("de").expect("supported locale");
    c.bench_function("singularize_de", |b| {
        b.iter(|| rules.singularize(black_box("produkte")));
    });
}

fn bench_pluralize_de(c: &mut Criterion) {
    let rules = language_rules("de").expect("supported locale");
    c.bench_function("pluralize_de", |b| {
        b.iter(|| rules.pluralize(black_box("produkt")));
    });
}

fn bench_singularize_fr(c: &mut Criterion) {
    let rules = language_rules("fr").expect("supported locale");
    c.bench_function("singularize_fr", |b| {
        b.iter(|| rules.singularize(black_box("journaux")));
    });
}

fn bench_singularize_invariant(c: &mut Criterion) {
    let rules = language_rules("ja").expect("supported locale");
    c.bench_function("singularize_invariant", |b| {
        b.iter(|| rules.singularize(black_box("user")));
    });
}

fn bench_pluralize_tr(c: &mut Criterion) {
    let rules = language_rules("tr").expect("supported locale");
    c.bench_function("pluralize_tr", |b| {
        b.iter(|| rules.pluralize(black_box("kullanici")));
    });
}

fn bench_language_rules_en(c: &mut Criterion) {
    c.bench_function("language_rules_en", |b| {
        b.iter(|| language_rules(black_box("en")).expect("supported locale"));
    });
}

fn bench_language_rules_en_us(c: &mut Criterion) {
    c.bench_function("language_rules_en_us", |b| {
        b.iter(|| language_rules(black_box("en-US")).expect("supported locale"));
    });
}

fn bench_language_rules_unknown(c: &mut Criterion) {
    c.bench_function("language_rules_unknown", |b| {
        b.iter(|| {
            let _ = language_rules(black_box("xx"));
        });
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
    bench_pluralize_tr,
    bench_language_rules_en,
    bench_language_rules_en_us,
    bench_language_rules_unknown,
);
criterion_main!(benches);
