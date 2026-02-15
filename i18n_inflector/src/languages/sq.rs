//! Albanian (sq) inflection rules.

use crate::language_rules::LanguageRuleSet;
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "sq",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Albanian noun to its singular form.
///
/// Handles `-e` and `-a` plural suffixes.
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    if let Some(stem) = name.strip_suffix('e')
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

/// Returns a list of possible plural forms for an Albanian noun.
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![format!("{name}e").into(), format!("{name}a").into()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("perdoruese"), "perdorues");
        assert_eq!(singularize("libra"), "libr");
        assert_eq!(singularize("shtete"), "shtet");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("perdorues"), "perdorues");
    }

    #[test]
    fn test_pluralize() {
        let result = pluralize("perdorues");
        assert!(result.iter().any(|v| v == "perdoruese"));
        assert!(result.iter().any(|v| v == "perdoruesa"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(singularize(""), "");
    }
}
