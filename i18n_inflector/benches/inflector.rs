use criterion::{Criterion, criterion_group, criterion_main};
use i18n_inflector::{InflectionRequest, LexicalClassId, language_profile};
use std::hint::black_box;

fn bench_profile_resolution(c: &mut Criterion) {
    c.bench_function("profile_en_us", |b| {
        b.iter(|| language_profile(black_box("en-US")));
    });
}

fn bench_verified_lexeme(c: &mut Criterion) {
    let Ok(profile) = language_profile("en") else {
        return;
    };
    c.bench_function("verified_en_child", |b| {
        b.iter(|| profile.inflect(InflectionRequest::plural(black_box("child"))));
    });
}

fn bench_productive_class(c: &mut Criterion) {
    let Ok(profile) = language_profile("en") else {
        return;
    };
    c.bench_function("class_en_regular_s", |b| {
        b.iter(|| {
            profile.inflect(
                InflectionRequest::plural(black_box("project"))
                    .lexical_class(LexicalClassId::new("regular-s")),
            )
        });
    });
}

fn bench_invariant_profile(c: &mut Criterion) {
    let Ok(profile) = language_profile("ja") else {
        return;
    };
    c.bench_function("invariant_ja", |b| {
        b.iter(|| profile.inflect(InflectionRequest::plural(black_box("猫"))));
    });
}

criterion_group!(
    benches,
    bench_profile_resolution,
    bench_verified_lexeme,
    bench_productive_class,
    bench_invariant_profile,
);
criterion_main!(benches);
