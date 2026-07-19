use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("umntu", "abantu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("xh", "xh", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("xh", &[&super::PROFILE]);
    }
}
