use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("사람", "사람"),
    VerifiedLexeme::new("집", "집"),
    VerifiedLexeme::new("책", "책"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ko", "ko", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ko", &[&super::PROFILE]);
    }
}
