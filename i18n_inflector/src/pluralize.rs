//! Pluralization of nouns.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use phf::phf_map;

use crate::error::{Error, Result};
use crate::languages;

type PluralizeFn = for<'a> fn(&'a str) -> Vec<Cow<'a, str>>;

static PLURALIZE_MAP: phf::Map<&'static str, PluralizeFn> = phf_map! {
    "aa" => languages::aa::pluralize,
    "ab" => languages::ab::pluralize,
    "ae" => languages::ae::pluralize,
    "af" => languages::af::pluralize,
    "ak" => languages::ak::pluralize,
    "am" => languages::am::pluralize,
    "an" => languages::an::pluralize,
    "ar" => languages::ar::pluralize,
    "as" => languages::r#as::pluralize,
    "av" => languages::av::pluralize,
    "ay" => languages::ay::pluralize,
    "az" => languages::az::pluralize,
    "ba" => languages::ba::pluralize,
    "be" => languages::be::pluralize,
    "bg" => languages::bg::pluralize,
    "bi" => languages::bi::pluralize,
    "bm" => languages::bm::pluralize,
    "bn" => languages::bn::pluralize,
    "bo" => languages::bo::pluralize,
    "br" => languages::br::pluralize,
    "bs" => languages::bs::pluralize,
    "ca" => languages::ca::pluralize,
    "ce" => languages::ce::pluralize,
    "ch" => languages::ch::pluralize,
    "co" => languages::co::pluralize,
    "cs" => languages::cs::pluralize,
    "cu" => languages::cu::pluralize,
    "cv" => languages::cv::pluralize,
    "cy" => languages::cy::pluralize,
    "da" => languages::da::pluralize,
    "de" => languages::de::pluralize,
    "dv" => languages::dv::pluralize,
    "dz" => languages::dz::pluralize,
    "ee" => languages::ee::pluralize,
    "el" => languages::el::pluralize,
    "en" => languages::en::pluralize,
    "eo" => languages::eo::pluralize,
    "es" => languages::es::pluralize,
    "et" => languages::et::pluralize,
    "eu" => languages::eu::pluralize,
    "fa" => languages::fa::pluralize,
    "ff" => languages::ff::pluralize,
    "fi" => languages::fi::pluralize,
    "fj" => languages::fj::pluralize,
    "fo" => languages::fo::pluralize,
    "fr" => languages::fr::pluralize,
    "fy" => languages::fy::pluralize,
    "ga" => languages::ga::pluralize,
    "gd" => languages::gd::pluralize,
    "gl" => languages::gl::pluralize,
    "gn" => languages::gn::pluralize,
    "gu" => languages::gu::pluralize,
    "gv" => languages::gv::pluralize,
    "ha" => languages::ha::pluralize,
    "he" => languages::he::pluralize,
    "hi" => languages::hi::pluralize,
    "ho" => languages::ho::pluralize,
    "hr" => languages::hr::pluralize,
    "ht" => languages::ht::pluralize,
    "hu" => languages::hu::pluralize,
    "hy" => languages::hy::pluralize,
    "ia" => languages::ia::pluralize,
    "id" => languages::id::pluralize,
    "ie" => languages::ie::pluralize,
    "ig" => languages::ig::pluralize,
    "ii" => languages::ii::pluralize,
    "ik" => languages::ik::pluralize,
    "is" => languages::is::pluralize,
    "it" => languages::it::pluralize,
    "iu" => languages::iu::pluralize,
    "ja" => languages::ja::pluralize,
    "jv" => languages::jv::pluralize,
    "ka" => languages::ka::pluralize,
    "kg" => languages::kg::pluralize,
    "ki" => languages::ki::pluralize,
    "kj" => languages::kj::pluralize,
    "kk" => languages::kk::pluralize,
    "km" => languages::km::pluralize,
    "kn" => languages::kn::pluralize,
    "ko" => languages::ko::pluralize,
    "ku" => languages::ku::pluralize,
    "kv" => languages::kv::pluralize,
    "kw" => languages::kw::pluralize,
    "ky" => languages::ky::pluralize,
    "la" => languages::la::pluralize,
    "lb" => languages::lb::pluralize,
    "lg" => languages::lg::pluralize,
    "li" => languages::li::pluralize,
    "lo" => languages::lo::pluralize,
    "lt" => languages::lt::pluralize,
    "lu" => languages::lu::pluralize,
    "lv" => languages::lv::pluralize,
    "mg" => languages::mg::pluralize,
    "mi" => languages::mi::pluralize,
    "mk" => languages::mk::pluralize,
    "ml" => languages::ml::pluralize,
    "mn" => languages::mn::pluralize,
    "mr" => languages::mr::pluralize,
    "ms" => languages::ms::pluralize,
    "mt" => languages::mt::pluralize,
    "my" => languages::my::pluralize,
    "nb" => languages::nb::pluralize,
    "nd" => languages::nd::pluralize,
    "ne" => languages::ne::pluralize,
    "nl" => languages::nl::pluralize,
    "nn" => languages::nn::pluralize,
    "no" => languages::no::pluralize,
    "nr" => languages::nr::pluralize,
    "nv" => languages::nv::pluralize,
    "ny" => languages::ny::pluralize,
    "oc" => languages::oc::pluralize,
    "oj" => languages::oj::pluralize,
    "om" => languages::om::pluralize,
    "or" => languages::or::pluralize,
    "os" => languages::os::pluralize,
    "pa" => languages::pa::pluralize,
    "pi" => languages::pi::pluralize,
    "pl" => languages::pl::pluralize,
    "ps" => languages::ps::pluralize,
    "pt" => languages::pt::pluralize,
    "qu" => languages::qu::pluralize,
    "rm" => languages::rm::pluralize,
    "ro" => languages::ro::pluralize,
    "ru" => languages::ru::pluralize,
    "rw" => languages::rw::pluralize,
    "sa" => languages::sa::pluralize,
    "sc" => languages::sc::pluralize,
    "sd" => languages::sd::pluralize,
    "se" => languages::se::pluralize,
    "sg" => languages::sg::pluralize,
    "si" => languages::si::pluralize,
    "sk" => languages::sk::pluralize,
    "sl" => languages::sl::pluralize,
    "sm" => languages::sm::pluralize,
    "sn" => languages::sn::pluralize,
    "so" => languages::so::pluralize,
    "sq" => languages::sq::pluralize,
    "sr" => languages::sr::pluralize,
    "ss" => languages::ss::pluralize,
    "st" => languages::st::pluralize,
    "su" => languages::su::pluralize,
    "sv" => languages::sv::pluralize,
    "sw" => languages::sw::pluralize,
    "ta" => languages::ta::pluralize,
    "te" => languages::te::pluralize,
    "tg" => languages::tg::pluralize,
    "th" => languages::th::pluralize,
    "ti" => languages::ti::pluralize,
    "tk" => languages::tk::pluralize,
    "tl" => languages::tl::pluralize,
    "tn" => languages::tn::pluralize,
    "tr" => languages::tr::pluralize,
    "ts" => languages::ts::pluralize,
    "tt" => languages::tt::pluralize,
    "ug" => languages::ug::pluralize,
    "uk" => languages::uk::pluralize,
    "ur" => languages::ur::pluralize,
    "uz" => languages::uz::pluralize,
    "ve" => languages::ve::pluralize,
    "vi" => languages::vi::pluralize,
    "wa" => languages::wa::pluralize,
    "wo" => languages::wo::pluralize,
    "xh" => languages::xh::pluralize,
    "yi" => languages::yi::pluralize,
    "yo" => languages::yo::pluralize,
    "zh" => languages::zh::pluralize,
    "zu" => languages::zu::pluralize,
};

/// Generates plural form candidates for a word based on the given locale.
///
/// Returns a list of possible plural forms. Multiple candidates are returned because many languages
/// have several pluralization patterns.
///
/// The locale must be a supported ISO 639 two-letter language code (e.g., `"en"`, `"es"`, `"de"`).
///
/// # Errors
///
/// Returns [`Error`] if the locale is not a supported language code.
///
/// # Examples
///
/// ```
/// use i18n_inflector::pluralize;
///
/// // English
/// let result = pluralize("en", "user").unwrap();
/// assert!(result.iter().any(|v| v == "users"));
///
/// let result = pluralize("en", "category").unwrap();
/// assert!(result.iter().any(|v| v == "categories"));
///
/// // Spanish
/// let result = pluralize("es", "ciudad").unwrap();
/// assert!(result.iter().any(|v| v == "ciudades"));
///
/// // Japanese (invariant)
/// assert_eq!(pluralize("ja", "user").unwrap(), vec!["user"]);
///
/// // Unsupported locale returns an error
/// assert!(pluralize("xx", "user").is_err());
/// ```
pub fn pluralize<'a>(locale: &str, name: &'a str) -> Result<Vec<Cow<'a, str>>> {
    PLURALIZE_MAP
        .get(locale)
        .map(|f| f(name))
        .ok_or_else(|| Error::new(format!("unsupported locale: {locale}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_aa() {
        assert_eq!(pluralize("aa", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ab() {
        assert_eq!(pluralize("ab", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ae() {
        assert_eq!(pluralize("ae", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_af() {
        let result = pluralize("af", "kat").unwrap();
        assert!(result.iter().any(|v| v == "kate"));
    }

    #[test]
    fn test_ak() {
        assert_eq!(pluralize("ak", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_am() {
        let result = pluralize("am", "bet").unwrap();
        assert!(result.iter().any(|v| v == "betoch"));
    }

    #[test]
    fn test_an() {
        let result = pluralize("an", "usuario").unwrap();
        assert!(result.iter().any(|v| v == "usuarios"));
    }

    #[test]
    fn test_ar() {
        let result = pluralize("ar", "mustahdim").unwrap();
        assert!(result.iter().any(|v| v == "mustahdimin"));
    }

    #[test]
    fn test_as() {
        let result = pluralize("as", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_av() {
        assert_eq!(pluralize("av", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ay() {
        let result = pluralize("ay", "uta").unwrap();
        assert!(result.iter().any(|v| v == "utanaka"));
    }

    #[test]
    fn test_az() {
        let result = pluralize("az", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_ba() {
        let result = pluralize("ba", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_be() {
        let result = pluralize("be", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkty"));
    }

    #[test]
    fn test_bg() {
        let result = pluralize("bg", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_bi() {
        assert_eq!(pluralize("bi", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_bm() {
        assert_eq!(pluralize("bm", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_bn() {
        let result = pluralize("bn", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_bo() {
        assert_eq!(pluralize("bo", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_br() {
        let result = pluralize("br", "bag").unwrap();
        assert!(result.iter().any(|v| v == "bagou"));
    }

    #[test]
    fn test_bs() {
        let result = pluralize("bs", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_ca() {
        let result = pluralize("ca", "gat").unwrap();
        assert!(result.iter().any(|v| v == "gats"));
    }

    #[test]
    fn test_ce() {
        assert_eq!(pluralize("ce", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ch() {
        assert_eq!(pluralize("ch", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_co() {
        let result = pluralize("co", "prodotto").unwrap();
        assert!(result.iter().any(|v| v == "prodotti"));
    }

    #[test]
    fn test_cs() {
        let result = pluralize("cs", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkty"));
    }

    #[test]
    fn test_cu() {
        assert_eq!(pluralize("cu", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_cv() {
        let result = pluralize("cv", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_cy() {
        let result = pluralize("cy", "cath").unwrap();
        assert!(result.iter().any(|v| v == "cathod"));
    }

    #[test]
    fn test_da() {
        let result = pluralize("da", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkter"));
    }

    #[test]
    fn test_de() {
        let result = pluralize("de", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkte"));
    }

    #[test]
    fn test_dv() {
        let result = pluralize("dv", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_dz() {
        assert_eq!(pluralize("dz", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ee() {
        assert_eq!(pluralize("ee", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_el() {
        let result = pluralize("el", "xristis").unwrap();
        assert!(result.iter().any(|v| v == "xristes"));
    }

    #[test]
    fn test_en() {
        let result = pluralize("en", "user").unwrap();
        assert!(result.iter().any(|v| v == "users"));
    }

    #[test]
    fn test_eo() {
        let result = pluralize("eo", "kato").unwrap();
        assert!(result.iter().any(|v| v == "katoj"));
    }

    #[test]
    fn test_es() {
        let result = pluralize("es", "usuario").unwrap();
        assert!(result.iter().any(|v| v == "usuarios"));
    }

    #[test]
    fn test_et() {
        let result = pluralize("et", "kasutaja").unwrap();
        assert!(result.iter().any(|v| v == "kasutajad"));
    }

    #[test]
    fn test_eu() {
        let result = pluralize("eu", "katu").unwrap();
        assert!(result.iter().any(|v| v == "katuak"));
    }

    #[test]
    fn test_fa() {
        let result = pluralize("fa", "ketab").unwrap();
        assert!(result.iter().any(|v| v == "ketabha"));
    }

    #[test]
    fn test_ff() {
        assert_eq!(pluralize("ff", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_fi() {
        let result = pluralize("fi", "tuote").unwrap();
        assert!(result.iter().any(|v| v == "tuotet"));
    }

    #[test]
    fn test_fj() {
        assert_eq!(pluralize("fj", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_fo() {
        let result = pluralize("fo", "notend").unwrap();
        assert!(result.iter().any(|v| v == "notendur"));
    }

    #[test]
    fn test_fr() {
        let result = pluralize("fr", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_fy() {
        let result = pluralize("fy", "klant").unwrap();
        assert!(result.iter().any(|v| v == "klanten"));
    }

    #[test]
    fn test_ga() {
        let result = pluralize("ga", "usaideoir").unwrap();
        assert!(result.iter().any(|v| v == "usaideoiri"));
    }

    #[test]
    fn test_gd() {
        let result = pluralize("gd", "usaideoir").unwrap();
        assert!(result.iter().any(|v| v == "usaideoiri"));
    }

    #[test]
    fn test_gl() {
        let result = pluralize("gl", "produto").unwrap();
        assert!(result.iter().any(|v| v == "produtos"));
    }

    #[test]
    fn test_gn() {
        let result = pluralize("gn", "mitã").unwrap();
        assert!(result.iter().any(|v| v == "mitãkuéra"));
    }

    #[test]
    fn test_gu() {
        let result = pluralize("gu", "chokra").unwrap();
        assert!(result.iter().any(|v| v == "chokrao"));
    }

    #[test]
    fn test_gv() {
        let result = pluralize("gv", "usaideoir").unwrap();
        assert!(result.iter().any(|v| v == "usaideoiri"));
    }

    #[test]
    fn test_ha() {
        let result = pluralize("ha", "littafi").unwrap();
        assert!(result.iter().any(|v| v == "littafiai"));
    }

    #[test]
    fn test_he() {
        let result = pluralize("he", "meshtatef").unwrap();
        assert!(result.iter().any(|v| v == "meshtatefim"));
    }

    #[test]
    fn test_hi() {
        let result = pluralize("hi", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_ho() {
        assert_eq!(pluralize("ho", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_hr() {
        let result = pluralize("hr", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_ht() {
        assert_eq!(pluralize("ht", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_hu() {
        let result = pluralize("hu", "felhasznalo").unwrap();
        assert!(result.iter().any(|v| v == "felhasznalok"));
    }

    #[test]
    fn test_hy() {
        let result = pluralize("hy", "girq").unwrap();
        assert!(result.iter().any(|v| v == "girqer"));
    }

    #[test]
    fn test_ia() {
        let result = pluralize("ia", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_id() {
        assert_eq!(pluralize("id", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ie() {
        let result = pluralize("ie", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_ig() {
        assert_eq!(pluralize("ig", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ii() {
        assert_eq!(pluralize("ii", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ik() {
        assert_eq!(pluralize("ik", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_is() {
        let result = pluralize("is", "notend").unwrap();
        assert!(result.iter().any(|v| v == "notendur"));
    }

    #[test]
    fn test_it() {
        let result = pluralize("it", "prodotto").unwrap();
        assert!(result.iter().any(|v| v == "prodotti"));
    }

    #[test]
    fn test_iu() {
        assert_eq!(pluralize("iu", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ja() {
        assert_eq!(pluralize("ja", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_jv() {
        assert_eq!(pluralize("jv", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ka() {
        assert_eq!(pluralize("ka", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_kg() {
        let result = pluralize("kg", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_ki() {
        let result = pluralize("ki", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_kj() {
        let result = pluralize("kj", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_kk() {
        let result = pluralize("kk", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_km() {
        assert_eq!(pluralize("km", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_kn() {
        let result = pluralize("kn", "pustaka").unwrap();
        assert!(result.iter().any(|v| v == "pustakagalu"));
    }

    #[test]
    fn test_ko() {
        assert_eq!(pluralize("ko", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ku() {
        let result = pluralize("ku", "dar").unwrap();
        assert!(result.iter().any(|v| v == "daran"));
    }

    #[test]
    fn test_kv() {
        assert_eq!(pluralize("kv", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_kw() {
        let result = pluralize("kw", "cath").unwrap();
        assert!(result.iter().any(|v| v == "cathod"));
    }

    #[test]
    fn test_ky() {
        let result = pluralize("ky", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_la() {
        let result = pluralize("la", "dominus").unwrap();
        assert!(result.iter().any(|v| v == "domini"));
    }

    #[test]
    fn test_lb() {
        let result = pluralize("lb", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkte"));
    }

    #[test]
    fn test_lg() {
        let result = pluralize("lg", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_li() {
        let result = pluralize("li", "klant").unwrap();
        assert!(result.iter().any(|v| v == "klanten"));
    }

    #[test]
    fn test_lo() {
        assert_eq!(pluralize("lo", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_lt() {
        let result = pluralize("lt", "vartotojas").unwrap();
        assert!(result.iter().any(|v| v == "vartotojai"));
    }

    #[test]
    fn test_lu() {
        let result = pluralize("lu", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_lv() {
        let result = pluralize("lv", "lietotajs").unwrap();
        assert!(result.iter().any(|v| v == "lietotaji"));
    }

    #[test]
    fn test_mg() {
        assert_eq!(pluralize("mg", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_mi() {
        assert_eq!(pluralize("mi", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_mk() {
        let result = pluralize("mk", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_ml() {
        let result = pluralize("ml", "pustaka").unwrap();
        assert!(result.iter().any(|v| v == "pustakakal"));
    }

    #[test]
    fn test_mn() {
        let result = pluralize("mn", "nom").unwrap();
        assert!(result.iter().any(|v| v == "nomuud"));
    }

    #[test]
    fn test_mr() {
        let result = pluralize("mr", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_ms() {
        assert_eq!(pluralize("ms", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_mt() {
        let result = pluralize("mt", "utent").unwrap();
        assert!(result.iter().any(|v| v == "utenti"));
    }

    #[test]
    fn test_my() {
        assert_eq!(pluralize("my", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_nb() {
        let result = pluralize("nb", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkter"));
    }

    #[test]
    fn test_nd() {
        let result = pluralize("nd", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_ne() {
        let result = pluralize("ne", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_nl() {
        let result = pluralize("nl", "klant").unwrap();
        assert!(result.iter().any(|v| v == "klanten"));
    }

    #[test]
    fn test_nn() {
        let result = pluralize("nn", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkter"));
    }

    #[test]
    fn test_no() {
        let result = pluralize("no", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkter"));
    }

    #[test]
    fn test_nr() {
        let result = pluralize("nr", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_nv() {
        assert_eq!(pluralize("nv", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ny() {
        let result = pluralize("ny", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_oc() {
        let result = pluralize("oc", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_oj() {
        assert_eq!(pluralize("oj", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_om() {
        let result = pluralize("om", "buug").unwrap();
        assert!(result.iter().any(|v| v == "buugo"));
    }

    #[test]
    fn test_or() {
        let result = pluralize("or", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_os() {
        let result = pluralize("os", "ketab").unwrap();
        assert!(result.iter().any(|v| v == "ketabha"));
    }

    #[test]
    fn test_pa() {
        let result = pluralize("pa", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_pi() {
        assert_eq!(pluralize("pi", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_pl() {
        let result = pluralize("pl", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkty"));
    }

    #[test]
    fn test_ps() {
        let result = pluralize("ps", "kitab").unwrap();
        assert!(result.iter().any(|v| v == "kitabuna"));
    }

    #[test]
    fn test_pt() {
        let result = pluralize("pt", "produto").unwrap();
        assert!(result.iter().any(|v| v == "produtos"));
    }

    #[test]
    fn test_qu() {
        let result = pluralize("qu", "wasi").unwrap();
        assert!(result.iter().any(|v| v == "wasikuna"));
    }

    #[test]
    fn test_rm() {
        let result = pluralize("rm", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_ro() {
        let result = pluralize("ro", "utilizator").unwrap();
        assert!(result.iter().any(|v| v == "utilizatori"));
    }

    #[test]
    fn test_ru() {
        let result = pluralize("ru", "klient").unwrap();
        assert!(result.iter().any(|v| v == "klienty"));
    }

    #[test]
    fn test_rw() {
        let result = pluralize("rw", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_sa() {
        assert_eq!(pluralize("sa", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_sc() {
        let result = pluralize("sc", "prodotto").unwrap();
        assert!(result.iter().any(|v| v == "prodotti"));
    }

    #[test]
    fn test_sd() {
        let result = pluralize("sd", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_se() {
        assert_eq!(pluralize("se", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_sg() {
        assert_eq!(pluralize("sg", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_si() {
        let result = pluralize("si", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_sk() {
        let result = pluralize("sk", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkty"));
    }

    #[test]
    fn test_sl() {
        let result = pluralize("sl", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_sm() {
        assert_eq!(pluralize("sm", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_sn() {
        let result = pluralize("sn", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_so() {
        let result = pluralize("so", "buug").unwrap();
        assert!(result.iter().any(|v| v == "buugo"));
    }

    #[test]
    fn test_sq() {
        let result = pluralize("sq", "perdorues").unwrap();
        assert!(result.iter().any(|v| v == "perdoruese"));
    }

    #[test]
    fn test_sr() {
        let result = pluralize("sr", "korisnik").unwrap();
        assert!(result.iter().any(|v| v == "korisnici"));
    }

    #[test]
    fn test_ss() {
        let result = pluralize("ss", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_st() {
        let result = pluralize("st", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_su() {
        assert_eq!(pluralize("su", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_sv() {
        let result = pluralize("sv", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkter"));
    }

    #[test]
    fn test_sw() {
        let result = pluralize("sw", "kitabu").unwrap();
        assert!(result.iter().any(|v| v == "kitabuni"));
    }

    #[test]
    fn test_ta() {
        let result = pluralize("ta", "pustaka").unwrap();
        assert!(result.iter().any(|v| v == "pustakakal"));
    }

    #[test]
    fn test_te() {
        let result = pluralize("te", "pustakaa").unwrap();
        assert!(result.iter().any(|v| v == "pustakaalu"));
    }

    #[test]
    fn test_tg() {
        let result = pluralize("tg", "ketab").unwrap();
        assert!(result.iter().any(|v| v == "ketabha"));
    }

    #[test]
    fn test_th() {
        assert_eq!(pluralize("th", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ti() {
        let result = pluralize("ti", "bet").unwrap();
        assert!(result.iter().any(|v| v == "betoch"));
    }

    #[test]
    fn test_tk() {
        let result = pluralize("tk", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_tl() {
        assert_eq!(pluralize("tl", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_tn() {
        let result = pluralize("tn", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_tr() {
        let result = pluralize("tr", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_ts() {
        let result = pluralize("ts", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_tt() {
        let result = pluralize("tt", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_ug() {
        let result = pluralize("ug", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_uk() {
        let result = pluralize("uk", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produkty"));
    }

    #[test]
    fn test_ur() {
        let result = pluralize("ur", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_uz() {
        let result = pluralize("uz", "kullanici").unwrap();
        assert!(result.iter().any(|v| v == "kullanicilar"));
    }

    #[test]
    fn test_ve() {
        let result = pluralize("ve", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_vi() {
        assert_eq!(pluralize("vi", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_wa() {
        let result = pluralize("wa", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_wo() {
        assert_eq!(pluralize("wo", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_xh() {
        let result = pluralize("xh", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_yi() {
        let result = pluralize("yi", "produkt").unwrap();
        assert!(result.iter().any(|v| v == "produktin"));
    }

    #[test]
    fn test_yo() {
        assert_eq!(pluralize("yo", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_zh() {
        assert_eq!(pluralize("zh", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_zu() {
        let result = pluralize("zu", "incwadi").unwrap();
        assert!(result.iter().any(|v| v == "incwadini"));
    }

    #[test]
    fn test_unsupported_locale() {
        let err = pluralize("xx", "user").unwrap_err();
        assert_eq!(err.to_string(), "unsupported locale: xx");
    }
}
