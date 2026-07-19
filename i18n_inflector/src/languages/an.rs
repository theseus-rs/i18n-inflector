use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new("regular-s", "regular nouns taking -s", Rule::Suffix("s")),
    LexicalClassSpec::new("regular-es", "regular nouns taking -es", Rule::Suffix("es")),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("casa", "casas"),
    VerifiedLexeme::new("casca", "cascas"),
    VerifiedLexeme::new("cascabelera", "cascabeleras"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("an", "an", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("an", &[&super::PROFILE]);
    }
}
