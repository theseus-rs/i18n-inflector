use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ijinerezh", "ijinerezhioù")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("br", "br", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("br", &[&super::PROFILE]);
    }
}
