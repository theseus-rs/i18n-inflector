//! Amharic (am) inflection rules.
//!
//! Also used for Tigrinya (ti).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Amharic noun (Latin transliteration) to its singular form.
///
/// Handles `-och` and `-at` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("am", "betoch").unwrap(), "bet");
/// assert_eq!(singularize("am", "mekinat").unwrap(), "mekin");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("och")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("at")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Amharic noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("am", "bet").unwrap();
/// assert!(result.iter().any(|v| v == "betoch"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}och").into(), format!("{name}at").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_och_suffix() {
        assert_eq!(singularize("betoch"), "bet");
        assert_eq!(singularize("sewoch"), "sew");
    }

    #[test]
    fn test_singularize_at_suffix() {
        assert_eq!(singularize("mekinat"), "mekin");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("bet"), "bet");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("och"), "och");
        assert_eq!(singularize("at"), "at");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("bet");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "betoch"));
        assert!(result.iter().any(|v| v == "betat"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
