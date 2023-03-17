/* ------------------------------------------------------------------------- */
use crate::scrap::*;
use crate::utils::*;

/* ------------------------------------------------------------------------- */
/**
 * \brief Scrap the different competitions, i.e give a list of the available
 *        competitions to check
 * \return Vec<entity::Competition> The list of competitions
 */
pub fn competitions() -> Vec<entity::Competition> {
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
pub async fn regions(url: &str) -> Vec<entity::Region> {
    /* Instanciate the return vector */
    let mut regions: Vec<entity::Region> = Vec::new();

    /* Instanciate the Scraper */
    let scraper: Scraper = Scraper::new(&url).await;

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

/* ------------------------------------------------------------------------- */
/**
 * \brief  Scrap the departments of the given competition in the given region
 * \param  competition The Competition from which scrap the departments
 * \param  region The region from which scrap the departements
 * \return Vec<entity::Department> The list of scraped departments
 */
pub async fn departments(url: &str, region: &str) -> Vec<entity::Department> {
    /* Instanciate the vector of departments */
    let mut departs: Vec<entity::Department> = Vec::new();

    /* Instanciate the scraper */
    let mut scraper: Scraper = Scraper::new(url).await;

    /* Scrap the different region names */
    let region_names: Vec<String> = scraper.scrap_value("thead tr td", HtmlType::InnerHtml);

    /* Find the index of the region in the different tables */
    let region_index: usize = region_names.iter().position(|x| x.eq(region)).unwrap();

    /* Scrap the content of each region */
    let region_departs: Vec<String> =
        scraper.scrap_value("table tbody tr td ul", HtmlType::InnerHtml);

    /* Update the document to scrap to be the part of the document holding the department names */
    scraper.new_fragment(&region_departs[region_index]);

    /* Scrap the departments names of the region */
    let depart_name: Vec<String> = scraper.scrap_value("li a", HtmlType::InnerHtml);

    /* If there is no department in this region, the department is the region */
    let depart_name = if depart_name.len() == 1 {
        vec![String::from(region)]
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

/* ------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::*;

    #[test]
    fn test_competitions_scrap() {
        let competitions: Vec<entity::Competition> = parse::competitions();
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
        let region: Vec<entity::Region> = parse::regions(competition.url()).await;
        let departs: Vec<entity::Department> =
            parse::departments(competition.url(), region[2].name()).await;

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
        let region: Vec<entity::Region> = parse::regions(competition.url()).await;
        let departs: Vec<entity::Department> =
            parse::departments(competition.url(), region[0].name()).await;

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
        let region: Vec<entity::Region> = parse::regions(competition.url()).await;
        let departs: Vec<entity::Department> =
            parse::departments(competition.url(), region[4].name()).await;

        assert!(departs.len() == 0);
    }

    #[actix_web::test]
    async fn test_departments_scrap_hard() {
        let competition: entity::Competition =
            entity::Competition::new(constant::CHAMP_DEP, constant::CHAMP_DEP_URL);
        let region: Vec<entity::Region> = parse::regions(competition.url()).await;
        let departs: Vec<entity::Department> =
            parse::departments(competition.url(), region[6].name()).await;

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

        let regions: Vec<entity::Region> = parse::regions(competition.url()).await;

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
