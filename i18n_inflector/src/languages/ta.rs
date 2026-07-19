use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("செந்நாய்", "செந்நாய்கள்")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ta", "ta", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ta", &[&super::PROFILE]);
    }
}
