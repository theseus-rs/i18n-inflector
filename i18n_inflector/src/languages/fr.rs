//! French (fr) inflection rules.
//!
//! Also used for Interlingua (ia), Interlingue (ie), Occitan (oc), Romansh (rm), and
//! Walloon (wa).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural French noun to its singular form.
///
/// Handles `-aux` -> `-al` transformation and regular `-s` plurals.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("fr", "utilisateurs").unwrap(), "utilisateur");
/// assert_eq!(singularize("fr", "journaux").unwrap(), "journal");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("aux")
        && !stem.is_empty()
    {
        return format!("{stem}al").into();
    }
    if let Some(stem) = name.strip_suffix('s')
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a French noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("fr", "utilisateur").unwrap();
/// assert!(result.iter().any(|v| v == "utilisateurs"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}s").into()];
    if let Some(stem) = name.strip_suffix("al") {
        candidates.push(format!("{stem}aux").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("utilisateurs"), "utilisateur");
        assert_eq!(singularize("produits"), "produit");
        assert_eq!(singularize("journaux"), "journal");
        assert_eq!(singularize("animaux"), "animal");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("utilisateur"), "utilisateur");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("utilisateur");
        assert!(result.iter().any(|v| v == "utilisateurs"));

        let result = pluralize("journal");
        assert!(result.iter().any(|v| v == "journaux"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
