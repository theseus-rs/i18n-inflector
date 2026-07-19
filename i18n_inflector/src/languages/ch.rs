use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("cheʼlu", "mañeʼlu")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ch", "ch", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ch", &[&super::PROFILE]);
    }
}
