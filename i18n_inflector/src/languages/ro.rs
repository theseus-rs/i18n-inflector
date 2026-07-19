use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("bărzăun", "bărzăuni")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ro", "ro", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ro", &[&super::PROFILE]);
    }
}
