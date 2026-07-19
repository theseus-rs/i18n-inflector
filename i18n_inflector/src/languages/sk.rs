use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ruka", "ruky")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sk", "sk", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sk", &[&super::PROFILE]);
    }
}
