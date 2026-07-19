use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[LexicalClassSpec::new(
    "vowel-harmony",
    "regular nouns whose plural follows two-way vowel harmony",
    Rule::Turkish,
)];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("köy", "köyler"),
    VerifiedLexeme::new("araba", "arabalar"),
    VerifiedLexeme::new("şehir", "şehirler"),
    VerifiedLexeme::new("saat", "saatler"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("tr", "tr", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("tr", &[&super::PROFILE]);
    }
}
