use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("eñame", "eñame"),
    VerifiedLexeme::new("ewak", "ewak"),
    VerifiedLexeme::new("robar", "robar"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("na", "na", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("na", &[&super::PROFILE]);
    }
}
