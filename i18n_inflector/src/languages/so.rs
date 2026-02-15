//! Somali (so) inflection rules.
//!
//! Also used for Oromo (om).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Somali noun to its singular form.
///
/// Handles `-oyin`, `-yo`, and `-o` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("so", "buugaagyo").unwrap(), "buugaag");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("oyin")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("yo")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('o')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Somali noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("so", "buug").unwrap();
/// assert!(result.iter().any(|v| v == "buugo"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}oyin").into(),
        format!("{name}yo").into(),
        format!("{name}o").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_oyin_suffix() {
        assert_eq!(singularize("magaaloyin"), "magaal");
    }

    #[test]
    fn test_singularize_yo_suffix() {
        assert_eq!(singularize("buugaagyo"), "buugaag");
    }

    #[test]
    fn test_singularize_o_suffix() {
        assert_eq!(singularize("buugo"), "buug");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("buug"), "buug");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "oyin" -> empty -> skip; "yo" -> no; 'o' -> no -> "oyin"
        assert_eq!(singularize("oyin"), "oyin");
        // "yo" -> empty -> skip; 'o' -> "y" non-empty -> "y"
        assert_eq!(singularize("yo"), "y");
        assert_eq!(singularize("o"), "o");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("buug");
        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|v| v == "buugoyin"));
        assert!(result.iter().any(|v| v == "buugyo"));
        assert!(result.iter().any(|v| v == "buugo"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 3);
    }
}
