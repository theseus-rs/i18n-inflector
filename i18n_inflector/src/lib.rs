//! Multilingual noun singularization and pluralization.
//!
//! `i18n_inflector` provides functions to convert between singular and plural forms of nouns across
//! ISO 639 two-letter language codes.
//!
//! # Quick Start
//!
//! ```
//! use i18n_inflector::{language_rules, LanguageRules};
//!
//! // English
//! let en = language_rules("en").unwrap();
//! assert_eq!(en.singularize("users"), "user");
//! assert_eq!(en.singularize("categories"), "category");
//! assert_eq!(en.singularize("children"), "child");
//!
//! let plurals = en.pluralize("user");
//! assert!(plurals.iter().any(|v| v == "users"));
//!
//! // Spanish
//! assert_eq!(language_rules("es").unwrap().singularize("ciudades"), "ciudad");
//!
//! // French
//! assert_eq!(language_rules("fr").unwrap().singularize("journaux"), "journal");
//!
//! // Japanese
//! assert_eq!(language_rules("ja").unwrap().singularize("user"), "user");
//!
//! // Unsupported locale returns an error
//! assert!(language_rules("xx").is_err());
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod error;
mod language_rules;
mod languages;
mod locale;
mod registry;

pub use error::{Error, Result};
pub use language_rules::{LanguageRuleSet, LanguageRules};

use crate::locale::normalize_locale;
use alloc::format;

/// Returns the [`LanguageRuleSet`] for the given locale.
///
/// The locale is normalized (case-insensitive, region suffixes stripped)
/// before lookup. Returns an error if the base language code is not recognized.
///
/// # Errors
///
/// Returns [`Error`] if the locale is not a supported language code.
///
/// # Examples
///
/// ```
/// use i18n_inflector::{language_rules, LanguageRules};
///
/// # fn main() -> i18n_inflector::Result<()> {
/// let rules = language_rules("en")?;
/// assert_eq!(rules.language(), "en");
/// assert_eq!(rules.singularize("users"), "user");
///
/// // Locale normalization
/// let rules = language_rules("en-US")?;
/// assert_eq!(rules.language(), "en");
///
/// // Unsupported locale
/// assert!(language_rules("xx").is_err());
/// # Ok(())
/// # }
/// ```
pub fn language_rules(locale: &str) -> Result<&'static LanguageRuleSet> {
    let normalized = normalize_locale(locale);
    registry::LANGUAGE_RULES_MAP
        .get(normalized.as_str())
        .copied()
        .ok_or_else(|| Error::new(format!("unsupported locale: {locale}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_language_rules_english() {
        let rules = language_rules("en").unwrap();
        assert_eq!(rules.language(), "en");
        assert_eq!(rules.singularize("users"), "user");
    }

    #[test]
    fn test_language_rules_locale_normalization() {
        let rules = language_rules("en-US").unwrap();
        assert_eq!(rules.language(), "en");

        let rules = language_rules("EN").unwrap();
        assert_eq!(rules.language(), "en");

        let rules = language_rules("EN-US").unwrap();
        assert_eq!(rules.language(), "en");

        let rules = language_rules("en_US").unwrap();
        assert_eq!(rules.language(), "en");
    }

    #[test]
    fn test_language_rules_unsupported() {
        let err = language_rules("xx").unwrap_err();
        assert_eq!(err.to_string(), "unsupported locale: xx");
    }

    #[test]
    fn test_language_rules_pluralize() {
        let rules = language_rules("en").unwrap();
        let result = rules.pluralize("child");
        assert_eq!(result, alloc::vec!["children"]);
    }

    #[test]
    fn test_language_rules_delegate() {
        let rules = language_rules("az").unwrap();
        assert_eq!(rules.language(), "az");
        // Delegates to Turkish
        assert_eq!(rules.singularize("kullanicilar"), "kullanici");
    }
}
