/**
 * \brief  Data that are computed by the scraper
 *
 * \file   entity.rs
 * \author Dique Mathieu
 * \data   January, 30 2023
 */
/* -------------------------------------------------------------------------- */
use derive_getters::Getters;

/* -------------------------------------------------------------------------- */
/** \struct Competition
 *  \brief  Entity describing a scraped competition
 */
#[derive(PartialEq, Getters)]
pub struct Competition {
    name: String, /*< The name of the competition */
    url: String,  /*< The url to follow the competition in the web page */
}

impl Competition {
    /**
     * \brief Constructor of the competition entity
     * \param name Name of the competition
     * \param url The url of the web page of this competition
     */
    pub fn new(name: &str, url: &str) -> Competition {
        Competition {
            name: String::from(name),
            url: String::from(url),
        }
    }
}

/* -------------------------------------------------------------------------- */
/** \struct Zone
 *  \brief  Entity describing a scraped zone
 */
#[derive(PartialEq, Getters)]
pub struct Zone {
    name: String, /*< The name of the zone */
}

impl Zone {
    /**
     * \brief Constructor of the Zone entity
     * \param name Name of the zone
     * \param url The matching url of the zone web page
     */
    pub fn new(name: &str) -> Zone {
        Zone {
            name: String::from(name),
        }
    }
}
