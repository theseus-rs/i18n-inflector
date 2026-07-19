use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("taga", "tagawa")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kr", "kr", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kr", &[&super::PROFILE]);
    }
}
