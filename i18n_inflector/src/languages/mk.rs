use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("китаб", "китаби")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mk", "mk", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mk", &[&super::PROFILE]);
    }
}
