use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("armej", "armej"),
    VerifiedLexeme::new("ek", "ek"),
    VerifiedLexeme::new("eh", "eh"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mh", "mh", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mh", &[&super::PROFILE]);
    }
}
