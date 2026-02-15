//! Armenian (hy) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Armenian noun (Latin transliteration) to its singular form.
///
/// Handles `-ner` and `-er` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("hy", "girqer").unwrap(), "girq");
/// assert_eq!(singularize("hy", "txaner").unwrap(), "txa");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ner")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("er")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an Armenian noun (Latin transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("hy", "girq").unwrap();
/// assert!(result.iter().any(|v| v == "girqer"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}ner").into(), format!("{name}er").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ner_suffix() {
        assert_eq!(singularize("txaner"), "txa");
        assert_eq!(singularize("mardner"), "mard");
    }

    #[test]
    fn test_singularize_er_suffix() {
        assert_eq!(singularize("girqer"), "girq");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("girq"), "girq");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "ner" -> empty -> skip; "er" -> "n" non-empty -> "n"
        assert_eq!(singularize("ner"), "n");
        assert_eq!(singularize("er"), "er");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("girq");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "girqner"));
        assert!(result.iter().any(|v| v == "girqer"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
