//! Irish (ga) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Irish noun to its singular form.
///
/// Handles `-i` and `-a` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ga", "usaideoiri").unwrap(), "usaideoir");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('a')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Irish noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ga", "usaideoir").unwrap();
/// assert!(result.iter().any(|v| v == "usaideoiri"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}i").into(), format!("{name}a").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_i_suffix() {
        assert_eq!(singularize("usaideoiri"), "usaideoir");
        assert_eq!(singularize("cairdi"), "caird");
    }

    #[test]
    fn test_singularize_a_suffix() {
        assert_eq!(singularize("barda"), "bard");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("usaideoir"), "usaideoir");
        assert_eq!(singularize("leabhair"), "leabhair");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("i"), "i");
        assert_eq!(singularize("a"), "a");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("usaideoir");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "usaideoiri"));
        assert!(result.iter().any(|v| v == "usaideoira"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
