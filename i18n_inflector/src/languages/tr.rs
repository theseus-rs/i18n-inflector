//! Turkish (tr) inflection rules.
//!
//! Also used for Azerbaijani (az), Bashkir (ba), Chuvash (cv), Kazakh (kk), Kyrgyz (ky),
//! Turkmen (tk), Tatar (tt), Uyghur (ug), and Uzbek (uz).

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use crate::language_rules::LanguageRuleSet;

/// Back vowels in Turkish vowel harmony (including dotless-i).
const BACK_VOWELS: &[char] = &['a', 'ı', 'o', 'u', 'A', 'I', 'O', 'U'];
/// Unambiguous front vowels in Turkish vowel harmony.
const FRONT_VOWELS: &[char] = &['e', 'ö', 'ü', 'E', 'İ', 'Ö', 'Ü'];
/// Ambiguous vowels that could be front or back in ASCII transliteration.
const AMBIGUOUS_VOWELS: &[char] = &['i'];

/// Determines vowel harmony for a Turkic word.
/// Returns `Some(true)` for back, `Some(false)` for front, `None` if ambiguous
/// or no vowels found.
fn last_vowel_harmony(s: &str) -> Option<bool> {
    for ch in s.chars().rev() {
        if BACK_VOWELS.contains(&ch) {
            return Some(true);
        }
        if FRONT_VOWELS.contains(&ch) {
            return Some(false);
        }
        if AMBIGUOUS_VOWELS.contains(&ch) {
            return None;
        }
    }
    None
}

pub(crate) static RULES: LanguageRuleSet = LanguageRuleSet {
    language: "tr",
    singularize_fn: singularize,
    pluralize_fn: pluralize,
};

/// Converts a plural Turkish noun to its singular form.
///
/// Turkish plurals use `-lar` (back vowel harmony) or `-ler` (front vowel
/// harmony).
pub(crate) fn singularize(name: &str) -> Cow<'_, str> {
    for suffix in &["lar", "ler"] {
        if let Some(stem) = name.strip_suffix(suffix)
            && !stem.is_empty()
        {
            return Cow::Borrowed(stem);
        }
    }
    Cow::Borrowed(name)
}

/// Returns the grammatically correct plural form for a Turkish noun based on
/// vowel harmony.
///
/// Analyzes the stem's last vowel to choose between `-lar` (back) and `-ler` (front).
pub(crate) fn pluralize(name: &str) -> Vec<Cow<'_, str>> {
    match last_vowel_harmony(name) {
        Some(true) => vec![format!("{name}lar").into()],
        Some(false) => vec![format!("{name}ler").into()],
        None => vec![format!("{name}lar").into(), format!("{name}ler").into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularize_lar() {
        assert_eq!(singularize("kullanicilar"), "kullanici");
        assert_eq!(singularize("arabalar"), "araba");
        assert_eq!(singularize("kitaplar"), "kitap");
    }

    #[test]
    fn test_singularize_ler() {
        assert_eq!(singularize("urunler"), "urun");
        assert_eq!(singularize("evler"), "ev");
        assert_eq!(singularize("siparisler"), "siparis");
    }

    #[test]
    fn test_singularize_already_singular() {
        assert_eq!(singularize("kullanici"), "kullanici");
        assert_eq!(singularize("ev"), "ev");
    }

    #[test]
    fn test_singularize_suffix_only() {
        assert_eq!(singularize("lar"), "lar");
        assert_eq!(singularize("ler"), "ler");
    }

    #[test]
    fn test_singularize_empty() {
        assert_eq!(singularize(""), "");
    }

    #[test]
    fn test_pluralize_back_vowel() {
        let result = pluralize("araba");
        assert_eq!(result, vec!["arabalar"]);

        let result = pluralize("kitap");
        assert_eq!(result, vec!["kitaplar"]);
    }

    #[test]
    fn test_pluralize_front_vowel() {
        let result = pluralize("ev");
        assert_eq!(result, vec!["evler"]);

        let result = pluralize("göz");
        assert_eq!(result, vec!["gözler"]);
    }

    #[test]
    fn test_pluralize_ambiguous_vowel() {
        let result = pluralize("kullanici");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "kullanicilar"));
        assert!(result.iter().any(|v| v == "kullaniciler"));
    }

    #[test]
    fn test_pluralize_no_vowel() {
        let result = pluralize("xyz");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|v| v == "xyzlar"));
        assert!(result.iter().any(|v| v == "xyzler"));
    }

    #[test]
    fn test_pluralize_empty() {
        let result = pluralize("");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_last_vowel_harmony_back() {
        assert_eq!(last_vowel_harmony("araba"), Some(true));
        assert_eq!(last_vowel_harmony("kitap"), Some(true));
        assert_eq!(last_vowel_harmony("kapı"), Some(true));
    }

    #[test]
    fn test_last_vowel_harmony_front() {
        assert_eq!(last_vowel_harmony("ev"), Some(false));
        assert_eq!(last_vowel_harmony("göz"), Some(false));
    }

    #[test]
    fn test_last_vowel_harmony_ambiguous() {
        assert_eq!(last_vowel_harmony("kullanici"), None);
    }

    #[test]
    fn test_last_vowel_harmony_none() {
        assert_eq!(last_vowel_harmony("xyz"), None);
        assert_eq!(last_vowel_harmony(""), None);
    }

    #[test]
    fn test_rules_language() {
        use crate::language_rules::LanguageRules;
        assert_eq!(RULES.language(), "tr");
    }
}
