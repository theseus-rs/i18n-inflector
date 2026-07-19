use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("uarán", "uaráin")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ga", "ga", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ga", &[&super::PROFILE]);
    }
}
