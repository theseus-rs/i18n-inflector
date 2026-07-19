use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ji", "jiw")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("bm", "bm", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("bm", &[&super::PROFILE]);
    }
}
