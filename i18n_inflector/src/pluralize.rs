//! Pluralization of nouns.

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use crate::error::{Error, Result};
use crate::languages;

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
#[expect(clippy::too_many_lines)]
pub fn pluralize<'a>(locale: &str, name: &'a str) -> Result<Vec<Cow<'a, str>>> {
    match locale {
        "af" => Ok(languages::af::pluralize(name)),
        "am" => Ok(languages::am::pluralize(name)),
        "an" => Ok(languages::an::pluralize(name)),
        "ar" => Ok(languages::ar::pluralize(name)),
        "as" => Ok(languages::r#as::pluralize(name)),
        "az" => Ok(languages::az::pluralize(name)),
        "be" => Ok(languages::be::pluralize(name)),
        "bg" => Ok(languages::bg::pluralize(name)),
        "bn" => Ok(languages::bn::pluralize(name)),
        "bo" => Ok(languages::bo::pluralize(name)),
        "br" => Ok(languages::br::pluralize(name)),
        "bs" => Ok(languages::bs::pluralize(name)),
        "ca" => Ok(languages::ca::pluralize(name)),
        "co" => Ok(languages::co::pluralize(name)),
        "cs" => Ok(languages::cs::pluralize(name)),
        "cy" => Ok(languages::cy::pluralize(name)),
        "da" => Ok(languages::da::pluralize(name)),
        "de" => Ok(languages::de::pluralize(name)),
        "el" => Ok(languages::el::pluralize(name)),
        "en" => Ok(languages::en::pluralize(name)),
        "eo" => Ok(languages::eo::pluralize(name)),
        "es" => Ok(languages::es::pluralize(name)),
        "et" => Ok(languages::et::pluralize(name)),
        "eu" => Ok(languages::eu::pluralize(name)),
        "fa" => Ok(languages::fa::pluralize(name)),
        "fi" => Ok(languages::fi::pluralize(name)),
        "fo" => Ok(languages::fo::pluralize(name)),
        "fr" => Ok(languages::fr::pluralize(name)),
        "fy" => Ok(languages::fy::pluralize(name)),
        "ga" => Ok(languages::ga::pluralize(name)),
        "gd" => Ok(languages::gd::pluralize(name)),
        "gl" => Ok(languages::gl::pluralize(name)),
        "gu" => Ok(languages::gu::pluralize(name)),
        "gv" => Ok(languages::gv::pluralize(name)),
        "ha" => Ok(languages::ha::pluralize(name)),
        "he" => Ok(languages::he::pluralize(name)),
        "hi" => Ok(languages::hi::pluralize(name)),
        "hr" => Ok(languages::hr::pluralize(name)),
        "ht" => Ok(languages::ht::pluralize(name)),
        "hu" => Ok(languages::hu::pluralize(name)),
        "hy" => Ok(languages::hy::pluralize(name)),
        "id" => Ok(languages::id::pluralize(name)),
        "ig" => Ok(languages::ig::pluralize(name)),
        "is" => Ok(languages::is::pluralize(name)),
        "it" => Ok(languages::it::pluralize(name)),
        "ja" => Ok(languages::ja::pluralize(name)),
        "jv" => Ok(languages::jv::pluralize(name)),
        "ka" => Ok(languages::ka::pluralize(name)),
        "kk" => Ok(languages::kk::pluralize(name)),
        "km" => Ok(languages::km::pluralize(name)),
        "kn" => Ok(languages::kn::pluralize(name)),
        "ko" => Ok(languages::ko::pluralize(name)),
        "ku" => Ok(languages::ku::pluralize(name)),
        "kw" => Ok(languages::kw::pluralize(name)),
        "ky" => Ok(languages::ky::pluralize(name)),
        "la" => Ok(languages::la::pluralize(name)),
        "lb" => Ok(languages::lb::pluralize(name)),
        "lo" => Ok(languages::lo::pluralize(name)),
        "lt" => Ok(languages::lt::pluralize(name)),
        "lv" => Ok(languages::lv::pluralize(name)),
        "mg" => Ok(languages::mg::pluralize(name)),
        "mi" => Ok(languages::mi::pluralize(name)),
        "mk" => Ok(languages::mk::pluralize(name)),
        "ml" => Ok(languages::ml::pluralize(name)),
        "mn" => Ok(languages::mn::pluralize(name)),
        "mr" => Ok(languages::mr::pluralize(name)),
        "ms" => Ok(languages::ms::pluralize(name)),
        "mt" => Ok(languages::mt::pluralize(name)),
        "my" => Ok(languages::my::pluralize(name)),
        "nb" => Ok(languages::nb::pluralize(name)),
        "nd" => Ok(languages::nd::pluralize(name)),
        "ne" => Ok(languages::ne::pluralize(name)),
        "nl" => Ok(languages::nl::pluralize(name)),
        "nn" => Ok(languages::nn::pluralize(name)),
        "no" => Ok(languages::no::pluralize(name)),
        "nr" => Ok(languages::nr::pluralize(name)),
        "oc" => Ok(languages::oc::pluralize(name)),
        "or" => Ok(languages::or::pluralize(name)),
        "pa" => Ok(languages::pa::pluralize(name)),
        "pl" => Ok(languages::pl::pluralize(name)),
        "ps" => Ok(languages::ps::pluralize(name)),
        "pt" => Ok(languages::pt::pluralize(name)),
        "qu" => Ok(languages::qu::pluralize(name)),
        "rm" => Ok(languages::rm::pluralize(name)),
        "ro" => Ok(languages::ro::pluralize(name)),
        "ru" => Ok(languages::ru::pluralize(name)),
        "rw" => Ok(languages::rw::pluralize(name)),
        "sc" => Ok(languages::sc::pluralize(name)),
        "sd" => Ok(languages::sd::pluralize(name)),
        "si" => Ok(languages::si::pluralize(name)),
        "sk" => Ok(languages::sk::pluralize(name)),
        "sl" => Ok(languages::sl::pluralize(name)),
        "sm" => Ok(languages::sm::pluralize(name)),
        "sn" => Ok(languages::sn::pluralize(name)),
        "so" => Ok(languages::so::pluralize(name)),
        "sq" => Ok(languages::sq::pluralize(name)),
        "sr" => Ok(languages::sr::pluralize(name)),
        "ss" => Ok(languages::ss::pluralize(name)),
        "st" => Ok(languages::st::pluralize(name)),
        "su" => Ok(languages::su::pluralize(name)),
        "sv" => Ok(languages::sv::pluralize(name)),
        "sw" => Ok(languages::sw::pluralize(name)),
        "ta" => Ok(languages::ta::pluralize(name)),
        "te" => Ok(languages::te::pluralize(name)),
        "tg" => Ok(languages::tg::pluralize(name)),
        "th" => Ok(languages::th::pluralize(name)),
        "ti" => Ok(languages::ti::pluralize(name)),
        "tk" => Ok(languages::tk::pluralize(name)),
        "tl" => Ok(languages::tl::pluralize(name)),
        "tn" => Ok(languages::tn::pluralize(name)),
        "tr" => Ok(languages::tr::pluralize(name)),
        "ts" => Ok(languages::ts::pluralize(name)),
        "tt" => Ok(languages::tt::pluralize(name)),
        "ug" => Ok(languages::ug::pluralize(name)),
        "uk" => Ok(languages::uk::pluralize(name)),
        "ur" => Ok(languages::ur::pluralize(name)),
        "uz" => Ok(languages::uz::pluralize(name)),
        "ve" => Ok(languages::ve::pluralize(name)),
        "vi" => Ok(languages::vi::pluralize(name)),
        "wo" => Ok(languages::wo::pluralize(name)),
        "xh" => Ok(languages::xh::pluralize(name)),
        "yi" => Ok(languages::yi::pluralize(name)),
        "yo" => Ok(languages::yo::pluralize(name)),
        "zh" => Ok(languages::zh::pluralize(name)),
        "zu" => Ok(languages::zu::pluralize(name)),
        _ => Err(Error::new(format!("unsupported locale: {locale}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_af() {
        let result = pluralize("af", "kat").unwrap();
        assert!(result.iter().any(|v| v == "kate"));
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
    fn test_az() {
        let result = pluralize("az", "kullanici").unwrap();
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
    fn test_fi() {
        let result = pluralize("fi", "tuote").unwrap();
        assert!(result.iter().any(|v| v == "tuotet"));
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
    fn test_id() {
        assert_eq!(pluralize("id", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_ig() {
        assert_eq!(pluralize("ig", "user").unwrap(), vec!["user"]);
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
    fn test_lo() {
        assert_eq!(pluralize("lo", "user").unwrap(), vec!["user"]);
    }

    #[test]
    fn test_lt() {
        let result = pluralize("lt", "vartotojas").unwrap();
        assert!(result.iter().any(|v| v == "vartotojai"));
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
    fn test_oc() {
        let result = pluralize("oc", "utilisateur").unwrap();
        assert!(result.iter().any(|v| v == "utilisateurs"));
    }

    #[test]
    fn test_or() {
        let result = pluralize("or", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
    }

    #[test]
    fn test_pa() {
        let result = pluralize("pa", "upyogakarta").unwrap();
        assert!(result.iter().any(|v| v == "upyogakartaon"));
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
