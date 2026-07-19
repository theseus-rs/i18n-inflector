use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tangata", "tangata"),
    VerifiedLexeme::new("whare", "whare"),
    VerifiedLexeme::new("pukapuka", "pukapuka"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("mi", "mi", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("mi", &[&super::PROFILE]);
    }
}
