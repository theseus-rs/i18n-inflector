//! Latvian (lv) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

/// Converts a plural Latvian noun to its singular form.
///
/// Handles `-i` -> `-s` and `-as` -> `-a` transformations.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("lv", "lietotaji").unwrap(), "lietotajs");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return format!("{stem}s").into();
    }
    if let Some(stem) = name.strip_suffix("as")
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Latvian noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("lv", "lietotajs").unwrap();
/// assert!(result.iter().any(|v| v == "lietotaji"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix('s') {
        candidates.push(format!("{stem}i").into());
    }
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}as").into());
    }
    if candidates.is_empty() {
        candidates.push(format!("{name}i").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("lietotaji"), "lietotajs");
        assert_eq!(singularize("produkti"), "produkts");
        assert_eq!(singularize("graamatas"), "graamata");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("lietotajs"), "lietotajs");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("lietotajs");
        assert!(result.iter().any(|v| v == "lietotaji"));

        let result = pluralize("graamata");
        assert!(result.iter().any(|v| v == "graamatas"));
    }

    #[test]
    fn test_pluralize_no_known_suffix() {
        let result = pluralize("produkt");
        assert!(result.iter().any(|v| v == "produkti"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
