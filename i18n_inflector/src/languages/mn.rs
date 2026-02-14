//! Mongolian (mn) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Mongolian noun (Latin transliteration) to its singular form.
///
/// Handles `-ууд`/`-uud` and `-нууд`/`-nuud` plural suffixes (in Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("mn", "nomuud").unwrap(), "nom");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("nuud")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("uud")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Mongolian noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("mn", "nom").unwrap();
/// assert!(result.iter().any(|v| v == "nomuud"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}nuud").into(), format!("{name}uud").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_nuud_suffix() {
        assert_eq!(singularize("morinuud"), "mori");
    }

    #[test]
    fn test_singularize_uud_suffix() {
        assert_eq!(singularize("nomuud"), "nom");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("nom"), "nom");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "nuud" -> empty -> skip; "uud" -> "n" non-empty -> "n"
        assert_eq!(singularize("nuud"), "n");
        assert_eq!(singularize("uud"), "uud");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("nom");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "nomnuud"));
        assert!(result.iter().any(|v| v == "nomuud"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
