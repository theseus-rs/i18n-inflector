use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("moun", "moun"),
    VerifiedLexeme::new("kay", "kay"),
    VerifiedLexeme::new("liv", "liv"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ht", "ht", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ht", &[&super::PROFILE]);
    }
}
