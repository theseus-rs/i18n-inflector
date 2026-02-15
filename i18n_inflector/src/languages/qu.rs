//! Quechua (qu) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "qu",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Quechua noun to its singular form.
///
/// Quechua uses `-kuna` as the plural suffix.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("kuna")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Quechua noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}kuna").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_kuna_suffix() {
        assert_eq!(singularize("wasikuna"), "wasi");
        assert_eq!(singularize("runakuna"), "runa");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("wasi"), "wasi");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("kuna"), "kuna");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("wasi");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "wasikuna"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
