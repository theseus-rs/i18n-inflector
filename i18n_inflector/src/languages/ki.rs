use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("mũndũ", "andũ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ki", "ki", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ki", &[&super::PROFILE]);
    }
}
