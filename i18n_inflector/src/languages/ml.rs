//! Malayalam (ml) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ml",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Malayalam noun (Latin transliteration) to its singular form.
///
/// Handles `-kal` plural suffix.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("kal")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Malayalam noun (Latin transliteration).
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}kal").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_kal_suffix() {
        assert_eq!(singularize("pustakakal"), "pustaka");
        assert_eq!(singularize("manushyakal"), "manushya");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("pustaka"), "pustaka");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("kal"), "kal");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("pustaka");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "pustakakal"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
