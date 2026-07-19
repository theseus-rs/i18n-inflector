use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("tangata", "tangata"),
    VerifiedLexeme::new("fale", "fale"),
    VerifiedLexeme::new("tohi", "tohi"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("to", "to", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("to", &[&super::PROFILE]);
    }
}
