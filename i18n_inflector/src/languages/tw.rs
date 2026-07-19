use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ɔhemmaa", "ahemmaa")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tw", "tw", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tw", &[&super::PROFILE]);
    }
}
