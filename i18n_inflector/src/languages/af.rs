//! Afrikaans (af) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "af",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Afrikaans noun to its singular form.
///
/// Handles `-e` and `-s` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('e')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('s')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Afrikaans noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}e").into(), format!("{name}s").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_e_suffix() {
        assert_eq!(singularize("honde"), "hond");
        assert_eq!(singularize("bome"), "bom");
    }

    #[test]
    fn test_singularize_s_suffix() {
        assert_eq!(singularize("motors"), "motor");
        assert_eq!(singularize("tafels"), "tafel");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kat"), "kat");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("e"), "e");
        assert_eq!(singularize("s"), "s");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kat");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "kate"));
        assert!(result.iter().any(|v| v == "kats"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
