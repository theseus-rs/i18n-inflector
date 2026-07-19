use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("mmadụ", "mmadụ"),
    VerifiedLexeme::new("ụlọ", "ụlọ"),
    VerifiedLexeme::new("akwụkwọ", "akwụkwọ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ig", "ig", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ig", &[&super::PROFILE]);
    }
}
