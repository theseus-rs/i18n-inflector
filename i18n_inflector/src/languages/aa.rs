use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("lee", "lelwa")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("aa", "aa", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("aa", &[&super::PROFILE]);
    }
}
