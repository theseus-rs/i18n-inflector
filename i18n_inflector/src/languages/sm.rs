use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tagata", "tagata"),
    VerifiedLexeme::new("fale", "fale"),
    VerifiedLexeme::new("tusi", "tusi"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("sm", "sm", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("sm", &[&super::PROFILE]);
    }
}
