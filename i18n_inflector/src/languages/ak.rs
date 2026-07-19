use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("onipa", "nnipa")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ak", "ak", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ak", &[&super::PROFILE]);
    }
}
