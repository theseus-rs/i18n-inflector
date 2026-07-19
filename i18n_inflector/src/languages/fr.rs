use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[LexicalClassSpec::new(
    "regular-s",
    "regular nouns taking -s without a stem change",
    Rule::Suffix("s"),
)];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("journal", "journaux"),
    VerifiedLexeme::new("amour", "amours"),
    VerifiedLexeme::new("matin", "matins"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("fr", "fr", false, None, CLASSES, (true, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("fr", &[&super::PROFILE]);
    }
}
