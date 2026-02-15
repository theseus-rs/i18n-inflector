//! Hindi (hi) inflection rules.
//!
//! Also used for Bengali (bn) and Divehi (dv).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Indic noun (Latin transliteration) to its singular form.
///
/// Handles `-on` and `-en` plural suffixes common in Hindi and Bengali
/// transliterations.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("hi", "upyogakartaon").unwrap(), "upyogakarta");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("on")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("en")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Indic noun (Latin
/// transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("hi", "upyogakarta").unwrap();
/// assert!(result.iter().any(|v| v == "upyogakartaon"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}on").into(), format!("{name}en").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("upyogakartaon"), "upyogakarta");
        assert_eq!(singularize("shikshaken"), "shikshak");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("upyogakarta"), "upyogakarta");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("on"), "on");
        assert_eq!(singularize("en"), "en");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("upyogakarta");
        assert!(result.iter().any(|v| v == "upyogakartaon"));
        assert!(result.iter().any(|v| v == "upyogakartaen"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
