use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new("regular-s", "regular nouns taking -s", Rule::Suffix("s")),
    LexicalClassSpec::new("regular-es", "regular nouns taking -es", Rule::Suffix("es")),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("percebe", "percebes"),
    VerifiedLexeme::new("grelo", "grelos"),
    VerifiedLexeme::new("vieira", "vieiras"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("gl", "gl", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("gl", &[&super::PROFILE]);
    }
}
