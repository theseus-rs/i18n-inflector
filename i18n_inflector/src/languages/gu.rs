//! Gujarati (gu) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "gu",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Gujarati noun (Latin transliteration) to its singular form.
///
/// Handles `-o` and `-on` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("on")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('o')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Gujarati noun (Latin transliteration).
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}o").into(), format!("{name}on").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_on_suffix() {
        assert_eq!(singularize("chokraon"), "chokra");
    }

    #[test]
    fn test_singularize_o_suffix() {
        assert_eq!(singularize("chokrao"), "chokra");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("chokra"), "chokra");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "on" -> strip "on" -> empty stem -> skip; strip 'o' -> no match (ends in 'n') -> "on"
        assert_eq!(singularize("on"), "on");
        assert_eq!(singularize("o"), "o");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("chokra");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "chokrao"));
        assert!(result.iter().any(|v| v == "chokraon"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
