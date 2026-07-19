use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "umu-aba",
        "class 1/2 human nouns",
        Rule::ReplacePrefix {
            singular: "umu",
            plural: "aba",
        },
    ),
    LexicalClassSpec::new(
        "isi-izi",
        "class 7/8 nouns",
        Rule::ReplacePrefix {
            singular: "isi",
            plural: "izi",
        },
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("impi", "izimpi"),
    VerifiedLexeme::new("umuntu", "abantu"),
    VerifiedLexeme::new("isikole", "izikole"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("zu", "zu", false, None, CLASSES, (false, true), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("zu", &[&super::PROFILE]);
    }
}
