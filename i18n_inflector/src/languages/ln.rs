use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("mobali", "mibali")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ln", "ln", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ln", &[&super::PROFILE]);
    }
}
