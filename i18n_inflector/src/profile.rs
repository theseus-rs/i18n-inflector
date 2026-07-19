use crate::features::{Countability, InflectionRequest, SelectorKind};
use crate::{Error, Result};
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use icu_normalizer::ComposingNormalizerBorrowed;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Rule {
    Suffix(&'static str),
    ReplaceSuffix {
        singular: &'static str,
        plural: &'static str,
    },
    ReplacePrefix {
        singular: &'static str,
        plural: &'static str,
    },
    Reduplicate(&'static str),
    Turkish,
}

impl Rule {
    fn apply(self, lemma: &str) -> Option<String> {
        match self {
            Self::Suffix(suffix) => Some(format!("{lemma}{suffix}")),
            Self::ReplaceSuffix { singular, plural } => lemma
                .strip_suffix(singular)
                .filter(|stem| !stem.is_empty())
                .map(|stem| format!("{stem}{plural}")),
            Self::ReplacePrefix { singular, plural } => lemma
                .strip_prefix(singular)
                .filter(|stem| !stem.is_empty())
                .map(|stem| format!("{plural}{stem}")),
            Self::Reduplicate(separator) => Some(format!("{lemma}{separator}{lemma}")),
            Self::Turkish => turkish_plural(lemma),
        }
    }
}

fn turkish_plural(lemma: &str) -> Option<String> {
    let suffix = lemma.chars().rev().find_map(|character| match character {
        'a' | 'ı' | 'o' | 'u' | 'A' | 'I' | 'O' | 'U' => Some("lar"),
        'e' | 'i' | 'ö' | 'ü' | 'E' | 'İ' | 'Ö' | 'Ü' => Some("ler"),
        _ => None,
    })?;
    Some(format!("{lemma}{suffix}"))
}

/// A documented productive inflection class supported by a language profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LexicalClassSpec {
    id: &'static str,
    description: &'static str,
    pub(crate) rule: Rule,
}

impl LexicalClassSpec {
    pub(crate) const fn new(id: &'static str, description: &'static str, rule: Rule) -> Self {
        Self {
            id,
            description,
            rule,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.id
    }

    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }
}

/// Selectors and productive classes accepted by a language profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LanguageCapabilities {
    classes: &'static [LexicalClassSpec],
    gender: bool,
    animacy: bool,
}

impl LanguageCapabilities {
    pub(crate) const fn new(
        classes: &'static [LexicalClassSpec],
        gender: bool,
        animacy: bool,
    ) -> Self {
        Self {
            classes,
            gender,
            animacy,
        }
    }

    #[must_use]
    pub const fn lexical_classes(&self) -> &'static [LexicalClassSpec] {
        self.classes
    }

    #[must_use]
    pub const fn supports_gender(&self) -> bool {
        self.gender
    }

    #[must_use]
    pub const fn supports_animacy(&self) -> bool {
        self.animacy
    }

    #[must_use]
    pub const fn supports_countability(&self) -> bool {
        true
    }
}

/// One or more verified forms for a single inflection request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InflectedForms<'a> {
    primary: Cow<'a, str>,
    alternatives: Vec<Cow<'a, str>>,
}

#[derive(Debug)]
pub(crate) struct VerifiedLexeme {
    lemma: &'static str,
    plural: Option<&'static str>,
    alternatives: &'static [&'static str],
}

impl VerifiedLexeme {
    pub(crate) const fn new(lemma: &'static str, plural: &'static str) -> Self {
        Self {
            lemma,
            plural: Some(plural),
            alternatives: &[],
        }
    }

    pub(crate) const fn with_alternatives(
        lemma: &'static str,
        plural: &'static str,
        alternatives: &'static [&'static str],
    ) -> Self {
        Self {
            lemma,
            plural: Some(plural),
            alternatives,
        }
    }

    pub(crate) const fn without_plural(lemma: &'static str) -> Self {
        Self {
            lemma,
            plural: None,
            alternatives: &[],
        }
    }

    pub(crate) const fn lemma(&self) -> &'static str {
        self.lemma
    }

    pub(crate) const fn plural(&self) -> Option<&'static str> {
        self.plural
    }

    pub(crate) const fn alternatives(&self) -> &'static [&'static str] {
        self.alternatives
    }

    pub(crate) fn forms<'a>(&self) -> Option<InflectedForms<'a>> {
        self.plural
            .map(|plural| InflectedForms::verified(plural, self.alternatives))
    }
}

impl<'a> InflectedForms<'a> {
    fn single(primary: Cow<'a, str>) -> Self {
        Self {
            primary,
            alternatives: Vec::new(),
        }
    }

    pub(crate) fn verified(primary: &'static str, alternatives: &'static [&'static str]) -> Self {
        let mut unique = Vec::new();
        for alternative in alternatives {
            if *alternative != primary && !unique.iter().any(|value| value == alternative) {
                unique.push(Cow::Borrowed(*alternative));
            }
        }
        Self {
            primary: Cow::Borrowed(primary),
            alternatives: unique,
        }
    }

    #[must_use]
    pub fn primary(&self) -> &str {
        self.primary.as_ref()
    }

    #[must_use]
    pub fn alternatives(&self) -> &[Cow<'a, str>] {
        &self.alternatives
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        core::iter::once(self.primary.as_ref()).chain(self.alternatives.iter().map(AsRef::as_ref))
    }
}

/// A static, locale-specific noun inflection profile.
#[derive(Debug)]
pub struct LanguageProfile {
    pub(crate) language: &'static str,
    pub(crate) locale: &'static str,
    pub(crate) invariant: bool,
    pub(crate) default_rule: Option<Rule>,
    pub(crate) capabilities: LanguageCapabilities,
    lexemes: &'static [VerifiedLexeme],
}

impl LanguageProfile {
    pub(crate) const fn new(
        language: &'static str,
        locale: &'static str,
        invariant: bool,
        default_rule: Option<Rule>,
        classes: &'static [LexicalClassSpec],
        selectors: (bool, bool),
        lexemes: &'static [VerifiedLexeme],
    ) -> Self {
        Self {
            language,
            locale,
            invariant,
            default_rule,
            capabilities: LanguageCapabilities::new(classes, selectors.0, selectors.1),
            lexemes,
        }
    }

    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    #[must_use]
    pub const fn locale(&self) -> &'static str {
        self.locale
    }

    #[must_use]
    pub const fn capabilities(&self) -> &LanguageCapabilities {
        &self.capabilities
    }

    pub(crate) const fn lexemes(&self) -> &'static [VerifiedLexeme] {
        self.lexemes
    }

    /// Generates a verified dictionary-form noun inflection.
    ///
    /// # Errors
    ///
    /// Returns a typed [`Error`] when the request is invalid, under-specified, or not covered by
    /// a curated embedded entry or documented productive class.
    pub fn inflect<'a>(&self, request: InflectionRequest<'a>) -> Result<InflectedForms<'a>> {
        if request.lemma.is_empty() {
            return Err(Error::EmptyLemma);
        }
        self.validate_selectors(request)?;

        let lemma = normalize(request.lemma);
        if request.number == crate::Number::Singular {
            return Ok(InflectedForms::single(lemma));
        }
        if request.countability == Some(Countability::Mass) {
            return Err(Error::NoForm {
                locale: self.locale,
                lemma: lemma.into_owned(),
                number: request.number,
            });
        }

        let entry = self
            .lexemes
            .iter()
            .find(|entry| entry.lemma == lemma.as_ref());

        if let Some(class) = request.lexical_class
            && let Some(rule) = self
                .capabilities
                .classes
                .iter()
                .find(|spec| spec.id == class.as_str())
                .map(|spec| spec.rule)
        {
            let Some(form) = rule.apply(lemma.as_ref()) else {
                return Err(Error::UnknownLemma {
                    locale: self.locale,
                    lemma: lemma.into_owned(),
                });
            };
            if let Some(entry) = entry {
                let Some(primary) = entry.plural() else {
                    return Err(Error::NoForm {
                        locale: self.locale,
                        lemma: lemma.into_owned(),
                        number: request.number,
                    });
                };
                if form != primary && !entry.alternatives().contains(&form.as_str()) {
                    return Err(Error::IncompatibleLexicalClass {
                        locale: self.locale,
                        lemma: lemma.into_owned(),
                        class: class.as_str().to_string(),
                    });
                }
            }
            return Ok(InflectedForms::single(Cow::Owned(form)));
        }

        if let Some(entry) = entry {
            return entry.forms().ok_or_else(|| Error::NoForm {
                locale: self.locale,
                lemma: lemma.into_owned(),
                number: request.number,
            });
        }

        if let Some(rule) = self.default_rule {
            return rule
                .apply(lemma.as_ref())
                .map(|form| InflectedForms::single(Cow::Owned(form)))
                .ok_or_else(|| Error::UnknownLemma {
                    locale: self.locale,
                    lemma: lemma.into_owned(),
                });
        }
        if self.invariant {
            return Ok(InflectedForms::single(lemma));
        }
        if !self.capabilities.classes.is_empty() {
            return Err(Error::MissingSelector {
                locale: self.locale,
                selector: SelectorKind::LexicalClass,
            });
        }
        Err(Error::UnknownLemma {
            locale: self.locale,
            lemma: lemma.into_owned(),
        })
    }

    fn validate_selectors(&self, request: InflectionRequest<'_>) -> Result<()> {
        if request.gender.is_some() && !self.capabilities.gender {
            return Err(Error::UnsupportedSelector {
                locale: self.locale,
                selector: SelectorKind::Gender,
            });
        }
        if request.animacy.is_some() && !self.capabilities.animacy {
            return Err(Error::UnsupportedSelector {
                locale: self.locale,
                selector: SelectorKind::Animacy,
            });
        }
        if let Some(class) = request.lexical_class
            && !self
                .capabilities
                .classes
                .iter()
                .any(|spec| spec.id == class.as_str())
        {
            return Err(Error::UnknownLexicalClass {
                locale: self.locale,
                class: class.as_str().to_string(),
            });
        }
        Ok(())
    }
}

fn normalize(value: &str) -> Cow<'_, str> {
    const NFC: ComposingNormalizerBorrowed<'static> = ComposingNormalizerBorrowed::new_nfc();
    NFC.normalize(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Animacy, Gender, LexicalClassId, Number, language_profile};

    fn primary(profile: &LanguageProfile, request: InflectionRequest<'_>) -> String {
        profile.inflect(request).map_or_else(
            |error| error.to_string(),
            |forms| forms.primary().to_string(),
        )
    }

    #[test]
    fn returns_verified_singular_plural_and_alternatives() {
        let english = language_profile("en").unwrap();
        assert_eq!(
            primary(english, InflectionRequest::singular("child")),
            "child"
        );
        assert_eq!(
            primary(english, InflectionRequest::plural("child")),
            "children"
        );

        let indonesian = language_profile("id").unwrap();
        let forms = indonesian
            .inflect(InflectionRequest::plural("tikus"))
            .unwrap();
        assert_eq!(forms.primary(), "tikus");
        assert_eq!(forms.alternatives(), &[Cow::Borrowed("tikus-tikus")]);
        assert_eq!(forms.iter().collect::<Vec<_>>(), ["tikus", "tikus-tikus"]);

        let deduplicated = InflectedForms::verified("one", &["one", "two", "two", "three"]);
        assert_eq!(
            deduplicated.iter().collect::<Vec<_>>(),
            ["one", "two", "three"]
        );
    }

    #[test]
    fn applies_english_and_italian_classes() {
        let english = language_profile("en").unwrap();
        let regular =
            InflectionRequest::plural("project").lexical_class(LexicalClassId::new("regular-s"));
        let sibilant =
            InflectionRequest::plural("church").lexical_class(LexicalClassId::new("sibilant-es"));
        let consonant_y =
            InflectionRequest::plural("category").lexical_class(LexicalClassId::new("consonant-y"));
        assert_eq!(primary(english, regular), "projects");
        assert_eq!(primary(english, sibilant), "churches");
        assert_eq!(primary(english, consonant_y), "categories");

        let italian = language_profile("it").unwrap();
        let feminine =
            InflectionRequest::plural("casa").lexical_class(LexicalClassId::new("feminine-a"));
        assert_eq!(primary(italian, feminine), "case");
    }

    #[test]
    fn applies_language_specific_suffix_classes() {
        let afrikaans = language_profile("af").unwrap();
        assert_eq!(
            primary(
                afrikaans,
                InflectionRequest::plural("tafel").lexical_class(LexicalClassId::new("suffix-s"))
            ),
            "tafels"
        );

        let dutch = language_profile("nl").unwrap();
        assert_eq!(
            primary(
                dutch,
                InflectionRequest::plural("boek").lexical_class(LexicalClassId::new("suffix-en"))
            ),
            "boeken"
        );

        let spanish = language_profile("es").unwrap();
        assert_eq!(
            primary(
                spanish,
                InflectionRequest::plural("hotel").lexical_class(LexicalClassId::new("regular-es"))
            ),
            "hoteles"
        );

        let interlingua = language_profile("ia").unwrap();
        assert_eq!(
            primary(
                interlingua,
                InflectionRequest::plural("libro")
                    .lexical_class(LexicalClassId::new("after-vowel-s"))
            ),
            "libros"
        );
        assert_eq!(
            primary(
                interlingua,
                InflectionRequest::plural("artichoc")
                    .lexical_class(LexicalClassId::new("final-c-ches"))
            ),
            "artichoches"
        );

        let occidental = language_profile("ie").unwrap();
        assert_eq!(
            primary(
                occidental,
                InflectionRequest::plural("parol")
                    .lexical_class(LexicalClassId::new("after-most-consonants-es"))
            ),
            "paroles"
        );
        assert_eq!(
            primary(
                occidental,
                InflectionRequest::plural("tric")
                    .lexical_class(LexicalClassId::new("final-c-g-m-s"))
            ),
            "trics"
        );
    }

    #[test]
    fn applies_prefix_reduplication_and_default_rules() {
        let swahili = language_profile("sw").unwrap();
        let human = InflectionRequest::plural("mtoto").lexical_class(LexicalClassId::new("m-wa"));
        let object =
            InflectionRequest::plural("kitabu").lexical_class(LexicalClassId::new("ki-vi"));
        assert_eq!(primary(swahili, human), "watoto");
        assert_eq!(primary(swahili, object), "vitabu");

        let zulu = language_profile("zu").unwrap();
        let human =
            InflectionRequest::plural("umuntu").lexical_class(LexicalClassId::new("umu-aba"));
        let object =
            InflectionRequest::plural("isikole").lexical_class(LexicalClassId::new("isi-izi"));
        assert_eq!(primary(zulu, human), "abantu");
        assert_eq!(primary(zulu, object), "izikole");

        let indonesian = language_profile("id").unwrap();
        let reduplicated = InflectionRequest::plural("buku")
            .lexical_class(LexicalClassId::new("explicit-reduplication"));
        assert_eq!(primary(indonesian, reduplicated), "buku-buku");
        let verified_reduplicated = InflectionRequest::plural("tikus")
            .lexical_class(LexicalClassId::new("explicit-reduplication"));
        assert_eq!(primary(indonesian, verified_reduplicated), "tikus-tikus");

        assert_eq!(
            primary(
                language_profile("eo").unwrap(),
                InflectionRequest::plural("hundo")
            ),
            "hundoj"
        );
        assert_eq!(
            primary(
                language_profile("uz").unwrap(),
                InflectionRequest::plural("daraxt")
            ),
            "daraxtlar"
        );
        assert_eq!(
            primary(
                language_profile("tr").unwrap(),
                InflectionRequest::plural("araba")
            ),
            "arabalar"
        );
        assert_eq!(
            primary(
                language_profile("tr").unwrap(),
                InflectionRequest::plural("şehir")
            ),
            "şehirler"
        );
    }

    #[test]
    fn validates_requests_without_guessing() {
        let english = language_profile("en").unwrap();
        assert_eq!(
            english.inflect(InflectionRequest::plural("")),
            Err(Error::EmptyLemma)
        );
        assert_eq!(
            english.inflect(InflectionRequest::plural("furniture")),
            Err(Error::NoForm {
                locale: "en",
                lemma: "furniture".into(),
                number: Number::Plural,
            })
        );
        assert_eq!(
            english.inflect(InflectionRequest::plural("water").countability(Countability::Mass)),
            Err(Error::NoForm {
                locale: "en",
                lemma: "water".into(),
                number: Number::Plural,
            })
        );
        assert_eq!(
            english.inflect(InflectionRequest::plural("project")),
            Err(Error::MissingSelector {
                locale: "en",
                selector: SelectorKind::LexicalClass,
            })
        );
        assert_eq!(
            english.inflect(
                InflectionRequest::plural("project")
                    .lexical_class(LexicalClassId::new("not-a-class"))
            ),
            Err(Error::UnknownLexicalClass {
                locale: "en",
                class: "not-a-class".into(),
            })
        );
    }

    #[test]
    fn rejects_incompatible_or_unsupported_selectors() {
        let english = language_profile("en").unwrap();
        assert_eq!(
            english.inflect(
                InflectionRequest::plural("child").lexical_class(LexicalClassId::new("regular-s"))
            ),
            Err(Error::IncompatibleLexicalClass {
                locale: "en",
                lemma: "child".into(),
                class: "regular-s".into(),
            })
        );
        assert_eq!(
            english.inflect(
                InflectionRequest::plural("furniture")
                    .lexical_class(LexicalClassId::new("regular-s"))
            ),
            Err(Error::NoForm {
                locale: "en",
                lemma: "furniture".into(),
                number: Number::Plural,
            })
        );
        assert_eq!(
            english.inflect(InflectionRequest::plural("dog").gender(Gender::Common)),
            Err(Error::UnsupportedSelector {
                locale: "en",
                selector: SelectorKind::Gender,
            })
        );
        assert_eq!(
            english.inflect(InflectionRequest::plural("dog").animacy(Animacy::Animate)),
            Err(Error::UnsupportedSelector {
                locale: "en",
                selector: SelectorKind::Animacy,
            })
        );
        assert_eq!(
            english.inflect(
                InflectionRequest::plural("dog").lexical_class(LexicalClassId::new("consonant-y"))
            ),
            Err(Error::UnknownLemma {
                locale: "en",
                lemma: "dog".into(),
            })
        );
        assert_eq!(
            language_profile("ar")
                .unwrap()
                .inflect(InflectionRequest::plural("رِسَالَة").gender(Gender::Masculine)),
            Err(Error::UnsupportedSelector {
                locale: "ar",
                selector: SelectorKind::Gender,
            })
        );
        assert_eq!(
            language_profile("sw")
                .unwrap()
                .inflect(InflectionRequest::plural("mtu").animacy(Animacy::Human)),
            Err(Error::UnsupportedSelector {
                locale: "sw",
                selector: SelectorKind::Animacy,
            })
        );
    }

    #[test]
    fn requires_an_explicit_class_for_unknown_turkish_lemmas() {
        assert_eq!(
            language_profile("tr").unwrap().inflect(
                InflectionRequest::plural("123")
                    .lexical_class(LexicalClassId::new("vowel-harmony"))
            ),
            Err(Error::UnknownLemma {
                locale: "tr",
                lemma: "123".into(),
            })
        );
        assert_eq!(
            language_profile("tr")
                .unwrap()
                .inflect(InflectionRequest::plural("unknown")),
            Err(Error::MissingSelector {
                locale: "tr",
                selector: SelectorKind::LexicalClass,
            })
        );
        assert_eq!(
            primary(
                language_profile("tr").unwrap(),
                InflectionRequest::plural("saat")
            ),
            "saatler"
        );
    }

    #[test]
    fn normalizes_input_and_exposes_profile_metadata() {
        let french = language_profile("fr").unwrap();
        assert_eq!(
            primary(french, InflectionRequest::singular("e\u{301}cole")),
            "école"
        );
        assert_eq!(french.language(), "fr");
        assert_eq!(french.locale(), "fr");
        assert!(!french.capabilities().supports_gender());
        assert!(!french.capabilities().supports_animacy());
        assert!(french.capabilities().supports_countability());
        let classes = french.capabilities().lexical_classes();
        assert_eq!(classes.first().map(LexicalClassSpec::id), Some("regular-s"));
        assert_eq!(
            classes.first().map(LexicalClassSpec::description),
            Some("regular nouns taking -s without a stem change")
        );

        let japanese = language_profile("ja").unwrap();
        assert_eq!(primary(japanese, InflectionRequest::plural("猫")), "猫");
        assert!(!japanese.capabilities().supports_gender());

        assert_eq!(
            primary(japanese, InflectionRequest::plural("")),
            "lemma must not be empty"
        );
    }

    #[test]
    fn constructs_profile_metadata_at_runtime() {
        let class = LexicalClassSpec::new(
            core::hint::black_box("runtime"),
            "runtime class",
            Rule::Suffix("s"),
        );
        let classes = alloc::vec![class];
        let capabilities =
            LanguageCapabilities::new(core::hint::black_box(classes.leak()), true, true);
        assert_eq!(
            capabilities
                .lexical_classes()
                .first()
                .map(LexicalClassSpec::id),
            Some("runtime")
        );

        let profile = LanguageProfile::new(
            core::hint::black_box("xy"),
            "xy",
            false,
            Some(Rule::Suffix("s")),
            capabilities.lexical_classes(),
            (
                capabilities.supports_gender(),
                capabilities.supports_animacy(),
            ),
            &[],
        );
        assert_eq!(
            primary(&profile, InflectionRequest::plural("noun")),
            "nouns"
        );
    }

    #[test]
    fn constructs_verified_lexemes_at_runtime() {
        let lemma = core::hint::black_box("one");
        let regular = VerifiedLexeme::new(lemma, "many");
        assert_eq!(regular.lemma(), "one");
        assert_eq!(regular.plural(), Some("many"));
        assert_eq!(regular.forms().unwrap().primary(), "many");

        let alternatives = VerifiedLexeme::with_alternatives(lemma, "many", &["many", "several"]);
        assert_eq!(alternatives.alternatives(), &["many", "several"]);
        assert_eq!(alternatives.forms().unwrap().alternatives(), &["several"]);

        let absent = VerifiedLexeme::without_plural(lemma);
        assert!(absent.forms().is_none());
    }
}
