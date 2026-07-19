use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("стаг", "нах")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ce", "ce", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ce", &[&super::PROFILE]);
    }
}
