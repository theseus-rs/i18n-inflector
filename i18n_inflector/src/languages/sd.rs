use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("آئِي", "آئِيُون")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sd", "sd-Arab", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sd", &[&super::PROFILE]);
    }
}
