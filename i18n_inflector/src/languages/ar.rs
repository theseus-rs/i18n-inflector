//! Arabic (ar) inflection rules.
//!
//! Also used for Hebrew (he) and Yiddish (yi).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Semitic noun (Latin transliteration) to its singular form.
///
/// Handles Hebrew/Arabic plural suffixes: `-im`, `-ot`, `-at`, `-in`.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("he", "meshtatfim").unwrap(), "meshtatf");
/// assert_eq!(singularize("ar", "mustahdimin").unwrap(), "mustahdim");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("im")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ot")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("at")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("in")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Semitic noun (Latin
/// transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("he", "meshtatef").unwrap();
/// assert!(result.iter().any(|v| v == "meshtatefim"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}im").into(),
        format!("{name}ot").into(),
        format!("{name}at").into(),
        format!("{name}in").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("meshtatfim"), "meshtatf");
        assert_eq!(singularize("mustahdimin"), "mustahdim");
        assert_eq!(singularize("produktin"), "produkt");
        assert_eq!(singularize("mishpachot"), "mishpach");
        assert_eq!(singularize("banat"), "ban");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("meshtatef"), "meshtatef");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("meshtatef");
        assert!(result.iter().any(|v| v == "meshtatefim"));
        assert!(result.iter().any(|v| v == "meshtatefot"));
        assert!(result.iter().any(|v| v == "meshtatefat"));
        assert!(result.iter().any(|v| v == "meshtatefin"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
