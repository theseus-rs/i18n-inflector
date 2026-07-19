use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("umuntu", "abantu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("nd", "nd", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("nd", &[&super::PROFILE]);
    }
}
