/**
 * \file query.rs
 *
 * \brief Describe all the interfaces that are requested to the backend. These
 *        structures are considered as parameters.
 *
 * \author Mathieu Dique
 */
/* -------------------------------------------------------------------------- */
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/* -------------------------------------------------------------------------- */
/** /struct Department
 *  /brief Describe the Region Query Parameter
 */
#[derive(Deserialize, Serialize, PartialEq, Getters)]
pub struct Department {
    url: String,    /*< The url of the departments page */
    region: String, /*< Name of the region of the departments */
}

impl Department {
    /**
     * /brief Constructor of the Department Query Parameter
     * \param url
     * \param region
     */
    pub fn new(url: &str, region: &str) -> Department {
        Department {
            url: String::from(url),
            region: String::from(region),
        }
    }
}

/* -------------------------------------------------------------------------- */

/** \struct Region
 *  \brief Describe the Region Query Parameter
 */
#[derive(Deserialize, Serialize, PartialEq, Getters)]
pub struct Region {
    url: String, /*< The url of the region page */
}

impl Region {
    /**
     * \brief Constructor of the Region Query Parameter
     * \param url
     */
    pub fn new(url: &str) -> Region {
        Region {
            url: String::from(url),
        }
    }
}
