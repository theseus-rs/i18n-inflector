//! Aymara (ay) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Aymara noun to its singular form.
///
/// Aymara uses `-naka` as the plural suffix.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ay", "utanaka").unwrap(), "uta");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("naka")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Aymara noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ay", "uta").unwrap();
/// assert!(result.iter().any(|v| v == "utanaka"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}naka").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_naka_suffix() {
        assert_eq!(singularize("utanaka"), "uta");
        assert_eq!(singularize("jaqinaka"), "jaqi");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("uta"), "uta");
    }

    #[test]
    fn test_singularize_suffix_only_input() {
        assert_eq!(singularize("naka"), "naka");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("uta");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "utanaka"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
