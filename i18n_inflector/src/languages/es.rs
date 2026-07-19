use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new("regular-s", "regular nouns taking -s", Rule::Suffix("s")),
    LexicalClassSpec::new("regular-es", "regular nouns taking -es", Rule::Suffix("es")),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("ciudad", "ciudades"),
    VerifiedLexeme::new("casa", "casas"),
    VerifiedLexeme::new("hotel", "hoteles"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("es", "es", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("es", &[&super::PROFILE]);
    }
}
