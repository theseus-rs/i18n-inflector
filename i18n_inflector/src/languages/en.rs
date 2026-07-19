use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new("regular-s", "regular nouns taking -s", Rule::Suffix("s")),
    LexicalClassSpec::new(
        "sibilant-es",
        "nouns taking -es after a sibilant",
        Rule::Suffix("es"),
    ),
    LexicalClassSpec::new(
        "consonant-y",
        "nouns changing final consonant+y to -ies",
        Rule::ReplaceSuffix {
            singular: "y",
            plural: "ies",
        },
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::without_plural("furniture"),
    VerifiedLexeme::new("contract", "contracts"),
    VerifiedLexeme::new("child", "children"),
    VerifiedLexeme::new("goose", "geese"),
    VerifiedLexeme::new("man", "men"),
    VerifiedLexeme::new("mouse", "mice"),
    VerifiedLexeme::new("ox", "oxen"),
    VerifiedLexeme::new("person", "people"),
    VerifiedLexeme::new("tooth", "teeth"),
    VerifiedLexeme::new("woman", "women"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("en", "en", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("en", &[&super::PROFILE]);
    }
}
