use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("аяҡ", "аяҡтар")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ba", "ba", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ba", &[&super::PROFILE]);
    }
}
