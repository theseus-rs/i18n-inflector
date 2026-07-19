use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "suffix-e",
        "nouns taking -e without a stem change",
        Rule::Suffix("e"),
    ),
    LexicalClassSpec::new(
        "suffix-s",
        "nouns taking -s without a stem change",
        Rule::Suffix("s"),
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("slang", "slange"),
    VerifiedLexeme::new("tafel", "tafels"),
    VerifiedLexeme::new("boom", "bome"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("af", "af", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("af", &[&super::PROFILE]);
    }
}
