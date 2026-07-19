use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("munhu", "vanhu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sn", "sn", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sn", &[&super::PROFILE]);
    }
}
