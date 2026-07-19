use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("gan", "ganlar")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tk", "tk", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tk", &[&super::PROFILE]);
    }
}
