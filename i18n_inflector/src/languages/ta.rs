//! Tamil (ta) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Tamil noun (Latin transliteration) to its singular form.
///
/// Handles `-kal` and `-gal` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ta", "pustakakal").unwrap(), "pustaka");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("kal")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("gal")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Tamil noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ta", "pustaka").unwrap();
/// assert!(result.iter().any(|v| v == "pustakakal"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}kal").into(), format!("{name}gal").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_kal_suffix() {
        assert_eq!(singularize("pustakakal"), "pustaka");
        assert_eq!(singularize("marangal"), "maran");
    }

    #[test]
    fn test_singularize_gal_suffix() {
        assert_eq!(singularize("penkal"), "pen");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("pustaka"), "pustaka");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("kal"), "kal");
        assert_eq!(singularize("gal"), "gal");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("pustaka");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "pustakakal"));
        assert!(result.iter().any(|v| v == "pustakagal"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
