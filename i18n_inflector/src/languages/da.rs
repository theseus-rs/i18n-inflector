//! Danish (da) inflection rules.
//!
//! Also used for Norwegian (no) and Swedish (sv).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Scandinavian noun to its singular form.
///
/// Handles `-ere`, `-er`, `-ar`, `-or`, and `-r` plural suffixes common
/// across Danish, Norwegian, and Swedish.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("sv", "produkter").unwrap(), "produkt");
/// assert_eq!(singularize("no", "brukere").unwrap(), "bruk");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ere")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("er")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ar")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("or")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('r')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Scandinavian noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("sv", "produkt").unwrap();
/// assert!(result.iter().any(|v| v == "produkter"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}ere").into(),
        format!("{name}er").into(),
        format!("{name}ar").into(),
        format!("{name}or").into(),
        format!("{name}r").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ere_suffix() {
        assert_eq!(singularize("brugere"), "brug");
        assert_eq!(singularize("brukere"), "bruk");
    }

    #[test]
    fn test_singularize_er_suffix() {
        assert_eq!(singularize("produkter"), "produkt");
    }

    #[test]
    fn test_singularize_ar_suffix() {
        assert_eq!(singularize("bilar"), "bil");
    }

    #[test]
    fn test_singularize_or_suffix() {
        assert_eq!(singularize("faktor"), "fakt");
    }

    #[test]
    fn test_singularize_r_suffix() {
        assert_eq!(singularize("dyr"), "dy");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("produkt"), "produkt");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // Only 'r' returns itself (empty stem after stripping)
        assert_eq!(singularize("r"), "r");
        // "ere" falls through all multi-char branches -> returns as-is
        assert_eq!(singularize("ere"), "ere");
        // "er" -> empty stem, skip -> 'r' branch: stem "e" non-empty -> "e"
        assert_eq!(singularize("er"), "e");
        // "ar" -> empty stem, skip -> 'r' branch: stem "a" non-empty -> "a"
        assert_eq!(singularize("ar"), "a");
        // "or" -> empty stem, skip -> 'r' branch: stem "o" non-empty -> "o"
        assert_eq!(singularize("or"), "o");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("produkt");
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|v| v == "produktere"));
        assert!(result.iter().any(|v| v == "produkter"));
        assert!(result.iter().any(|v| v == "produktar"));
        assert!(result.iter().any(|v| v == "produktor"));
        assert!(result.iter().any(|v| v == "produktr"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 5);
    }
}
