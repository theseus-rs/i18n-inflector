use crate::profile::{LanguageProfile, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("មនុស្ស", "មនុស្ស"),
    VerifiedLexeme::new("ផ្ទះ", "ផ្ទះ"),
    VerifiedLexeme::new("សៀវភៅ", "សៀវភៅ"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("km", "km", true, None, &[], (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("km", &[&super::PROFILE]);
    }
}
