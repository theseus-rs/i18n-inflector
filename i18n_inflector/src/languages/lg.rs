use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("omuntu", "abantu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("lg", "lg", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("lg", &[&super::PROFILE]);
    }
}
