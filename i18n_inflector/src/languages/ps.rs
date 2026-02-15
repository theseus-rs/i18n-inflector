//! Pashto (ps) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Pashto noun (Latin transliteration) to its singular form.
///
/// Handles `-una` and `-an` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ps", "kitabuna").unwrap(), "kitab");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("una")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("an")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Pashto noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ps", "kitab").unwrap();
/// assert!(result.iter().any(|v| v == "kitabuna"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}una").into(), format!("{name}an").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_una_suffix() {
        assert_eq!(singularize("kitabuna"), "kitab");
    }

    #[test]
    fn test_singularize_an_suffix() {
        assert_eq!(singularize("dokhturan"), "dokhtur");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kitab"), "kitab");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("una"), "una");
        assert_eq!(singularize("an"), "an");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kitab");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "kitabuna"));
        assert!(result.iter().any(|v| v == "kitaban"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
