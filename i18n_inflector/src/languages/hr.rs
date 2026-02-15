//! Croatian (hr) inflection rules.
//!
//! Also used for Serbian (sr), Slovenian (sl), Macedonian (mk), and
//! Bulgarian (bg).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural South Slavic noun to its singular form.
///
/// Handles `-ovi`, `-evi`, `-ci` -> `-k`, `-i`, and `-a` plural suffixes common across Croatian,
/// Serbian, Slovenian, Macedonian, and Bulgarian.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("hr", "korisnici").unwrap(), "korisnik");
/// assert_eq!(singularize("bg", "produkti").unwrap(), "produkt");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ovi")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("evi")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ci")
        && !stem.is_empty()
    {
        return format!("{stem}k").into();
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix('a')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a South Slavic noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("hr", "korisnik").unwrap();
/// assert!(result.iter().any(|v| v == "korisnici"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![
        format!("{name}i").into(),
        format!("{name}ovi").into(),
        format!("{name}evi").into(),
    ];
    if let Some(stem) = name.strip_suffix('k') {
        candidates.push(format!("{stem}ci").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("korisnici"), "korisnik");
        assert_eq!(singularize("produkti"), "produkt");
        assert_eq!(singularize("gradovi"), "grad");
        assert_eq!(singularize("kraljevi"), "kralj");
        assert_eq!(singularize("zena"), "zen");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("korisnik"), "korisnik");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("korisnik");
        assert!(result.iter().any(|v| v == "korisnici"));
        assert!(result.iter().any(|v| v == "korisnikovi"));

        let result = pluralize("grad");
        assert!(result.iter().any(|v| v == "gradi"));
        assert!(result.iter().any(|v| v == "gradovi"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
