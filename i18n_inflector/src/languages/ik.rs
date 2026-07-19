use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("iñuk", "iñuit")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ik", "ik", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ik", &[&super::PROFILE]);
    }
}
