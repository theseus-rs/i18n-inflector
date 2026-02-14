# i18n Inflector

[![Documentation](https://docs.rs/i18n_inflector/badge.svg)](https://docs.rs/i18n_inflector)
[![Code Coverage](https://codecov.io/gh/theseus-rs/i18n-inflector/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/i18n-inflector)
[![Latest version](https://img.shields.io/crates/v/i18n_inflector.svg)](https://crates.io/crates/i18n_inflector)
[![License](https://img.shields.io/crates/l/i18n_inflector)](https://github.com/theseus-rs/i18n-inflector#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

i18n_inflector is a Rust crate that provides a comprehensive set of inflection rules for multiple languages, allowing
you to easily convert words between different forms (e.g., singular to plural, plural to singular) in an
internationalized context.

## Getting Started

Add `i18n_inflector` to your `Cargo.toml`:

```toml
[dependencies]
i18n_inflector = "0.1"
```

### Usage

```rust
use i18n_inflector::{singularize, pluralize};

// English
assert_eq!(singularize("en", "users").unwrap(), "user");
assert_eq!(singularize("en", "categories").unwrap(), "category");

let plurals = pluralize("en", "user").unwrap();
assert!(plurals.contains(&"users".to_string()));

// Spanish
assert_eq!(singularize("es", "ciudades").unwrap(), "ciudad");

// French
assert_eq!(singularize("fr", "journaux").unwrap(), "journal");

// Japanese (no morphological plural)
assert_eq!(singularize("ja", "user").unwrap(), "user");

// Unsupported locale returns an error
assert!(singularize("xx", "users").is_err());
```

The first argument is always an ISO 639 two-letter language code. If the locale
is not recognized, English rules are used as a fallback.

### Supported Languages

| Code | Language   | Code | Language   | Code | Language   |
|------|-----------|------|-----------|------|-----------|
| ar   | Arabic    | ga   | Irish     | mt   | Maltese   |
| be   | Belarusian| he   | Hebrew    | nl   | Dutch     |
| bg   | Bulgarian | hi   | Hindi     | no   | Norwegian |
| bn   | Bengali   | hr   | Croatian  | pl   | Polish    |
| cs   | Czech     | hu   | Hungarian | pt   | Portuguese|
| da   | Danish    | is   | Icelandic | ro   | Romanian  |
| de   | German    | it   | Italian   | ru   | Russian   |
| el   | Greek     | ja   | Japanese  | sk   | Slovak    |
| en   | English   | jv   | Javanese  | sl   | Slovenian |
| es   | Spanish   | ka   | Georgian  | sq   | Albanian  |
| et   | Estonian  | ko   | Korean    | sr   | Serbian   |
| fi   | Finnish   | lt   | Lithuanian| sv   | Swedish   |
| fr   | French    | lv   | Latvian   | th   | Thai      |
| tr   | Turkish   | uk   | Ukrainian | vi   | Vietnamese|
| mk   | Macedonian| ms   | Malay     | yi   | Yiddish   |
| zh   | Chinese   |      |           |      |           |

See the [`examples/basic_usage`](examples/basic_usage) directory for a
runnable example.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
