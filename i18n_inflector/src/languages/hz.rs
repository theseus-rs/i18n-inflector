use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("omundu", "ovandu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("hz", "hz", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("hz", &[&super::PROFILE]);
    }
}
