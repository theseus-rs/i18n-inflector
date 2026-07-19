use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "suffix-en",
        "nouns taking -en without a stem change",
        Rule::Suffix("en"),
    ),
    LexicalClassSpec::new(
        "suffix-s",
        "nouns taking -s without a stem change",
        Rule::Suffix("s"),
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("kat", "katten"),
    VerifiedLexeme::new("boek", "boeken"),
    VerifiedLexeme::new("mening", "meningen"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("nl", "nl", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("nl", &[&super::PROFILE]);
    }
}
