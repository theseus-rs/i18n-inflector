use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("लड़का", "लड़के")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("hi", "hi", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("hi", &[&super::PROFILE]);
    }
}
