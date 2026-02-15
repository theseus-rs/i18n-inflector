//! Greek (el) inflection rules (Latin transliteration).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

/// Converts a plural Greek noun (Latin transliteration) to its singular form.
///
/// Handles `-es` -> `-is` transformation and `-a` suffix stripping.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("el", "xristes").unwrap(), "xristis");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("es")
        && !stem.is_empty()
    {
        return format!("{stem}is").into();
    }
    if let Some(stem) = name.strip_suffix('a')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Greek noun (Latin
/// transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("el", "xristis").unwrap();
/// assert!(result.iter().any(|v| v == "xristes"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix("is") {
        candidates.push(format!("{stem}es").into());
    }
    candidates.push(format!("{name}a").into());
    candidates.push(format!("{name}es").into());
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("xristes"), "xristis");
        assert_eq!(singularize("themata"), "themat");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("xristis"), "xristis");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("xristis");
        assert!(result.iter().any(|v| v == "xristes"));

        let result = pluralize("themat");
        assert!(result.iter().any(|v| v == "themata"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
