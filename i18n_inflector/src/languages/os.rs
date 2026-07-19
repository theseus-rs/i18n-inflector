use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("лæппу", "лæппутæ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("os", "os", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("os", &[&super::PROFILE]);
    }
}
