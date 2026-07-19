use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("hygienička", "hygieničky")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("cs", "cs", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("cs", &[&super::PROFILE]);
    }
}
