use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("人", "人"),
    VerifiedLexeme::new("家", "家"),
    VerifiedLexeme::new("本", "本"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ja", "ja", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ja", &[&super::PROFILE]);
    }
}
