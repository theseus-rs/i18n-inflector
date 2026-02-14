//! Breton (br) inflection rules.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// Converts a plural Breton noun to its singular form.
///
/// Handles `-ioù`, `-où`, and `-ed` plural suffixes.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("br", "bagou").unwrap(), "bag");
/// assert_eq!(singularize("br", "laboured").unwrap(), "labour");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix("iou")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ou")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    if let Some(stem) = name.strip_suffix("ed")
        && !stem.is_empty()
    {
        return Cow::Borrowed(stem);
    }
    Cow::Borrowed(name)
}

/// Returns a list of possible plural forms for a Breton noun.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("br", "bag").unwrap();
/// assert!(result.iter().any(|v| v == "bagou"));
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![
        format!("{name}iou").into(),
        format!("{name}ou").into(),
        format!("{name}ed").into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_iou_suffix() {
        assert_eq!(singularize("levriou"), "levr");
    }

    #[test]
    fn test_singularize_ou_suffix() {
        assert_eq!(singularize("bagou"), "bag");
    }

    #[test]
    fn test_singularize_ed_suffix() {
        assert_eq!(singularize("laboured"), "labour");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("bag"), "bag");
    }

    #[test]
    fn test_singularize_suffix_only_inputs() {
        // "iou" -> strip "iou" empty stem -> skip; strip "ou" -> "i" non-empty -> "i"
        assert_eq!(singularize("iou"), "i");
        assert_eq!(singularize("ou"), "ou");
        assert_eq!(singularize("ed"), "ed");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("bag");
        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|v| v == "bagiou"));
        assert!(result.iter().any(|v| v == "bagou"));
        assert!(result.iter().any(|v| v == "baged"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
        let result = pluralize("");
        assert_eq!(result.len(), 3);
    }
}
