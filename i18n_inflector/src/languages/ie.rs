use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "after-vowel-s",
        "nouns ending in a vowel and taking -s",
        Rule::Suffix("s"),
    ),
    LexicalClassSpec::new(
        "after-most-consonants-es",
        "nouns ending in most consonants and taking -es",
        Rule::Suffix("es"),
    ),
    LexicalClassSpec::new(
        "final-c-g-m-s",
        "nouns ending in -c, -g, or -m and taking -s",
        Rule::Suffix("s"),
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("angul", "angules"),
    VerifiedLexeme::new("cat", "cats"),
    VerifiedLexeme::new("cato", "catos"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("ie", "ie", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("ie", &[&super::PROFILE]);
    }
}
