//! Kannada (kn) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "kn",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Kannada noun (Latin transliteration) to its singular form.
///
/// Handles `-galu` plural suffix.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("galu")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Kannada noun (Latin transliteration).
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}galu").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_galu_suffix() {
        assert_eq!(singularize("pustakagalu"), "pustaka");
        assert_eq!(singularize("manegalu"), "mane");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("pustaka"), "pustaka");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("galu"), "galu");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("pustaka");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "pustakagalu"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
