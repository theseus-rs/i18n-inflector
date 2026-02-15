//! Spanish (es) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "es",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Spanish noun to its singular form.
///
/// Handles `-es` plurals for words ending in consonants (`d`, `r`, `n`, `l`, `z`, `j`, `s`) and
/// regular `-s` plurals.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("es")
        && (stem.ends_with('d')
            || stem.ends_with('r')
            || stem.ends_with('n')
            || stem.ends_with('l')
            || stem.ends_with('z')
            || stem.ends_with('j')
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

/// Returns a list of possible plural forms for a Spanish noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}s").into()];
    if name.ends_with('d')
        || name.ends_with('r')
        || name.ends_with('n')
        || name.ends_with('l')
        || name.ends_with('z')
        || name.ends_with('j')
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
        assert_eq!(singularize("usuarios"), "usuario");
        assert_eq!(singularize("ciudades"), "ciudad");
        assert_eq!(singularize("productos"), "producto");
        assert_eq!(singularize("clientes"), "cliente");
        assert_eq!(singularize("animales"), "animal");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("usuario"), "usuario");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("usuario");
        assert!(result.iter().any(|v| v == "usuarios"));

        let result = pluralize("ciudad");
        assert!(result.iter().any(|v| v == "ciudades"));

        let result = pluralize("animal");
        assert!(result.iter().any(|v| v == "animales"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
