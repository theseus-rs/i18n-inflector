use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("faasaag", "faasaagey")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("gv", "gv", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("gv", &[&super::PROFILE]);
    }
}
