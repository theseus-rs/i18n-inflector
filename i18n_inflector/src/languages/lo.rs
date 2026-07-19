use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("ຄົນ", "ຄົນ"),
    VerifiedLexeme::new("ເຮືອນ", "ເຮືອນ"),
    VerifiedLexeme::new("ປຶ້ມ", "ປຶ້ມ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("lo", "lo", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("lo", &[&super::PROFILE]);
    }
}
