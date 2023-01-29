/**
 * \brief  Scraper class to get values from FFVB website
 *
 * \file   scrap.rs
 * \author Dique Mathieu
 * \date   January, 29th 2023
 */
/* ------------------------------------------------------------------------- */
use reqwest::*;
use scraper::*;

/* ------------------------------------------------------------------------- */
const _MAIN_URI: &str = "http://www.ffvb.org/"; /*< Welcom web page of ffvb */

/* ------------------------------------------------------------------------- */
/**
 * \struct Scraper
 * \brief  Scraper structure that is built around the scraping library
 */
pub struct Scraper {
    html: String, /*< The web page content as a string */
    doc: Html,    /*< The document representing the scraped web page */
}

impl Scraper {
    /**
     * \brief Scraper structure constructor
     * \param uri The uri of the web page to scrap
     */
    pub async fn new(uri: &str) -> Scraper {
        /* Create a new scraper */
        let mut new_scraper: Scraper = Scraper {
            html: String::new(),
            doc: Html::new_document(),
        };

        /* Create the client builder, accept invalid_certificates */
        let builder: ClientBuilder = ClientBuilder::new().danger_accept_invalid_certs(true);

        /* Build the client */
        let client: Client = builder.build().unwrap();

        /* Get the html content as a String and block to avoid async */
        new_scraper.html = client.get(uri).send().await.unwrap().text().await.unwrap();
        new_scraper.doc = Html::parse_document(&new_scraper.html);

        new_scraper
    }
}

/* ------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use crate::scrap::*;

    #[actix_web::test]
    async fn test_level_scrap() {
        assert_eq!(true, true);
    }
}
