use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("kosam", "kose")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ff", "ff", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ff", &[&super::PROFILE]);
    }
}
