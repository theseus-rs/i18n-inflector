//! Ukrainian (uk) inflection rules.
//!
//! Also used for Belarusian (be).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

/// Converts a plural East Slavic noun (Latin transliteration) to its singular form.
///
/// Handles `-y` -> `-a` transformation and `-i` suffix stripping, similar to Russian but with
/// distinct patterns for Ukrainian and Belarusian.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("uk", "produkty").unwrap(), "produkta");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('y')
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for an East Slavic noun (Latin
/// transliteration).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("uk", "produkt").unwrap();
/// assert!(result.iter().any(|v| v == "produkty"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}y").into());
        candidates.push(format!("{stem}i").into());
    }
    candidates.push(format!("{name}y").into());
    candidates.push(format!("{name}i").into());
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("produkty"), "produkta");
        assert_eq!(singularize("pradukty"), "pradukta");
        assert_eq!(singularize("klienti"), "klient");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("produkt"), "produkt");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("produkt");
        assert!(result.iter().any(|v| v == "produkty"));
        assert!(result.iter().any(|v| v == "produkti"));

        let result = pluralize("tablica");
        assert!(result.iter().any(|v| v == "tablicy"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
