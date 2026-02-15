//! Russian (ru) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "ru",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Russian noun (Latin transliteration) to its singular form.
///
/// Handles common transliterated Russian plural patterns: `-y` -> `-a` and
/// stripping `-i`.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('y')
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Russian noun (Latin transliteration).
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}y").into());
        candidates.push(format!("{stem}i").into());
    }
    candidates.push(format!("{name}y").into());
    candidates.push(format!("{name}i").into());
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("produkty"), "produkta");
        assert_eq!(singularize("klienti"), "klient");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("klient"), "klient");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("klient");
        assert!(result.iter().any(|v| v == "klienty"));
        assert!(result.iter().any(|v| v == "klienti"));

        let result = pluralize("tablica");
        assert!(result.iter().any(|v| v == "tablicy"));
        assert!(result.iter().any(|v| v == "tablici"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
