//! Swahili (sw) inflection rules.
//!
//! Also used for Kinyarwanda (rw) and Shona (sn).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Swahili noun to its singular form.
///
/// Swahili uses prefix-based noun classes for plurality, but common suffixes `-ni` and `-zi` are
/// stripped when present.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("sw", "vitabuni").unwrap(), "vitabu");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ni")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("zi")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Swahili noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("sw", "kitabu").unwrap();
/// assert!(result.iter().any(|v| v == "kitabuni"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}ni").into(), format!("{name}zi").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ni_suffix() {
        assert_eq!(singularize("vitabuni"), "vitabu");
        assert_eq!(singularize("nyumbani"), "nyumba");
    }

    #[test]
    fn test_singularize_zi_suffix() {
        assert_eq!(singularize("kalazi"), "kala");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kitabu"), "kitabu");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("ni"), "ni");
        assert_eq!(singularize("zi"), "zi");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kitabu");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "kitabuni"));
        assert!(result.iter().any(|v| v == "kitabuzi"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
