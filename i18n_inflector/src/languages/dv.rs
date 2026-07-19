use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("ކިރު", "ކިރުތައް")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("dv", "dv", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("dv", &[&super::PROFILE]);
    }
}
