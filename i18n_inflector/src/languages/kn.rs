use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ಬೆಕ್ಕು", "ಬೆಕ್ಕುಗಳು")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kn", "kn", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kn", &[&super::PROFILE]);
    }
}
