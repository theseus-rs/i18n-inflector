use crate::profile::{LanguageProfile, LexicalClassSpec, Rule, VerifiedLexeme};

const CLASSES: &[LexicalClassSpec] = &[
    LexicalClassSpec::new(
        "masculine-o",
        "regular nouns changing -o to -i",
        Rule::ReplaceSuffix {
            singular: "o",
            plural: "i",
        },
    ),
    LexicalClassSpec::new(
        "feminine-a",
        "regular nouns changing -a to -e",
        Rule::ReplaceSuffix {
            singular: "a",
            plural: "e",
        },
    ),
    LexicalClassSpec::new(
        "regular-e",
        "regular nouns changing -e to -i",
        Rule::ReplaceSuffix {
            singular: "e",
            plural: "i",
        },
    ),
];

const LEXEMES: &[VerifiedLexeme] = &[
    VerifiedLexeme::new("cardinale", "cardinali"),
    VerifiedLexeme::new("casa", "case"),
    VerifiedLexeme::new("libro", "libri"),
    VerifiedLexeme::new("uomo", "uomini"),
];

pub(crate) static PROFILE: LanguageProfile =
    LanguageProfile::new("it", "it", false, None, CLASSES, (false, false), LEXEMES);

#[cfg(test)]
mod tests {
    #[test]
    fn profile_data_is_valid() {
        crate::languages::assert_language_profiles("it", &[&super::PROFILE]);
    }
}
