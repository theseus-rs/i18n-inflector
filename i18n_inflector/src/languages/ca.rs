//! Catalan (ca) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Catalan noun to its singular form.
///
/// Handles `-es` (after consonant), `-ns`, and `-s` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ca", "gats").unwrap(), "gat");
/// assert_eq!(singularize("ca", "ciutats").unwrap(), "ciutat");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("es")
        && (stem.ends_with('r')
            || stem.ends_with('n')
            || stem.ends_with('l')
            || stem.ends_with('s')
            || stem.ends_with('z'))
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ns")
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

/// Returns a list of possible plural forms for a Catalan noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ca", "gat").unwrap();
/// assert!(result.iter().any(|v| v == "gats"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = vec![format!("{name}s").into()];
    if name.ends_with('r')
        || name.ends_with('n')
        || name.ends_with('l')
        || name.ends_with('s')
        || name.ends_with('z')
    {
        candidates.push(format!("{name}es").into());
    }
    candidates.push(format!("{name}ns").into());
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_es_suffix() {
        assert_eq!(singularize("flores"), "flor");
        assert_eq!(singularize("panes"), "pan");
        assert_eq!(singularize("cables"), "cabl");
        assert_eq!(singularize("grosses"), "gross");
        assert_eq!(singularize("pazes"), "paz");
    }

    #[test]
    fn test_singularize_es_suffix_no_consonant() {
        // "-es" where stem doesn't end in r/n/l/s/z falls through to strip '-s'
        assert_eq!(singularize("cubes"), "cube");
    }

    #[test]
    fn test_singularize_ns_suffix() {
        assert_eq!(singularize("homens"), "home");
    }

    #[test]
    fn test_singularize_s_suffix() {
        assert_eq!(singularize("gats"), "gat");
        assert_eq!(singularize("ciutats"), "ciutat");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("gat"), "gat");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "ns" -> strip_suffix("ns") -> "" empty -> skip; strip_suffix('s') -> "n" non-empty -> "n"
        assert_eq!(singularize("ns"), "n");
        assert_eq!(singularize("s"), "s");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("gat");
        assert!(result.iter().any(|v| v == "gats"));
        assert!(result.iter().any(|v| v == "gatns"));

        let result = pluralize("flor");
        assert!(result.iter().any(|v| v == "flores"));
        assert!(result.iter().any(|v| v == "flors"));
    }

    #[test]
    fn test_pluralize_consonant_endings() {
        // 'n' ending
        let result = pluralize("pan");
        assert!(result.iter().any(|v| v == "pans"));
        assert!(result.iter().any(|v| v == "panes"));
        assert!(result.iter().any(|v| v == "panns"));

        // 'l' ending
        let result = pluralize("cabl");
        assert!(result.iter().any(|v| v == "cables"));

        // 's' ending
        let result = pluralize("gros");
        assert!(result.iter().any(|v| v == "groses"));

        // 'z' ending
        let result = pluralize("paz");
        assert!(result.iter().any(|v| v == "pazes"));
    }

    #[test]
    fn test_pluralize_no_consonant_ending() {
        let result = pluralize("casa");
        assert!(result.iter().any(|v| v == "casas"));
        assert!(result.iter().any(|v| v == "casans"));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }
}
