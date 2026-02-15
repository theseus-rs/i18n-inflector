//! German (de) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural German noun to its singular form.
///
/// Handles common German plural suffixes: `-en`, `-er`, `-e`, `-n`, `-s`.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("de", "kunden").unwrap(), "kund");
/// assert_eq!(singularize("de", "produkte").unwrap(), "produkt");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("en")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("er")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('e')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('n')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('s')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a German noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("de", "produkt").unwrap();
/// assert!(result.iter().any(|v| v == "produkte"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}e").into(),
        format!("{name}en").into(),
        format!("{name}er").into(),
        format!("{name}n").into(),
        format!("{name}s").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_en_suffix() {
        assert_eq!(singularize("kunden"), "kund");
        assert_eq!(singularize("frauen"), "frau");
    }

    #[test]
    fn test_singularize_er_suffix() {
        assert_eq!(singularize("kinder"), "kind");
    }

    #[test]
    fn test_singularize_e_suffix() {
        assert_eq!(singularize("produkte"), "produkt");
    }

    #[test]
    fn test_singularize_n_suffix() {
        // Words ending in 'n' but not 'en' hit the '-n' branch
        assert_eq!(singularize("lin"), "li");
    }

    #[test]
    fn test_singularize_s_suffix() {
        assert_eq!(singularize("autos"), "auto");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("produkt"), "produkt");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // Single-char suffixes return themselves when the stem would be empty
        assert_eq!(singularize("e"), "e");
        assert_eq!(singularize("n"), "n");
        assert_eq!(singularize("s"), "s");
        // "en" -> empty stem, skip -> 'n' branch: stem "e" non-empty -> "e"
        assert_eq!(singularize("en"), "e");
        // "er" -> empty stem, skip -> no further suffix match -> "er"
        assert_eq!(singularize("er"), "er");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("produkt");
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|v| v == "produkte"));
        assert!(result.iter().any(|v| v == "produkten"));
        assert!(result.iter().any(|v| v == "produkter"));
        assert!(result.iter().any(|v| v == "produktn"));
        assert!(result.iter().any(|v| v == "produkts"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 5);
    }
}
