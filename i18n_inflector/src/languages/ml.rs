use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[VerifiedLexeme::new("സർപ്പക്കാട്", "സർപ്പക്കാടുകൾ")];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ml", "ml", false, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ml", &[&super::PROFILE]);
    }
}
