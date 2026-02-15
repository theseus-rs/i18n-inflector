//! Registry of all language rules, keyed by ISO 639-1 language code.

use phf::phf_map;

use crate::language_rules::LanguageRuleSet;
use crate::languages;

/// A compile-time map of ISO 639-1 language codes to their [`LanguageRuleSet`].
pub(crate) static LANGUAGE_RULES_MAP: phf::Map<&'static str, &'static LanguageRuleSet> = phf_map! {
    "aa" => &languages::aa::RULES,
    "ab" => &languages::ab::RULES,
    "ae" => &languages::ae::RULES,
    "af" => &languages::af::RULES,
    "ak" => &languages::ak::RULES,
    "am" => &languages::am::RULES,
    "an" => &languages::an::RULES,
    "ar" => &languages::ar::RULES,
    "as" => &languages::r#as::RULES,
    "av" => &languages::av::RULES,
    "ay" => &languages::ay::RULES,
    "az" => &languages::az::RULES,
    "ba" => &languages::ba::RULES,
    "be" => &languages::be::RULES,
    "bg" => &languages::bg::RULES,
    "bi" => &languages::bi::RULES,
    "bm" => &languages::bm::RULES,
    "bn" => &languages::bn::RULES,
    "bo" => &languages::bo::RULES,
    "br" => &languages::br::RULES,
    "bs" => &languages::bs::RULES,
    "ca" => &languages::ca::RULES,
    "ce" => &languages::ce::RULES,
    "ch" => &languages::ch::RULES,
    "co" => &languages::co::RULES,
    "cs" => &languages::cs::RULES,
    "cu" => &languages::cu::RULES,
    "cv" => &languages::cv::RULES,
    "cy" => &languages::cy::RULES,
    "da" => &languages::da::RULES,
    "de" => &languages::de::RULES,
    "dv" => &languages::dv::RULES,
    "dz" => &languages::dz::RULES,
    "ee" => &languages::ee::RULES,
    "el" => &languages::el::RULES,
    "en" => &languages::en::RULES,
    "eo" => &languages::eo::RULES,
    "es" => &languages::es::RULES,
    "et" => &languages::et::RULES,
    "eu" => &languages::eu::RULES,
    "fa" => &languages::fa::RULES,
    "ff" => &languages::ff::RULES,
    "fi" => &languages::fi::RULES,
    "fj" => &languages::fj::RULES,
    "fo" => &languages::fo::RULES,
    "fr" => &languages::fr::RULES,
    "fy" => &languages::fy::RULES,
    "ga" => &languages::ga::RULES,
    "gd" => &languages::gd::RULES,
    "gl" => &languages::gl::RULES,
    "gn" => &languages::gn::RULES,
    "gu" => &languages::gu::RULES,
    "gv" => &languages::gv::RULES,
    "ha" => &languages::ha::RULES,
    "he" => &languages::he::RULES,
    "hi" => &languages::hi::RULES,
    "ho" => &languages::ho::RULES,
    "hr" => &languages::hr::RULES,
    "ht" => &languages::ht::RULES,
    "hu" => &languages::hu::RULES,
    "hy" => &languages::hy::RULES,
    "ia" => &languages::ia::RULES,
    "id" => &languages::id::RULES,
    "ie" => &languages::ie::RULES,
    "ig" => &languages::ig::RULES,
    "ii" => &languages::ii::RULES,
    "ik" => &languages::ik::RULES,
    "is" => &languages::is::RULES,
    "it" => &languages::it::RULES,
    "iu" => &languages::iu::RULES,
    "ja" => &languages::ja::RULES,
    "jv" => &languages::jv::RULES,
    "ka" => &languages::ka::RULES,
    "kg" => &languages::kg::RULES,
    "ki" => &languages::ki::RULES,
    "kj" => &languages::kj::RULES,
    "kk" => &languages::kk::RULES,
    "km" => &languages::km::RULES,
    "kn" => &languages::kn::RULES,
    "ko" => &languages::ko::RULES,
    "ku" => &languages::ku::RULES,
    "kv" => &languages::kv::RULES,
    "kw" => &languages::kw::RULES,
    "ky" => &languages::ky::RULES,
    "la" => &languages::la::RULES,
    "lb" => &languages::lb::RULES,
    "lg" => &languages::lg::RULES,
    "li" => &languages::li::RULES,
    "lo" => &languages::lo::RULES,
    "lt" => &languages::lt::RULES,
    "lu" => &languages::lu::RULES,
    "lv" => &languages::lv::RULES,
    "mg" => &languages::mg::RULES,
    "mi" => &languages::mi::RULES,
    "mk" => &languages::mk::RULES,
    "ml" => &languages::ml::RULES,
    "mn" => &languages::mn::RULES,
    "mr" => &languages::mr::RULES,
    "ms" => &languages::ms::RULES,
    "mt" => &languages::mt::RULES,
    "my" => &languages::my::RULES,
    "nb" => &languages::nb::RULES,
    "nd" => &languages::nd::RULES,
    "ne" => &languages::ne::RULES,
    "nl" => &languages::nl::RULES,
    "nn" => &languages::nn::RULES,
    "no" => &languages::no::RULES,
    "nr" => &languages::nr::RULES,
    "nv" => &languages::nv::RULES,
    "ny" => &languages::ny::RULES,
    "oc" => &languages::oc::RULES,
    "oj" => &languages::oj::RULES,
    "om" => &languages::om::RULES,
    "or" => &languages::or::RULES,
    "os" => &languages::os::RULES,
    "pa" => &languages::pa::RULES,
    "pi" => &languages::pi::RULES,
    "pl" => &languages::pl::RULES,
    "ps" => &languages::ps::RULES,
    "pt" => &languages::pt::RULES,
    "qu" => &languages::qu::RULES,
    "rm" => &languages::rm::RULES,
    "ro" => &languages::ro::RULES,
    "ru" => &languages::ru::RULES,
    "rw" => &languages::rw::RULES,
    "sa" => &languages::sa::RULES,
    "sc" => &languages::sc::RULES,
    "sd" => &languages::sd::RULES,
    "se" => &languages::se::RULES,
    "sg" => &languages::sg::RULES,
    "si" => &languages::si::RULES,
    "sk" => &languages::sk::RULES,
    "sl" => &languages::sl::RULES,
    "sm" => &languages::sm::RULES,
    "sn" => &languages::sn::RULES,
    "so" => &languages::so::RULES,
    "sq" => &languages::sq::RULES,
    "sr" => &languages::sr::RULES,
    "ss" => &languages::ss::RULES,
    "st" => &languages::st::RULES,
    "su" => &languages::su::RULES,
    "sv" => &languages::sv::RULES,
    "sw" => &languages::sw::RULES,
    "ta" => &languages::ta::RULES,
    "te" => &languages::te::RULES,
    "tg" => &languages::tg::RULES,
    "th" => &languages::th::RULES,
    "ti" => &languages::ti::RULES,
    "tk" => &languages::tk::RULES,
    "tl" => &languages::tl::RULES,
    "tn" => &languages::tn::RULES,
    "tr" => &languages::tr::RULES,
    "ts" => &languages::ts::RULES,
    "tt" => &languages::tt::RULES,
    "ug" => &languages::ug::RULES,
    "uk" => &languages::uk::RULES,
    "ur" => &languages::ur::RULES,
    "uz" => &languages::uz::RULES,
    "ve" => &languages::ve::RULES,
    "vi" => &languages::vi::RULES,
    "wa" => &languages::wa::RULES,
    "wo" => &languages::wo::RULES,
    "xh" => &languages::xh::RULES,
    "yi" => &languages::yi::RULES,
    "yo" => &languages::yo::RULES,
    "zh" => &languages::zh::RULES,
    "zu" => &languages::zu::RULES,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language_rules::LanguageRules;

    #[test]
    fn test_all_languages_registered() {
        assert_eq!(LANGUAGE_RULES_MAP.len(), 167);
    }

    #[test]
    fn test_lookup_english() {
        let rules = LANGUAGE_RULES_MAP.get("en").unwrap();
        assert_eq!(rules.language(), "en");
    }

    #[test]
    fn test_lookup_turkish() {
        let rules = LANGUAGE_RULES_MAP.get("tr").unwrap();
        assert_eq!(rules.language(), "tr");
    }

    #[test]
    fn test_lookup_delegate_has_own_language() {
        let rules = LANGUAGE_RULES_MAP.get("az").unwrap();
        assert_eq!(rules.language(), "az");
    }

    #[test]
    fn test_lookup_nonexistent() {
        assert!(LANGUAGE_RULES_MAP.get("xx").is_none());
    }

    #[test]
    fn test_all_languages_have_correct_language_code() {
        for (code, rules) in LANGUAGE_RULES_MAP.entries() {
            assert_eq!(*code, rules.language());
        }
    }

    #[test]
    fn test_singularize_via_registry() {
        let rules = LANGUAGE_RULES_MAP.get("en").unwrap();
        assert_eq!(rules.singularize("users"), "user");
    }

    #[test]
    fn test_pluralize_via_registry() {
        let rules = LANGUAGE_RULES_MAP.get("en").unwrap();
        let result = rules.pluralize("user");
        assert!(result.iter().any(|v| v == "users"));
    }
}
