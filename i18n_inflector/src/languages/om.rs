use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("daa'ima", "daa'imman")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("om", "om", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("om", &[&super::PROFILE]);
    }
}
