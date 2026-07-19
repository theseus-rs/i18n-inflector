use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("mora", "bara")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("st", "st", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("st", &[&super::PROFILE]);
    }
}
