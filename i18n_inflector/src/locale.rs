use crate::registry::{base_profile, script_profile};
use crate::{Error, LanguageProfile, Result};
use alloc::format;
use alloc::string::ToString;
use icu_locale_core::LanguageIdentifier;

pub(crate) fn resolve(locale: &str) -> Result<&'static LanguageProfile> {
    let parsed = locale
        .parse::<LanguageIdentifier>()
        .map_err(|_| Error::InvalidLocale {
            locale: locale.to_string(),
        })?;

    if !parsed.variants.is_empty() {
        return Err(Error::UnsupportedLocale {
            locale: locale.to_string(),
        });
    }

    let language = parsed.language.as_str();
    if let Some(script) = parsed.script {
        let profile = format!("{language}-{}", script.as_str());
        return script_profile(profile.as_str()).ok_or_else(|| Error::UnsupportedLocale {
            locale: locale.to_string(),
        });
    }

    base_profile(language).ok_or_else(|| Error::UnsupportedLocale {
        locale: locale.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_base_region_and_scripts() {
        assert_eq!(resolve("en").unwrap().locale(), "en");
        assert_eq!(resolve("EN-us").unwrap().locale(), "en");
        assert_eq!(resolve("sr-Latn-RS").unwrap().locale(), "sr-Latn");
        assert_eq!(resolve("zh-hant-TW").unwrap().locale(), "zh-Hant");
    }

    #[test]
    fn rejects_invalid_or_unsupported_locales() {
        assert_eq!(
            resolve("en_US").map(LanguageProfile::locale),
            Err(Error::InvalidLocale {
                locale: "en_US".into(),
            })
        );
        for locale in ["xx", "en-Cyrl", "en-fonipa"] {
            assert_eq!(
                resolve(locale).map(LanguageProfile::locale),
                Err(Error::UnsupportedLocale {
                    locale: locale.into(),
                })
            );
        }
    }
}
