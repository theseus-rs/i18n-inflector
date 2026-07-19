use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("kuća", "kuće")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("bs", "bs", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("bs", &[&super::PROFILE]);
    }
}
