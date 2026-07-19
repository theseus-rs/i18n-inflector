use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tamata", "tamata"),
    VerifiedLexeme::new("vale", "vale"),
    VerifiedLexeme::new("ivola", "ivola"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("fj", "fj", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("fj", &[&super::PROFILE]);
    }
}
