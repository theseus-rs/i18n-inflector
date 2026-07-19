use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ci", "cŵn")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("cy", "cy", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("cy", &[&super::PROFILE]);
    }
}
