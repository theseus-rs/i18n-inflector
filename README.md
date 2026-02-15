# i18n_inflector

[![Documentation](https://docs.rs/i18n_inflector/badge.svg)](https://docs.rs/i18n_inflector)
[![Code Coverage](https://codecov.io/gh/theseus-rs/i18n-inflector/branch/main/graph/badge.svg)](https://codecov.io/gh/theseus-rs/i18n-inflector)
[![Latest version](https://img.shields.io/crates/v/i18n_inflector.svg)](https://crates.io/crates/i18n_inflector)
[![License](https://img.shields.io/crates/l/i18n_inflector)](https://github.com/theseus-rs/i18n-inflector#license)
[![Semantic Versioning](https://img.shields.io/badge/%E2%9A%99%EF%B8%8F_SemVer-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)

i18n_inflector is a Rust crate that provides a comprehensive set of inflection rules for multiple languages, allowing
you to easily convert words between different forms (e.g., singular to plural, plural to singular) in an
internationalized context.

## Example

```rust
use i18n_inflector::{singularize, pluralize};

// Singularize
assert_eq!(singularize("en", "users").unwrap(), "user");
assert_eq!(singularize("es", "ciudades").unwrap(), "ciudad");

// Pluralize
let plurals = pluralize("en", "user").unwrap();
assert!(plurals.contains(&"users".to_string()));

// Unsupported locale returns an error
assert!(singularize("xx", "users").is_err());
```

## Supported Languages

The following ISO 639-1 language codes are supported:

| Code | Language          |
|------|-------------------|
| af   | Afrikaans         |
| am   | Amharic           |
| an   | Aragonese         |
| ar   | Arabic            |
| as   | Assamese          |
| az   | Azerbaijani       |
| be   | Belarusian        |
| bg   | Bulgarian         |
| bn   | Bengali           |
| bo   | Tibetan           |
| br   | Breton            |
| bs   | Bosnian           |
| ca   | Catalan           |
| co   | Corsican          |
| cs   | Czech             |
| cy   | Welsh             |
| da   | Danish            |
| de   | German            |
| el   | Greek             |
| en   | English           |
| eo   | Esperanto         |
| es   | Spanish           |
| et   | Estonian          |
| eu   | Basque            |
| fa   | Persian           |
| fi   | Finnish           |
| fo   | Faroese           |
| fr   | French            |
| fy   | Western Frisian   |
| ga   | Irish             |
| gd   | Scottish Gaelic   |
| gl   | Galician          |
| gu   | Gujarati          |
| gv   | Manx              |
| ha   | Hausa             |
| he   | Hebrew            |
| hi   | Hindi             |
| hr   | Croatian          |
| ht   | Haitian Creole    |
| hu   | Hungarian         |
| hy   | Armenian          |
| id   | Indonesian        |
| ig   | Igbo              |
| is   | Icelandic         |
| it   | Italian           |
| ja   | Japanese          |
| jv   | Javanese          |
| ka   | Georgian          |
| kk   | Kazakh            |
| km   | Khmer             |
| kn   | Kannada           |
| ko   | Korean            |
| ku   | Kurdish           |
| kw   | Cornish           |
| ky   | Kyrgyz            |
| la   | Latin             |
| lb   | Luxembourgish     |
| lo   | Lao               |
| lt   | Lithuanian        |
| lv   | Latvian           |
| mg   | Malagasy          |
| mi   | Māori             |
| mk   | Macedonian        |
| ml   | Malayalam         |
| mn   | Mongolian         |
| mr   | Marathi           |
| ms   | Malay             |
| mt   | Maltese           |
| my   | Burmese           |
| nb   | Norwegian Bokmål  |
| nd   | Northern Ndebele  |
| ne   | Nepali            |
| nl   | Dutch             |
| nn   | Norwegian Nynorsk |
| no   | Norwegian         |
| nr   | Southern Ndebele  |
| oc   | Occitan           |
| or   | Odia              |
| pa   | Punjabi           |
| pl   | Polish            |
| ps   | Pashto            |
| pt   | Portuguese        |
| qu   | Quechua           |
| rm   | Romansh           |
| ro   | Romanian          |
| ru   | Russian           |
| rw   | Kinyarwanda       |
| sc   | Sardinian         |
| sd   | Sindhi            |
| si   | Sinhala           |
| sk   | Slovak            |
| sl   | Slovenian         |
| sm   | Samoan            |
| sn   | Shona             |
| so   | Somali            |
| sq   | Albanian          |
| sr   | Serbian           |
| ss   | Swati             |
| st   | Southern Sotho    |
| su   | Sundanese         |
| sv   | Swedish           |
| sw   | Swahili           |
| ta   | Tamil             |
| te   | Telugu            |
| tg   | Tajik             |
| th   | Thai              |
| ti   | Tigrinya          |
| tk   | Turkmen           |
| tl   | Tagalog           |
| tn   | Tswana            |
| tr   | Turkish           |
| ts   | Tsonga            |
| tt   | Tatar             |
| ug   | Uyghur            |
| uk   | Ukrainian         |
| ur   | Urdu              |
| uz   | Uzbek             |
| ve   | Venda             |
| vi   | Vietnamese        |
| wo   | Wolof             |
| xh   | Xhosa             |
| yi   | Yiddish           |
| yo   | Yoruba            |
| zh   | Chinese           |
| zu   | Zulu              |

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
