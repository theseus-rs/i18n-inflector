use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("иго", "ига")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("cu", "cu", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("cu", &[&super::PROFILE]);
    }
}
