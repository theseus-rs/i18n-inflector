use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("คน", "คน"),
    VerifiedLexeme::new("บ้าน", "บ้าน"),
    VerifiedLexeme::new("หนังสือ", "หนังสือ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("th", "th", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("th", &[&super::PROFILE]);
    }
}
