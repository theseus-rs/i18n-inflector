use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("cane", "canes")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sc", "sc", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sc", &[&super::PROFILE]);
    }
}
