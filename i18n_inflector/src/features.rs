use core::fmt;

/// The grammatical number requested for a dictionary-form noun.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Number {
    /// The singular dictionary form.
    Singular,
    /// The plural base or nominative form.
    Plural,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Singular => f.write_str("singular"),
            Self::Plural => f.write_str("plural"),
        }
    }
}

/// A noun's grammatical gender when a language profile uses it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    Common,
}

/// A noun's animacy class when a language profile uses it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Animacy {
    Animate,
    Inanimate,
    Human,
    NonHuman,
}

/// Whether the requested noun sense can normally be counted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Countability {
    Count,
    Mass,
}

/// Identifies a validated, language-specific inflection class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LexicalClassId<'a>(&'a str);

impl<'a> LexicalClassId<'a> {
    #[must_use]
    pub const fn new(id: &'a str) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.0
    }
}

/// The selector named by an inflection error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectorKind {
    Gender,
    Animacy,
    Countability,
    LexicalClass,
}

impl fmt::Display for SelectorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gender => f.write_str("gender"),
            Self::Animacy => f.write_str("animacy"),
            Self::Countability => f.write_str("countability"),
            Self::LexicalClass => f.write_str("lexical class"),
        }
    }
}

/// A dictionary lemma and the target noun features to generate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InflectionRequest<'a> {
    pub(crate) lemma: &'a str,
    pub(crate) number: Number,
    pub(crate) gender: Option<Gender>,
    pub(crate) animacy: Option<Animacy>,
    pub(crate) countability: Option<Countability>,
    pub(crate) lexical_class: Option<LexicalClassId<'a>>,
}

impl<'a> InflectionRequest<'a> {
    #[must_use]
    pub const fn new(lemma: &'a str, number: Number) -> Self {
        Self {
            lemma,
            number,
            gender: None,
            animacy: None,
            countability: None,
            lexical_class: None,
        }
    }

    #[must_use]
    pub const fn singular(lemma: &'a str) -> Self {
        Self::new(lemma, Number::Singular)
    }

    #[must_use]
    pub const fn plural(lemma: &'a str) -> Self {
        Self::new(lemma, Number::Plural)
    }

    #[must_use]
    pub const fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    #[must_use]
    pub const fn animacy(mut self, animacy: Animacy) -> Self {
        self.animacy = Some(animacy);
        self
    }

    #[must_use]
    pub const fn countability(mut self, countability: Countability) -> Self {
        self.countability = Some(countability);
        self
    }

    #[must_use]
    pub const fn lexical_class(mut self, lexical_class: LexicalClassId<'a>) -> Self {
        self.lexical_class = Some(lexical_class);
        self
    }

    #[must_use]
    pub const fn lemma(&self) -> &'a str {
        self.lemma
    }

    #[must_use]
    pub const fn number(&self) -> Number {
        self.number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn request_builders_and_accessors() {
        let request = InflectionRequest::singular("lemma")
            .gender(Gender::Common)
            .animacy(Animacy::Human)
            .countability(Countability::Count)
            .lexical_class(LexicalClassId::new("class"));
        assert_eq!(request.lemma(), "lemma");
        assert_eq!(request.number(), Number::Singular);
        assert_eq!(request.gender, Some(Gender::Common));
        assert_eq!(request.animacy, Some(Animacy::Human));
        assert_eq!(request.countability, Some(Countability::Count));
        assert_eq!(
            request.lexical_class.map(LexicalClassId::as_str),
            Some("class")
        );

        let plural = InflectionRequest::plural("lemmas");
        assert_eq!(plural.number(), Number::Plural);
    }

    #[test]
    fn display_values() {
        assert_eq!(format!("{}", Number::Singular), "singular");
        assert_eq!(format!("{}", Number::Plural), "plural");
        assert_eq!(format!("{}", SelectorKind::Gender), "gender");
        assert_eq!(format!("{}", SelectorKind::Animacy), "animacy");
        assert_eq!(format!("{}", SelectorKind::Countability), "countability");
        assert_eq!(format!("{}", SelectorKind::LexicalClass), "lexical class");

        let _ = [Gender::Masculine, Gender::Feminine, Gender::Neuter];
        let _ = [Animacy::Animate, Animacy::Inanimate, Animacy::NonHuman];
        let _ = Countability::Mass;
    }
}
