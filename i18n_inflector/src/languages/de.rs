use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "suffix-e",
        "nouns taking -e without a stem change",
        Rule::Suffix("e"),
    ),
    LexicalClassSpec::new(
        "suffix-en",
        "nouns taking -en without a stem change",
        Rule::Suffix("en"),
    ),
    LexicalClassSpec::new(
        "suffix-er",
        "nouns taking -er without a stem change",
        Rule::Suffix("er"),
    ),
    LexicalClassSpec::new(
        "suffix-s",
        "loanwords taking -s without a stem change",
        Rule::Suffix("s"),
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("Druck", "Drucke"),
    VerifiedLexeme::new("Wurmloch", "Wurmlöcher"),
    VerifiedLexeme::new("Etui", "Etuis"),
    VerifiedLexeme::new("Kind", "Kinder"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("de", "de", false, None, CLASSES, (true, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("de", &[&super::PROFILE]);
    }
}
