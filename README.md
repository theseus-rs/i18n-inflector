# i18n_inflector

[![Documentation](https://docs.rs/i18n_inflector/badge.svg)](https://docs.rs/i18n_inflector)
[![Code Coverage](https://codecov.io/gh/theseus-rs/i18n-inflector/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/i18n-inflector)
[![Latest version](https://img.shields.io/crates/v/i18n_inflector.svg)](https://crates.io/crates/i18n_inflector)
[![License](https://img.shields.io/crates/l/i18n_inflector)](https://github.com/theseus-rs/i18n-inflector#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

`i18n_inflector` provides correctness-first dictionary-form noun inflection for all 183 ISO
639-1 language codes. It accepts BCP 47 language identifiers, normalizes lemmas to Unicode NFC,
and returns typed errors when a form cannot be generated without guessing.

The API intentionally accepts a singular dictionary lemma, not an arbitrary inflected word. Results
come from an attested conformance lexicon, an invariant-number profile, or an explicitly documented
productive lexical class.

## Example

```rust
use i18n_inflector::{InflectionRequest, LexicalClassId, language_profile};

fn main() -> i18n_inflector::Result<()> {
    let english = language_profile("en-US")?;

    assert_eq!(
        english.inflect(InflectionRequest::plural("child"))?.primary(),
        "children"
    );
    assert_eq!(
        english
            .inflect(
                InflectionRequest::plural("project")
                    .lexical_class(LexicalClassId::new("regular-s"))
            )?
            .primary(),
        "projects"
    );

    let japanese = language_profile("ja")?;
    assert_eq!(
        japanese.inflect(InflectionRequest::plural("猫"))?.primary(),
        "猫"
    );

    Ok(())
}
```

Use `LanguageProfile::capabilities()` to discover the selectors and lexical classes supported by a
profile.

## Scope

- Singular and plural dictionary/base or nominative forms
- All ISO 639-1 codes and selected script-specific BCP 47 profiles
- `no_std` environments with `alloc`
- Typed outcomes for unsupported locales, selectors, classes, lemmas, and absent forms

Dual, paucal, oblique case paradigms, and inflection from already-inflected input are outside the
current API contract.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
