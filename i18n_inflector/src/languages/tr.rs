use crate::profile::{LanguageProfile, Rule, VerifiedLexeme};

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("köy", "köyler"),
    VerifiedLexeme::new("araba", "arabalar"),
    VerifiedLexeme::new("şehir", "şehirler"),
];

pub(crate) static PROFILE: LanguageProfile = LanguageProfile::new(
    "tr",
    "tr",
    false,
    Some(Rule::Turkish),
    &[],
    (false, false),
    LEXEMES,
);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tr", &[&super::PROFILE]);
    }
}
