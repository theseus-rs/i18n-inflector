//! Latin (la) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

/// Converts a plural Latin noun to its singular form.
///
/// Handles `-ae` -> `-a`, `-i` -> `-us`, `-es` -> `-is`, and `-a` -> `-um` transformations.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("la", "domini").unwrap(), "dominus");
/// assert_eq!(singularize("la", "rosae").unwrap(), "rosa");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("ae")
        && !stem.is_empty()
    {
        return format!("{stem}a").into();
    }
    if let Some(stem) = name.strip_suffix('i')
        && !stem.is_empty()
    {
        return format!("{stem}us").into();
    }
    if let Some(stem) = name.strip_suffix("es")
        && !stem.is_empty()
    {
        return format!("{stem}is").into();
    }
    if let Some(stem) = name.strip_suffix('a')
        && !stem.is_empty()
    {
        return format!("{stem}um").into();
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Latin noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("la", "dominus").unwrap();
/// assert!(result.iter().any(|v| v == "domini"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    let mut candidates = Vec::new();
    if let Some(stem) = name.strip_suffix("us") {
        candidates.push(format!("{stem}i").into());
    }
    if let Some(stem) = name.strip_suffix('a') {
        candidates.push(format!("{stem}ae").into());
    }
    if let Some(stem) = name.strip_suffix("um") {
        candidates.push(format!("{stem}a").into());
    }
    if let Some(stem) = name.strip_suffix("is") {
        candidates.push(format!("{stem}es").into());
    }
    if candidates.is_empty() {
        candidates.push(format!("{name}es").into());
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_ae_suffix() {
        assert_eq!(singularize("rosae"), "rosa");
    }

    #[test]
    fn test_singularize_i_suffix() {
        assert_eq!(singularize("domini"), "dominus");
    }

    #[test]
    fn test_singularize_es_suffix() {
        assert_eq!(singularize("civites"), "civitis");
    }

    #[test]
    fn test_singularize_a_suffix() {
        assert_eq!(singularize("bella"), "bellum");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("dominus"), "dominus");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "ae" -> empty -> skip; 'i' -> no; "es" -> no; 'a' -> no -> "ae"
        assert_eq!(singularize("ae"), "ae");
        assert_eq!(singularize("i"), "i");
        assert_eq!(singularize("es"), "es");
        assert_eq!(singularize("a"), "a");
    }

    #[test]
    fn test_pluralize_us_suffix() {
        let result = pluralize("dominus");
        assert!(result.iter().any(|v| v == "domini"));
    }

    #[test]
    fn test_pluralize_a_suffix() {
        let result = pluralize("rosa");
        assert!(result.iter().any(|v| v == "rosae"));
    }

    #[test]
    fn test_pluralize_um_suffix() {
        let result = pluralize("bellum");
        assert!(result.iter().any(|v| v == "bella"));
    }

    #[test]
    fn test_pluralize_is_suffix() {
        let result = pluralize("civis");
        assert!(result.iter().any(|v| v == "cives"));
    }

    #[test]
    fn test_pluralize_no_known_suffix() {
        let result = pluralize("rex");
        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|v| v == "rexes"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 1);
    }
}
