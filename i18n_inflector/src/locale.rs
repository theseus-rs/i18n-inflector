use crate::registry::{base_profile, script_profile};
use crate::{Error, LanguageProfile, Result};
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::ToString;
use icu_locale::LocaleExpander;
use icu_locale_core::LanguageIdentifier;

const LOCALE_EXPANDER: LocaleExpander = LocaleExpander::new_extended();

pub(crate) fn resolve(locale: &str) -> Result<&'static LanguageProfile> {
    let mut parsed = locale
        .parse::<LanguageIdentifier>()
        .map_err(|_| Error::InvalidLocale {
            locale: locale.to_string(),
        })?;

    if !parsed.variants.is_empty() {
        return Err(Error::UnsupportedLocale {
            locale: locale.to_string(),
        });
    }

    let language = parsed.language;
    if let Some(script) = parsed.script {
        let profile = format!("{}-{}", language.as_str(), script.as_str());
        return script_profile(profile.as_str()).ok_or_else(|| Error::UnsupportedLocale {
            locale: locale.to_string(),
        });
    }

    let base = base_profile(language.as_str()).ok_or_else(|| Error::UnsupportedLocale {
        locale: locale.to_string(),
    })?;

    if parsed.region.is_some() {
        LOCALE_EXPANDER.maximize(&mut parsed);
        let profile = parsed
            .script
            .map_or(Cow::Borrowed(base.locale()), |script| {
                Cow::Owned(format!("{}-{}", language.as_str(), script.as_str()))
            });
        if let Some(profile) = script_profile(profile.as_ref()) {
            return Ok(profile);
        }

        if base.locale() != language.as_str() && base.locale() != profile.as_ref() {
            return Err(Error::UnsupportedLocale {
                locale: locale.to_string(),
            });
        }
    }

    Ok(base)
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
        assert_eq!(resolve("zh-TW").unwrap().locale(), "zh-Hant");
        assert_eq!(resolve("zh-CN").unwrap().locale(), "zh-Hans");
        assert_eq!(resolve("pa-PK").unwrap().locale(), "pa-Arab");
        assert_eq!(resolve("pa-IN").unwrap().locale(), "pa-Guru");
        assert_eq!(resolve("uz-AF").unwrap().locale(), "uz-Arab");
    }

    #[test]
    fn rejects_invalid_or_unsupported_locales() {
        assert_eq!(
            resolve("en_US").map(LanguageProfile::locale),
            Err(Error::InvalidLocale {
                locale: "en_US".into(),
            })
        );
        for locale in ["xx", "en-Cyrl", "en-fonipa", "az-IR"] {
            assert_eq!(
                resolve(locale).map(LanguageProfile::locale),
                Err(Error::UnsupportedLocale {
                    locale: locale.into(),
                })
            );
        }
    }
}
