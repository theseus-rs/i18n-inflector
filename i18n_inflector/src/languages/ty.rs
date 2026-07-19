use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("ʻuru", "ʻuru"),
    VerifiedLexeme::new("ʻana", "ʻana"),
    VerifiedLexeme::new("taro", "taro"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ty", "ty", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ty", &[&super::PROFILE]);
    }
}
