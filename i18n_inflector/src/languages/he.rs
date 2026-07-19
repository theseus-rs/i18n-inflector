use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ילד", "ילדים")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("he", "he", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("he", &[&super::PROFILE]);
    }
}
