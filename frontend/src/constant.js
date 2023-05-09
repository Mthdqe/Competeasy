/** \file constant.js
 * 
 *  \brief This file contains the different constant the website requires. These
 *         constants can be related to the backend or simply the flow of the 
 *         logic.
 * 
 * \author Mathieu Dique
 */

/**
 * @brief Enum that describes all the possible competitions and make a match 
 *         with the competition names sent by the backend.
 */
export const Competition = {
    National: "National",          /*!< National competition */
    Regional: "Régional",          /*!< Regional competition */
    Departemental: "Départemental" /*!< Departmental competition */
};

/**
 * @brief Enum that describes the different states the search can be. It always
 *        start by competition which is the first search with have to do.
 */
export const State = {
    Competition: 0,
    Region: 1,
    Department: 2
}
