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

// English
assert_eq!(singularize("en", "users").unwrap(), "user");
assert_eq!(singularize("en", "categories").unwrap(), "category");
assert_eq!(singularize("en-US", "children").unwrap(), "child");

let plurals = pluralize("en", "user").unwrap();
assert!(plurals.iter().any(|v| v == "users"));

// Spanish
assert_eq!(singularize("es", "ciudades").unwrap(), "ciudad");

// French
assert_eq!(singularize("fr", "journaux").unwrap(), "journal");

// Japanese
assert_eq!(singularize("ja", "user").unwrap(), "user");

// Unsupported locale returns an error
assert!(singularize("xx", "users").is_err());
```

## Supported Languages

The following ISO 639-1 language codes are supported:

| Code | Language          | Code | Language          | Code | Language          |
|------|-------------------|------|-------------------|------|-------------------|
| aa   | Afar              | ho   | Hiri Motu         | om   | Oromo             |
| ab   | Abkhaz            | hr   | Croatian          | or   | Odia              |  
| ae   | Avestan           | ht   | Haitian Creole    | os   | Ossetian          |  
| af   | Afrikaans         | hu   | Hungarian         | pa   | Punjabi           |  
| ak   | Akan              | hy   | Armenian          | pi   | Pali              |  
| am   | Amharic           | ia   | Interlingua       | pl   | Polish            |  
| an   | Aragonese         | id   | Indonesian        | ps   | Pashto            |  
| ar   | Arabic            | ie   | Interlingue       | pt   | Portuguese        |  
| as   | Assamese          | ig   | Igbo              | qu   | Quechua           |  
| av   | Avar              | ii   | Sichuan Yi        | rm   | Romansh           |  
| ay   | Aymara            | ik   | Inupiaq           | ro   | Romanian          |  
| az   | Azerbaijani       | is   | Icelandic         | ru   | Russian           |  
| ba   | Bashkir           | it   | Italian           | rw   | Kinyarwanda       |  
| be   | Belarusian        | iu   | Inuktitut         | sa   | Sanskrit          |  
| bg   | Bulgarian         | ja   | Japanese          | sc   | Sardinian         |  
| bi   | Bislama           | jv   | Javanese          | sd   | Sindhi            |  
| bm   | Bambara           | ka   | Georgian          | se   | Northern Sami     |  
| bn   | Bengali           | kg   | Kongo             | sg   | Sango             |  
| bo   | Tibetan           | ki   | Kikuyu            | si   | Sinhala           |  
| br   | Breton            | kj   | Kuanyama          | sk   | Slovak            |  
| bs   | Bosnian           | kk   | Kazakh            | sl   | Slovenian         |  
| ca   | Catalan           | km   | Khmer             | sm   | Samoan            |  
| ce   | Chechen           | kn   | Kannada           | sn   | Shona             |  
| ch   | Chamorro          | ko   | Korean            | so   | Somali            |  
| co   | Corsican          | ku   | Kurdish           | sq   | Albanian          |  
| cs   | Czech             | kv   | Komi              | sr   | Serbian           |  
| cu   | Church Slavonic   | kw   | Cornish           | ss   | Swati             |  
| cv   | Chuvash           | ky   | Kyrgyz            | st   | Southern Sotho    |  
| cy   | Welsh             | la   | Latin             | su   | Sundanese         |  
| da   | Danish            | lb   | Luxembourgish     | sv   | Swedish           |  
| de   | German            | lg   | Luganda           | sw   | Swahili           |  
| dv   | Divehi            | li   | Limburgish        | ta   | Tamil             |  
| dz   | Dzongkha          | lo   | Lao               | te   | Telugu            |  
| ee   | Ewe               | lt   | Lithuanian        | tg   | Tajik             |  
| el   | Greek             | lu   | Luba-Katanga      | th   | Thai              |  
| en   | English           | lv   | Latvian           | ti   | Tigrinya          |  
| eo   | Esperanto         | mg   | Malagasy          | tk   | Turkmen           |  
| es   | Spanish           | mi   | Māori             | tl   | Tagalog           |  
| et   | Estonian          | mk   | Macedonian        | tn   | Tswana            |  
| eu   | Basque            | ml   | Malayalam         | tr   | Turkish           |  
| fa   | Persian           | mn   | Mongolian         | ts   | Tsonga            |  
| ff   | Fula              | mr   | Marathi           | tt   | Tatar             |  
| fi   | Finnish           | ms   | Malay             | ug   | Uyghur            |  
| fj   | Fijian            | mt   | Maltese           | uk   | Ukrainian         |  
| fo   | Faroese           | my   | Burmese           | ur   | Urdu              |  
| fr   | French            | nb   | Norwegian Bokmål  | uz   | Uzbek             |  
| fy   | Western Frisian   | nd   | Northern Ndebele  | ve   | Venda             |  
| ga   | Irish             | ne   | Nepali            | vi   | Vietnamese        |  
| gd   | Scottish Gaelic   | nl   | Dutch             | wa   | Walloon           |  
| gl   | Galician          | nn   | Norwegian Nynorsk | wo   | Wolof             |  
| gn   | Guarani           | no   | Norwegian         | xh   | Xhosa             |  
| gu   | Gujarati          | nr   | Southern Ndebele  | yi   | Yiddish           |  
| gv   | Manx              | nv   | Navajo            | yo   | Yoruba            |  
| ha   | Hausa             | ny   | Chichewa          | zh   | Chinese           |  
| he   | Hebrew            | oc   | Occitan           | zu   | Zulu              |  
| hi   | Hindi             | oj   | Ojibwe            |      |                   |

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
