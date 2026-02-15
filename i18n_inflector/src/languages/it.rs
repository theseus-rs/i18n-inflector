//! Italian (it) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "it",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Italian noun to its singular form.
///
/// Handles masculine `-i` -> `-o` and feminine `-e` -> `-a` transformations.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return format!("{stem}o").into();
    }
    if let Some(stem) = name.strip_suffix('e')
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Italian noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix('o') {
        candidates.push(format!("{stem}i").into());
    }
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}e").into());
        candidates.push(format!("{stem}i").into());
    }
    if let Some(stem) = name.strip_suffix('e') {
        candidates.push(format!("{stem}i").into());
    }
    if candidates.is_empty() {
        candidates.push(format!("{name}i").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("prodotti"), "prodotto");
        assert_eq!(singularize("aziende"), "azienda");
        assert_eq!(singularize("clienti"), "cliento");
        assert_eq!(singularize("libri"), "libro");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("prodotto"), "prodotto");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("prodotto");
        assert!(result.iter().any(|v| v == "prodotti"));

        let result = pluralize("azienda");
        assert!(result.iter().any(|v| v == "aziende"));
        assert!(result.iter().any(|v| v == "aziendi"));

        let result = pluralize("studente");
        assert!(result.iter().any(|v| v == "studenti"));
    }

    #[test]
    fn test_pluralize_no_known_suffix() {
        let result = pluralize("bar");
        assert!(result.iter().any(|v| v == "bari"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
