//! Esperanto (eo) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "eo",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Esperanto noun to its singular form.
///
/// Esperanto plurals are regular: nouns end in `-oj` (nominative plural).
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('j')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Esperanto noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}j").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_j_suffix() {
        assert_eq!(singularize("katoj"), "kato");
        assert_eq!(singularize("hundoj"), "hundo");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kato"), "kato");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("j"), "j");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kato");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "katoj"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
