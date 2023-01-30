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
     * \brief  Scrap the sequence and gets the inner html content
     * \param  sequence The sequence to parse from the document
     * \return Vec<String> The list of content parsed from the sequence
     */
    pub fn scrap_sequence(&self, sequence: &str) -> Vec<String> {
        /* Create the selector of the given sequence */
        let selector: Selector = Selector::parse(sequence).unwrap();

        /* Parse the sequence from the document */
        self.doc.select(&selector).map(|x| x.inner_html()).collect()
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
 * \brief Scrap the zones of the given Competition
 * \param competition The competition to get the zones from
 * \return Vec<entity::Zone> The list of the zones of the matching competition
 */
pub async fn scrap_zones(competition: &entity::Competition) -> Vec<entity::Zone> {
    /* Error management */
    if competition.name() == constant::CHAMP_NAT {
        panic!();
    }

    /* Instanciate the Scraper */
    let scraper: Scraper = Scraper::new(competition.url()).await;

    /* Scrap the zone names */
    let zone_names: Vec<String> = scraper.scrap_sequence("thead>tr>td");

    /* Create zone entities from the given names */
    zone_names.iter().map(|x| entity::Zone::new(&x)).collect()
}

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

    #[actix_web::test]
    async fn test_zones_scrap_dep() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);

        let zones: Vec<entity::Zone> = scrap_zones(&competition).await;

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

    #[actix_web::test]
    async fn test_zones_scrap_reg() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_REG, constant::CHAMP_REG_URL);

        let zones: Vec<entity::Zone> = scrap_zones(&competition).await;

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

    #[actix_web::test]
    #[should_panic]
    async fn test_zones_scrap_nat() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_NAT, constant::CHAMP_NAT_URL);

        let zones: Vec<entity::Zone> = scrap_zones(&competition).await;
    }
}
