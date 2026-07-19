use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("paire", "paires")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("oc", "oc", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("oc", &[&super::PROFILE]);
    }
}
