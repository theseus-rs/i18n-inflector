use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("gropë", "gropa")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sq", "sq", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sq", &[&super::PROFILE]);
    }
}
