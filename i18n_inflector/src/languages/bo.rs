use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("མི", "མི"),
    VerifiedLexeme::new("ཁང་པ", "ཁང་པ"),
    VerifiedLexeme::new("དེབ", "དེབ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("bo", "bo", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("bo", &[&super::PROFILE]);
    }
}
