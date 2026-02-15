//! Zulu (zu) inflection rules.
//!
//! Also used for Xhosa (xh), Swati (ss), Southern Sotho (st), Tswana (tn), Tsonga (ts),
//! North Ndebele (nd), South Ndebele (nr), and Venda (ve).
//!
//! Bantu languages use prefix-based noun classes. This module handles common suffixes found in
//! Latin transliterations.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "zu",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Bantu noun to its singular form.
///
/// Handles the `-ni` plural/locative suffix.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ni")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Bantu noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}ini").into(), format!("{name}ni").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ini_suffix() {
        assert_eq!(singularize("izincwadini"), "izincwadi");
    }

    #[test]
    fn test_singularize_ni_suffix() {
        assert_eq!(singularize("isibhedleni"), "isibhedle");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("incwadi"), "incwadi");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "ni" -> strip "ni" -> "" empty -> skip -> "ni"
        assert_eq!(singularize("ni"), "ni");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("incwadi");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "incwadini"));
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
