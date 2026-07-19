use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("кыв", "кывъяс")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("kv", "kv", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("kv", &[&super::PROFILE]);
    }
}
