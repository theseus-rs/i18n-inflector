use crate::{Number, SelectorKind};
use alloc::string::String;
use core::fmt;

/// Result type for inflector operations.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// A typed failure produced while selecting a profile or generating a form.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidLocale {
        locale: String,
    },
    UnsupportedLocale {
        locale: String,
    },
    EmptyLemma,
    UnsupportedSelector {
        locale: &'static str,
        selector: SelectorKind,
    },
    MissingSelector {
        locale: &'static str,
        selector: SelectorKind,
    },
    UnknownLexicalClass {
        locale: &'static str,
        class: String,
    },
    IncompatibleLexicalClass {
        locale: &'static str,
        lemma: String,
        class: String,
    },
    UnknownLemma {
        locale: &'static str,
        lemma: String,
    },
    NoForm {
        locale: &'static str,
        lemma: String,
        number: Number,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLocale { locale } => write!(f, "invalid BCP 47 locale: {locale}"),
            Self::UnsupportedLocale { locale } => write!(f, "unsupported locale: {locale}"),
            Self::EmptyLemma => f.write_str("lemma must not be empty"),
            Self::UnsupportedSelector { locale, selector } => {
                write!(f, "{locale} does not support the {selector} selector")
            }
            Self::MissingSelector { locale, selector } => {
                write!(f, "{locale} requires a {selector} selector for this lemma")
            }
            Self::UnknownLexicalClass { locale, class } => {
                write!(f, "unknown lexical class for {locale}: {class}")
            }
            Self::IncompatibleLexicalClass {
                locale,
                lemma,
                class,
            } => write!(
                f,
                "lexical class {class} is incompatible with {locale} lemma: {lemma}"
            ),
            Self::UnknownLemma { locale, lemma } => {
                write!(f, "no verified inflection for {locale} lemma: {lemma}")
            }
            Self::NoForm {
                locale,
                lemma,
                number,
            } => write!(f, "{locale} lemma {lemma} has no {number} form"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn all_errors_have_stable_messages() {
        let cases = [
            (
                Error::InvalidLocale { locale: "_".into() },
                "invalid BCP 47 locale: _",
            ),
            (
                Error::UnsupportedLocale {
                    locale: "xx".into(),
                },
                "unsupported locale: xx",
            ),
            (Error::EmptyLemma, "lemma must not be empty"),
            (
                Error::UnsupportedSelector {
                    locale: "en",
                    selector: SelectorKind::Gender,
                },
                "en does not support the gender selector",
            ),
            (
                Error::MissingSelector {
                    locale: "en",
                    selector: SelectorKind::LexicalClass,
                },
                "en requires a lexical class selector for this lemma",
            ),
            (
                Error::UnknownLexicalClass {
                    locale: "en",
                    class: "bad".into(),
                },
                "unknown lexical class for en: bad",
            ),
            (
                Error::IncompatibleLexicalClass {
                    locale: "en",
                    lemma: "child".into(),
                    class: "regular-s".into(),
                },
                "lexical class regular-s is incompatible with en lemma: child",
            ),
            (
                Error::UnknownLemma {
                    locale: "de",
                    lemma: "Wort".into(),
                },
                "no verified inflection for de lemma: Wort",
            ),
            (
                Error::NoForm {
                    locale: "en",
                    lemma: "furniture".into(),
                    number: Number::Plural,
                },
                "en lemma furniture has no plural form",
            ),
        ];
        for (error, expected) in cases {
            assert_eq!(format!("{error}"), expected);
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_implements_std_error() {
        let error = Error::EmptyLemma;
        assert!(std::error::Error::source(&error).is_none());
    }
}
