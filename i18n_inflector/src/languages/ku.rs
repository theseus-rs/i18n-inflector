//! Kurdish (ku) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ku",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Kurdish noun to its singular form.
///
/// Handles `-an` and `-en` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("an")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("en")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Kurdish noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}an").into(), format!("{name}en").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_an_suffix() {
        assert_eq!(singularize("daristan"), "darist");
        assert_eq!(singularize("minan"), "min");
    }

    #[test]
    fn test_singularize_en_suffix() {
        assert_eq!(singularize("pirtken"), "pirtk");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("dar"), "dar");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("an"), "an");
        assert_eq!(singularize("en"), "en");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("dar");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "daran"));
        assert!(result.iter().any(|v| v == "daren"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
