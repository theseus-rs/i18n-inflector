//! Japanese (ja) inflection rules.
//!
//! Japanese does not mark plurality on nouns; words are returned unchanged. Also used for
//! Afar (aa), Abkhaz (ab), Avestan (ae), Akan (ak), Avar (av), Bislama (bi), Bambara (bm),
//! Tibetan (bo), Chechen (ce), Chamorro (ch), Church Slavonic (cu), Dzongkha (dz), Ewe (ee),
//! Fula (ff), Fijian (fj), Hiri Motu (ho), Indonesian (id), Igbo (ig), Sichuan Yi (ii),
//! Inupiaq (ik), Inuktitut (iu), Javanese (jv), Georgian (ka), Khmer (km), Korean (ko),
//! Komi (kv), Lao (lo), Malagasy (mg), Māori (mi), Malay (ms), Burmese (my), Navajo (nv),
//! Ojibwe (oj), Pali (pi), Sanskrit (sa), Northern Sami (se), Sango (sg), Samoan (sm),
//! Sundanese (su), Thai (th), Tagalog (tl), Vietnamese (vi), Wolof (wo), and Yoruba (yo).

use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;

/// Returns the word unchanged since the language has no morphological plural.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::singularize;
/// assert_eq!(singularize("ja", "user").unwrap(), "user");
/// assert_eq!(singularize("zh", "user").unwrap(), "user");
/// ```
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    Cow::Borrowed(name)
}

/// Returns the word unchanged since the language has no morphological plural.
///
/// # Examples
///
/// ```
/// # use i18n_inflector::pluralize;
/// let result = pluralize("ja", "user").unwrap();
/// assert_eq!(result, vec!["user"]);
/// ```
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    vec![Cow::Borrowed(name)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize() {
        assert_eq!(singularize("user"), "user");
        assert_eq!(singularize("product"), "product");
        assert_eq!(singularize(""), "");
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize("user"), vec!["user"]);
        assert_eq!(pluralize("product"), vec!["product"]);
        assert_eq!(pluralize(""), vec![""]);
    }
}
