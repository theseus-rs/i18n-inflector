use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("လူ", "လူ"),
    VerifiedLexeme::new("အိမ်", "အိမ်"),
    VerifiedLexeme::new("စာအုပ်", "စာအုပ်"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("my", "my", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("my", &[&super::PROFILE]);
    }
}
