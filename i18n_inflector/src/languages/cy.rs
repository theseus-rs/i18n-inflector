//! Welsh (cy) inflection rules.
//!
//! Also used for Cornish (kw).

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "cy",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Welsh noun to its singular form.
///
/// Handles `-iau`, `-au`, `-oedd`, and `-od` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("iau")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("au")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("oedd")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("od")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Welsh noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}iau").into(),
        format!("{name}au").into(),
        format!("{name}oedd").into(),
        format!("{name}od").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_iau_suffix() {
        assert_eq!(singularize("llyfiau"), "llyf");
    }

    #[test]
    fn test_singularize_au_suffix() {
        assert_eq!(singularize("pennau"), "penn");
    }

    #[test]
    fn test_singularize_oedd_suffix() {
        assert_eq!(singularize("ceiroedd"), "ceir");
    }

    #[test]
    fn test_singularize_od_suffix() {
        assert_eq!(singularize("cathod"), "cath");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("cath"), "cath");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "iau" -> strip "iau" empty -> skip; strip "au" -> "i" non-empty -> "i"
        assert_eq!(singularize("iau"), "i");
        assert_eq!(singularize("au"), "au");
        assert_eq!(singularize("oedd"), "oedd");
        assert_eq!(singularize("od"), "od");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("cath");
        assert_eq!(result.len(), 4);
        assert!(result.iter().any(|v| v == "cathiau"));
        assert!(result.iter().any(|v| v == "cathau"));
        assert!(result.iter().any(|v| v == "cathoedd"));
        assert!(result.iter().any(|v| v == "cathod"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 4);
    }
}
