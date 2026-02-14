//! Basque (eu) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Basque noun to its singular form.
///
/// Basque marks the plural with `-ak` (definite plural) and `-ek` (ergative
/// plural).
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("eu", "katuak").unwrap(), "katu");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ak")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ek")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Basque noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("eu", "katu").unwrap();
/// assert!(result.iter().any(|v| v == "katuak"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}ak").into(), format!("{name}ek").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ak_suffix() {
        assert_eq!(singularize("katuak"), "katu");
        assert_eq!(singularize("etxeak"), "etxe");
    }

    #[test]
    fn test_singularize_ek_suffix() {
        assert_eq!(singularize("gizonek"), "gizon");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("katu"), "katu");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        assert_eq!(singularize("ak"), "ak");
        assert_eq!(singularize("ek"), "ek");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("katu");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "katuak"));
        assert!(result.iter().any(|v| v == "katuek"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
