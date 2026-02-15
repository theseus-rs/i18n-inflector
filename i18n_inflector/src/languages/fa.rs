//! Persian (fa) inflection rules.
//!
//! Also used for Ossetian (os) and Tajik (tg).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Persian noun (Latin transliteration) to its singular form.
///
/// Handles `-ha` and `-an` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("fa", "ketabha").unwrap(), "ketab");
/// assert_eq!(singularize("fa", "daneshan").unwrap(), "danesh");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ha")
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

/// Returns a list of possible plural forms for a Persian noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("fa", "ketab").unwrap();
/// assert!(result.iter().any(|v| v == "ketabha"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}ha").into(), format!("{name}an").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ha_suffix() {
        assert_eq!(singularize("ketabha"), "ketab");
        assert_eq!(singularize("mashinha"), "mashin");
    }

    #[test]
    fn test_singularize_an_suffix() {
        assert_eq!(singularize("daneshan"), "danesh");
        assert_eq!(singularize("dokhtan"), "dokht");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("ketab"), "ketab");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("ha"), "ha");
        assert_eq!(singularize("an"), "an");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("ketab");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "ketabha"));
        assert!(result.iter().any(|v| v == "ketaban"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
