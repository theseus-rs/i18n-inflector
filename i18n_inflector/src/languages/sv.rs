use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new(
    "matöverkänslighet",
    "matöverkänsligheter",
)];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sv", "sv", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sv", &[&super::PROFILE]);
    }
}
