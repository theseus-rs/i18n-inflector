//! Turkish (tr) inflection rules.
//!
//! Also used for Azerbaijani (az), Bashkir (ba), Chuvash (cv), Kazakh (kk), Kyrgyz (ky),
//! Turkmen (tk), Tatar (tt), Uyghur (ug), and Uzbek (uz).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Turkish noun to its singular form.
///
/// Turkish plurals use `-lar` (back vowel harmony) or `-ler` (front vowel
/// harmony).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("tr", "kullanicilar").unwrap(), "kullanici");
/// assert_eq!(singularize("tr", "urunler").unwrap(), "urun");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("lar")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ler")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Turkish noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("tr", "kullanici").unwrap();
/// assert!(result.iter().any(|v| v == "kullanicilar"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}lar").into(), format!("{name}ler").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("kullanicilar"), "kullanici");
        assert_eq!(singularize("urunler"), "urun");
        assert_eq!(singularize("siparisler"), "siparis");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kullanici"), "kullanici");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("kullanici");
        assert!(result.iter().any(|v| v == "kullanicilar"));
        assert!(result.iter().any(|v| v == "kullaniciler"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
