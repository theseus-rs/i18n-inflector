//! Locale normalization utilities.
//!
//! Handles common locale variations like `en-US` -> `en`, `EN` -> `en`.

/// Normalizes a locale string to a two-letter ISO 639-1 language code.
///
/// Strips region/country suffixes (e.g., `en-US` -> `en`, `pt-BR` -> `pt`), trims whitespace,
/// and converts to lowercase. Returns a stack-allocated [`NormalizedLocale`] to avoid heap
/// allocation.
/// Maximum length of an ISO 639-1 language code.
const MAX_LANGUAGE_CODE_LENGTH: usize = 2;

/// A stack-allocated normalized locale string (avoids heap allocation).
pub(crate) struct NormalizedLocale {
    buffer: [u8; MAX_LANGUAGE_CODE_LENGTH],
    len: u8,
}

impl NormalizedLocale {
    #[inline]
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        // SAFETY: The internal buffer is guaranteed to be valid UTF-8 by the constructors of
        // NormalizedLocale.
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.len as usize]) }
    }
}

#[inline]
#[must_use]
pub(crate) fn normalize_locale(locale: &str) -> NormalizedLocale {
    let bytes = locale.as_bytes();
    let mut buffer = [0u8; MAX_LANGUAGE_CODE_LENGTH];
    let mut len: u8 = 0;

    for i in 0..bytes.len().min(MAX_LANGUAGE_CODE_LENGTH) {
        let b = bytes[i];
        if b == b'-' || b == b'_' {
            break;
        }
        buffer[i] = b.to_ascii_lowercase();
        len += 1;
    }

    NormalizedLocale { buffer, len }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_simple() {
        assert_eq!(normalize_locale("en").as_str(), "en");
        assert_eq!(normalize_locale("fr").as_str(), "fr");
    }

    #[test]
    fn test_normalize_with_region() {
        assert_eq!(normalize_locale("a_b").as_str(), "a");
        assert_eq!(normalize_locale("a-b").as_str(), "a");
        assert_eq!(normalize_locale("en_US").as_str(), "en");
        assert_eq!(normalize_locale("en-US").as_str(), "en");
        assert_eq!(normalize_locale("pt-BR").as_str(), "pt");
        assert_eq!(normalize_locale("zh-TW").as_str(), "zh");
    }

    #[test]
    fn test_normalize_with_underscore() {
        assert_eq!(normalize_locale("en_US").as_str(), "en");
        assert_eq!(normalize_locale("pt_BR").as_str(), "pt");
    }

    #[test]
    fn test_normalize_uppercase() {
        assert_eq!(normalize_locale("EN").as_str(), "en");
        assert_eq!(normalize_locale("EN-US").as_str(), "en");
    }

    #[test]
    fn test_normalize_unicode() {
        assert_eq!(normalize_locale("ük").as_str(), "ü");
        assert_eq!(normalize_locale("EN-ÜS").as_str(), "en");
    }

    #[test]
    fn test_normalize_empty() {
        assert_eq!(normalize_locale("").as_str(), "");
    }
}
