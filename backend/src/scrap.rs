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
