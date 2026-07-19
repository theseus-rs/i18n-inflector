use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("eglė", "eglės")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("lt", "lt", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("lt", &[&super::PROFILE]);
    }
}
