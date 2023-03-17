pub mod constant;
pub mod entity;

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
        let html: String = client.get(uri).send().await.unwrap().text().await.unwrap();

        Scraper {
            doc: Html::parse_document(&html), /* Parse the html document */
        }
    }

    /**
     * \brief Replace the actual doc of the scraper by a fragment of Html
     * \param fragment The fragment of html to parse that will replace the actual doc
     */
    pub fn new_fragment(&mut self, fragment: &String) {
        self.doc = Html::parse_fragment(fragment);
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
     *  \brief Parse the href value of an ElementRef
     *  \param elt_ref The ElementRef to parse
     *  \return String The href value of the ElementRef
     */
    fn parse_href(elt_ref: &ElementRef) -> String {
        String::from(elt_ref.value().attr("href").unwrap())
    }

    /**
     * \brief  Scrap the sequence and gets the asked value
     * \param  sequence The sequence to parse from the document
     * \param  html_type The type of value we want to scrap
     * \return Vec<String> The list of content parsed from the sequence
     */
    pub fn scrap_value(&self, sequence: &str, html_type: HtmlType) -> Vec<String> {
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

    /**
     * \brief  Scrap the sequence and returns the sraped selector
     * \param  sequence The sequence to parse from the document
     * \return Selector The scraped selector to pursue the scraping
     */
    pub fn scrap(&self, sequence: &str) -> Selector {
        Selector::parse(sequence).unwrap()
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
    /* Instanciate the return vector */
    let mut regions: Vec<entity::Region> = Vec::new();

    /* Instanciate the Scraper */
    let scraper: Scraper = Scraper::new(competition.url()).await;

    /* Scrap the region names */
    let names: Vec<String> = scraper.scrap_value("thead tr td", HtmlType::InnerHtml);

    /* Scrap the region pools url */
    let pools: Vec<String> = scraper.scrap_value("tbody tr td ul li a", HtmlType::Href);

    /* Build the vector of regions */
    for i in 0..std::cmp::min(names.len(), pools.len()) {
        regions.push(entity::Region::new(&names[i], &pools[i]));
    }

    regions
}

/**
 * \brief  Scrap the departments of the given competition in the given region
 * \param  competition The Competition from which scrap the departments
 * \param  region The region from which scrap the departements
 * \return Vec<entity::Department> The list of scraped departments
 */
pub async fn scrap_departments(
    competition: &entity::Competition,
    region: &entity::Region,
) -> Vec<entity::Department> {
    /* Instanciate the vector of departments */
    let mut departs: Vec<entity::Department> = Vec::new();

    /* Instanciate the scraper */
    let mut scraper: Scraper = Scraper::new(competition.url()).await;

    /* Scrap the different region names */
    let region_names: Vec<String> = scraper.scrap_value("thead tr td", HtmlType::InnerHtml);

    /* Find the index of the region in the different tables */
    let region_index: usize = region_names
        .iter()
        .position(|x| x.eq(region.name()))
        .unwrap();

    /* Scrap the content of each region */
    let region_departs: Vec<String> =
        scraper.scrap_value("table tbody tr td ul", HtmlType::InnerHtml);

    /* Update the document to scrap to be the part of the document holding the department names */
    scraper.new_fragment(&region_departs[region_index]);

    /* Scrap the departments names of the region */
    let depart_name: Vec<String> = scraper.scrap_value("li a", HtmlType::InnerHtml);

    /* If there is no department in this region, the department is the region */
    let depart_name = if depart_name.len() == 1 {
        vec![String::from(region.name())]
    } else {
        depart_name
    };

    /* Scrap the departments urls of the region */
    let depart_urls: Vec<String> = scraper.scrap_value("li a", HtmlType::Href);

    /* Build the department vector */
    for i in 0..depart_name.len() {
        departs.push(entity::Department::new(&depart_name[i], &depart_urls[i]));
    }

    departs
}

/**
 * \brief Scrap the different
 */
/* ------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use crate::ffvb_scraper::*;

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

    #[actix_web::test]
    async fn test_departments_scrap_simple() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);
        let region: Vec<entity::Region> = scrap_regions(&competition).await;
        let departs: Vec<entity::Department> = scrap_departments(&competition, &region[2]).await;

        assert_eq!(departs.len(), 4);

        assert!(departs.contains(&entity::Department::new(
            "22 Côtes d'Armor",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTBR22"
        )));

        assert!(departs.contains(&entity::Department::new(
            "29 Finistère",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTBR29"
        )));

        assert!(departs.contains(&entity::Department::new(
            "35 Ille-et-Vilaine",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTBR35"
        )));

        assert!(departs.contains(&entity::Department::new(
            "56 Morbihan",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTBR56"
        )));
    }

    #[actix_web::test]
    async fn test_departments_scrap_moderate() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);
        let region: Vec<entity::Region> = scrap_regions(&competition).await;
        let departs: Vec<entity::Department> = scrap_departments(&competition, &region[0]).await;

        assert_eq!(departs.len(), 8);

        assert!(departs.contains(&entity::Department::new(
            "01 Ain",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTRA01"
        )));
        assert!(departs.contains(&entity::Department::new(
            "07/26 Drôme-Ardèche",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTRA26"
        )));
        assert!(departs.contains(&entity::Department::new(
            "15 Cantal",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTAU15"
        )));
        assert!(departs.contains(&entity::Department::new(
            "38 Isère",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTRA38"
        )));
        assert!(departs.contains(&entity::Department::new(
            "42 Loire",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTRA42"
        )));
        assert!(departs.contains(&entity::Department::new(
            "43 Haute Loire",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTAU43"
        )));
        assert!(departs.contains(&entity::Department::new(
            "63 Puy de Dôme",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTAU63"
        )));

        assert!(departs.contains(&entity::Department::new(
            "69 Rhône Métropole de Lyon",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=PTRA69"
        )));
    }

    #[actix_web::test]
    async fn test_departments_scrap_none() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);
        let region: Vec<entity::Region> = scrap_regions(&competition).await;
        let departs: Vec<entity::Department> = scrap_departments(&competition, &region[4]).await;

        assert!(departs.len() == 0);
    }

    #[actix_web::test]
    async fn test_departments_scrap_hard() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);
        let region: Vec<entity::Region> = scrap_regions(&competition).await;
        let departs: Vec<entity::Department> = scrap_departments(&competition, &region[6]).await;

        assert!(departs.len() == 1);

        println!("{}", departs[0].name());
        println!("{}", departs[0].url());

        assert!(departs.contains(&entity::Department::new(
            "GUADELOUPE",
            "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_home.php?codent=LIGU"
        )));
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
