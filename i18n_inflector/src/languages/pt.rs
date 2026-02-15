//! Portuguese (pt) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "pt",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Portuguese noun to its singular form.
///
/// Handles `-es` plurals for consonant-ending words and regular `-s` plurals.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("es")
        && (stem.ends_with('r')
            || stem.ends_with('z')
            || stem.ends_with('l')
            || stem.ends_with('n')
            || stem.ends_with('s'))
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

/// Returns a list of possible plural forms for a Portuguese noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}s").into()];
    if name.ends_with('r')
        || name.ends_with('z')
        || name.ends_with('l')
        || name.ends_with('n')
        || name.ends_with('s')
    {
        candidates.push(format!("{name}es").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("clientes"), "cliente");
        assert_eq!(singularize("produtos"), "produto");
        assert_eq!(singularize("flores"), "flor");
        assert_eq!(singularize("animais"), "animai");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("produto"), "produto");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("produto");
        assert!(result.iter().any(|v| v == "produtos"));

        let result = pluralize("flor");
        assert!(result.iter().any(|v| v == "flores"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
