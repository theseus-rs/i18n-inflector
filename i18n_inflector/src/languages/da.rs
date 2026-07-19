use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("bog", "bøger")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("da", "da", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("da", &[&super::PROFILE]);
    }
}
