/** \file requests.js
 * 
 *  \brief This file implements the different request we cam make to the backend
 *         to get the different informations we require from the ffvb website
 * 
 * \author Mathieu Dique
 */

import { Competition } from "./constant";

/* -------------------------------------------------------------------------- */
/**
 * @brief Makes a request to the COMPETITIONS endpoint
 * @return A list of the different available competitions 
 */
export async function get_competitions() {
    let uri = "http://localhost:8000/competitions";

    let response = await fetch(uri);
    let competitions = await response.json();

    return competitions;
}

/* -------------------------------------------------------------------------- */
/**
 * @brief Makes a request to the REGIONS endpoint
 * @param url The url of the region page
 * @returns A list of the regions in the given url page
 */
export async function get_regions(url) {
    let uri = "http://localhost:8000/regions?url=" + encodeURI(url);

    let response = await fetch(uri);
    let regions = await response.json();

    return regions;
}

/* -------------------------------------------------------------------------- */
/**
 * @brief Makes a request to the DEPARTMENTS endpoint
 * @param url The url of the department page
 * @param region The region of the departments we want to get
 * @returns A list of the departments in the given region
 */
export async function get_departments(url, region) {
    let uri = "http://localhost:8000/departments?url=" +
        encodeURIComponent(url) +
        "&region=" + region;

    console.log(uri);

    let response = await fetch(uri);
    let departments = await response.json();

    return departments;
}
