//! Lithuanian (lt) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "lt",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Lithuanian noun to its singular form.
///
/// Handles `-ai` -> `-as`, `-os` -> `-a`, and `-es` -> `-e` transformations.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ai")
        && !stem.is_empty()
    {
        return format!("{stem}as").into();
    }
    if let Some(stem) = name.strip_suffix("os")
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    if let Some(stem) = name.strip_suffix("es")
        && !stem.is_empty()
    {
        return format!("{stem}e").into();
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Lithuanian noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix("as") {
        candidates.push(format!("{stem}ai").into());
    }
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}os").into());
    }
    if let Some(stem) = name.strip_suffix('e') {
        candidates.push(format!("{stem}es").into());
    }
    if candidates.is_empty() {
        candidates.push(format!("{name}ai").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ai_suffix() {
        assert_eq!(singularize("vartotojai"), "vartotojas");
    }

    #[test]
    fn test_singularize_os_suffix() {
        assert_eq!(singularize("knygos"), "knyga");
    }

    #[test]
    fn test_singularize_es_suffix() {
        assert_eq!(singularize("gatves"), "gatve");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("vartotojas"), "vartotojas");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("ai"), "ai");
        assert_eq!(singularize("os"), "os");
        assert_eq!(singularize("es"), "es");
    }

    #[test]
    fn test_pluralize_as_suffix() {
        let result = pluralize("vartotojas");
        assert!(result.iter().any(|v| v == "vartotojai"));
    }

    #[test]
    fn test_pluralize_a_suffix() {
        let result = pluralize("knyga");
        assert!(result.iter().any(|v| v == "knygos"));
    }

    #[test]
    fn test_pluralize_e_suffix() {
        let result = pluralize("gatve");
        assert!(result.iter().any(|v| v == "gatves"));
    }

    #[test]
    fn test_pluralize_no_known_suffix() {
        let result = pluralize("produkt");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "produktai"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
