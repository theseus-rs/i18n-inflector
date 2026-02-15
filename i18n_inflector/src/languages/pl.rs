//! Polish (pl) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Polish noun to its singular form.
///
/// Handles `-y` and `-i` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("pl", "produkty").unwrap(), "produkt");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('y')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Polish noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("pl", "produkt").unwrap();
/// assert!(result.iter().any(|v| v == "produkty"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}y").into(), format!("{name}i").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("produkty"), "produkt");
        assert_eq!(singularize("klienci"), "klienc");
        assert_eq!(singularize("zamowieni"), "zamowien");
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
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
