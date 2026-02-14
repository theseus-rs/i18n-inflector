//! Telugu (te) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Telugu noun (Latin transliteration) to its singular form.
///
/// Handles `-lu` plural suffix.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("te", "pustakaalu").unwrap(), "pustakaa");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("lu")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Telugu noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("te", "pustakaa").unwrap();
/// assert!(result.iter().any(|v| v == "pustakaalu"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}lu").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_lu_suffix() {
        assert_eq!(singularize("pustakaalu"), "pustakaa");
        assert_eq!(singularize("pillalu"), "pilla");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("pustakaa"), "pustakaa");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("lu"), "lu");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("pustakaa");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "pustakaalu"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
