use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("người", "người"),
    VerifiedLexeme::new("nhà", "nhà"),
    VerifiedLexeme::new("sách", "sách"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("vi", "vi", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("vi", &[&super::PROFILE]);
    }
}
