//! Finnish (fi) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "fi",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Finnish noun to its singular form.
///
/// Finnish nominative plurals typically end in `-t`.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('t')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Finnish noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}t").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("tuotteet"), "tuottee");
        assert_eq!(singularize("asiakkaat"), "asiakkaa");
        assert_eq!(singularize("tietokannat"), "tietokanna");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("tuote"), "tuote");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("tuote");
        assert!(result.iter().any(|v| v == "tuotet"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
