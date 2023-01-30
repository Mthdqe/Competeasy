mod constant;
/**
 * \brief  Scraper class to get values from FFVB website
 *
 * \file   scrap.rs
 * \author Dique Mathieu
 * \date   January, 29th 2023
 */
/* ------------------------------------------------------------------------- */
mod entity;

/* ------------------------------------------------------------------------- */
use reqwest::*;
use scraper::html::*;
use scraper::*;

/* ------------------------------------------------------------------------- */
/** \enum  HtmlType
 *  \brief Enumerate the different kind of values that can be scraped from html
 */
pub enum HtmlType {
    InnerHtml, /*< The value is the internal text between two html balises */
    Href,      /*< The value of a link */
}

/* ------------------------------------------------------------------------- */
/**
 * \struct Scraper
 * \brief  Scraper structure that is built around the scraping library
 */
pub struct Scraper {
    doc: Html, /*< The document representing the scraped web page */
}

/* ------------------------------------------------------------------------- */
impl Scraper {
    /**
     * \brief Scraper structure constructor
     * \param uri The uri of the web page to scrap
     */
    pub async fn new(uri: &str) -> Scraper {
        /* Create the client builder, accept invalid_certificates */
        let builder: ClientBuilder = ClientBuilder::new().danger_accept_invalid_certs(true);

        /* Build the client */
        let client: Client = builder.build().unwrap();

        /* Get the html content as a String and block to avoid async */
        let html = client.get(uri).send().await.unwrap().text().await.unwrap();

        Scraper {
            doc: Html::parse_document(&html), /* Parse the html document */
        }
    }

    /**
     *  \brief Parse the inner html of an ElementRef
     *  \param elt_ref The ElementRef to parse
     *  \return String The inner html of this element ref
     */
    fn parse_inner_html(elt_ref: &ElementRef) -> String {
        elt_ref.inner_html()
    }

    /**
     *  \brief
     */
    fn parse_href(elt_ref: &ElementRef) -> String {
        String::from(elt_ref.value().attr("href").unwrap())
    }

    /**
     * \brief  Scrap the sequence and gets the inner html content
     * \param  sequence The sequence to parse from the document
     * \param  html_type The type of value we want to scrap
     * \return Vec<String> The list of content parsed from the sequence
     */
    pub fn scrap_sequence(&self, sequence: &str, html_type: HtmlType) -> Vec<String> {
        /* Create the selector of the given sequence */
        let selector: Selector = Selector::parse(sequence).unwrap();

        /* Select the right parsing function depending on the type we want
         * to parse */
        let parsing_func = match html_type {
            HtmlType::InnerHtml => |x| Scraper::parse_inner_html(&x),
            HtmlType::Href => |x| Scraper::parse_href(&x),
        };

        /* Parse the sequence from the document */
        self.doc
            .select(&selector)
            .map(|x| parsing_func(x))
            .collect()
    }
}

/* ------------------------------------------------------------------------- */
/**
 * \brief Scrap the different competitions, i.e give a list of the available
 *        competitions to check
 * \return Vec<entity::Competition> The list of competitions
 */
pub fn scrap_competitions() -> Vec<entity::Competition> {
    let mut competitions: Vec<entity::Competition> = Vec::new();

    /* This function is hardcoded as we only want to manage some competitions */

    competitions.push(entity::Competition::new(
        constant::CHAMP_NAT,
        constant::CHAMP_NAT_URL,
    ));

    competitions.push(entity::Competition::new(
        constant::CHAMP_REG,
        constant::CHAMP_REG_URL,
    ));

    competitions.push(entity::Competition::new(
        constant::CHAMP_DEP,
        constant::CHAMP_DEP_URL,
    ));

    competitions
}

/* ------------------------------------------------------------------------- */
/**
 * \brief Scrap the regions of the given Competition
 * \param competition The competition to get the regions from
 * \return Vec<entity::Region> The list of the regions of the matching competition
 */
pub async fn scrap_regions(competition: &entity::Competition) -> Vec<entity::Region> {
    /* Error management */
    if competition.name() == constant::CHAMP_NAT || competition.name() == constant::CHAMP_DEP {
        panic!();
    }

    /* Instanciate the return vector */
    let mut regions: Vec<entity::Region> = Vec::new();

    /* Instanciate the Scraper */
    let scraper: Scraper = Scraper::new(competition.url()).await;

    /* Scrap the region names */
    let names: Vec<String> = scraper.scrap_sequence("thead tr td", HtmlType::InnerHtml);

    /* Scrap the region pools url */
    let pools: Vec<String> = scraper.scrap_sequence("tbody tr td ul li a", HtmlType::Href);

    /* Build the vector of regions */
    for i in 0..std::cmp::min(names.len(), pools.len()) {
        regions.push(entity::Region::new(&names[i], &pools[i]));
    }

    regions
}

/**
 * \brief Scrap the different
 */
/* ------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use crate::scrap::*;

    #[test]
    fn test_competitions_scrap() {
        let competitions: Vec<entity::Competition> = scrap_competitions();
        assert_eq!(competitions.len(), 3);

        assert!(competitions.contains(&entity::Competition::new(
            constant::CHAMP_NAT,
            constant::CHAMP_NAT_URL,
        )));

        assert!(competitions.contains(&entity::Competition::new(
            constant::CHAMP_REG,
            constant::CHAMP_REG_URL,
        )));

        assert!(competitions.contains(&entity::Competition::new(
            constant::CHAMP_DEP,
            constant::CHAMP_DEP_URL,
        )));
    }

    /*
    #[actix_web::test]
    async fn test_regions_scrap() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);

        let zones: Vec<entity::Region> = scrap_zones(&competition).await;

        assert_eq!(zones.len(), 18);

        assert!(zones.contains(&entity::Zone::new("AUVERGNE-RHÔNE-ALPES")));
        assert!(zones.contains(&entity::Zone::new("BOURGOGNE-FRANCHE-COMTE")));
        assert!(zones.contains(&entity::Zone::new("BRETAGNE")));
        assert!(zones.contains(&entity::Zone::new("CENTRE-VAL DE LOIRE")));
        assert!(zones.contains(&entity::Zone::new("CORSE")));
        assert!(zones.contains(&entity::Zone::new("GRAND EST")));
        assert!(zones.contains(&entity::Zone::new("GUADELOUPE")));
        assert!(zones.contains(&entity::Zone::new("GUYANE")));
        assert!(zones.contains(&entity::Zone::new("HAUTS-DE-FRANCE")));
        assert!(zones.contains(&entity::Zone::new("ILE-DE-FRANCE")));
        assert!(zones.contains(&entity::Zone::new("LA REUNION")));
        assert!(zones.contains(&entity::Zone::new("MARTINIQUE")));
        assert!(zones.contains(&entity::Zone::new("MAYOTTE")));
        assert!(zones.contains(&entity::Zone::new("NORMANDIE")));
        assert!(zones.contains(&entity::Zone::new("NOUVELLE AQUITAINE")));
        assert!(zones.contains(&entity::Zone::new("OCCITANIE")));
        assert!(zones.contains(&entity::Zone::new("PAYS DE LA LOIRE")));
        assert!(zones.contains(&entity::Zone::new("PROVENCE-ALPES-CÔTE D’AZUR")));
    }
    */

    #[actix_web::test]
    #[should_panic]
    async fn test_regions_scrap_dep_err() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);

        scrap_regions(&competition).await;
    }

    #[actix_web::test]
    #[should_panic]
    async fn test_regions_scrap_nat_err() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_NAT, constant::CHAMP_NAT_URL);

        scrap_regions(&competition).await;
    }

    #[actix_web::test]
    async fn test_regions_scrap() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_REG, constant::CHAMP_REG_URL);

        let regions: Vec<entity::Region> = scrap_regions(&competition).await;

        assert_eq!(regions.len(), 18);

        assert!(regions.contains(&entity::Region::new(
            "AUVERGNE-RHÔNE-ALPES",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIRA"
        )));
        assert!(regions.contains(&entity::Region::new(
            "BOURGOGNE-FRANCHE-COMTE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIBOUR"
        )));
        assert!(regions.contains(&entity::Region::new(
            "BRETAGNE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIBR"
        )));
        assert!(regions.contains(&entity::Region::new(
            "CENTRE-VAL DE LOIRE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LICE"
        )));
        assert!(regions.contains(&entity::Region::new(
            "CORSE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LICO"
        )));
        assert!(regions.contains(&entity::Region::new(
            "GRAND EST",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LILO"
        )));
        assert!(regions.contains(&entity::Region::new(
            "GUADELOUPE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIGU"
        )));
        assert!(regions.contains(&entity::Region::new(
            "GUYANE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIGY"
        )));
        assert!(regions.contains(&entity::Region::new(
            "HAUTS-DE-FRANCE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIFL"
        )));
        assert!(regions.contains(&entity::Region::new(
            "ILE-DE-FRANCE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIIDF"
        )));
        assert!(regions.contains(&entity::Region::new(
            "LA REUNION",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIRE"
        )));
        assert!(regions.contains(&entity::Region::new(
            "MARTINIQUE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIMART"
        )));
        assert!(regions.contains(&entity::Region::new(
            "MAYOTTE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIMY"
        )));
        assert!(regions.contains(&entity::Region::new(
            "NORMANDIE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LILBNV"
        )));
        assert!(regions.contains(&entity::Region::new(
            "NOUVELLE AQUITAINE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIAQ"
        )));
        assert!(regions.contains(&entity::Region::new(
            "OCCITANIE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LILR"
        )));
        assert!(regions.contains(&entity::Region::new(
            "PAYS DE LA LOIRE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIPL"
        )));
        assert!(regions.contains(&entity::Region::new(
            "PROVENCE-ALPES-CÔTE D’AZUR",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LICA"
        )));
    }
}
