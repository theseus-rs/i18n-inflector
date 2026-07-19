use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("аҽы", "аҽқәа")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ab", "ab", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ab", &[&super::PROFILE]);
    }
}
