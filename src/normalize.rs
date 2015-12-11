use regex::Regex;
use unidecode::unidecode;

fn remove_tokens<'a>(s: &str) -> String {
    let corp_toks = Regex::new(r"\b(llc|inc|ltd|pte|intl|gmbh|corp|domaine|chateau|corporation|company|co|sa|sl|winery|wines|bodega|slu|vineyard|winework|cellar|the)\b").unwrap();

    let rm_ch = Regex::new(r"\bch\b").unwrap().replace_all(s, "chateau");
    let rm_dom = Regex::new(r"\bdom\b").unwrap().replace_all(&rm_ch, "domaine");
    let rm_mt = Regex::new(r"\bmtn\b").unwrap().replace_all(&rm_dom, "mountain");

    let s3 = corp_toks.replace_all(&rm_mt, " ");
    s3.replace("&", " ")
        .replace(" de ", " de")
        .replace(".", " ")
        .replace(",", "")
        .trim().to_string()
}

pub fn normalize(s: &str) -> String {
    let lower = &unidecode(s).to_lowercase();
    let trailing_s = Regex::new(r"'?s\b").unwrap();
    let x = trailing_s.replace_all(lower, "");
    remove_tokens(&x)
}

#[cfg(test)]

mod tests {
    use super::normalize;

    #[test]
    fn removes_llc() {
        assert_eq!(normalize("Peach LLC"), normalize("Peach"))
    }

    #[test]
    fn removes_domaine() {
        assert_eq!(normalize("Domaine FFF"), normalize("FFF"))
    }

    #[test]
    fn removes_trailing_s() {
        assert_eq!(normalize("Strands"), normalize("Strand"));
        assert_eq!(normalize("Strand's"), normalize("Strand"));
        assert!(normalize("Strandes") != normalize("Strand"));
    }
}
