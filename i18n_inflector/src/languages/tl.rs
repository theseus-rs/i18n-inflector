use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tao", "tao"),
    VerifiedLexeme::new("bahay", "bahay"),
    VerifiedLexeme::new("aklat", "aklat"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tl", "tl", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tl", &[&super::PROFILE]);
    }
}
