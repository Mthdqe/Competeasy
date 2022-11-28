use reqwest::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

const RANKING_TABLE: usize = 2;
const MATCH_TABLE: usize = 3;

/**
 * @brief Gets the html content of a web page as a string
 * @param uri The uri from which gets the web page content
 * @return The web page as a text (string)
 */
async fn get_uri_html_content(uri: &str) -> String {
    /* Create the client builder */
    let client_builder: ClientBuilder = ClientBuilder::new().danger_accept_invalid_certs(true);

    /* Build the client */
    let client: Client = client_builder.build().unwrap();

    /* Sends the get request and returns the html content as a String */
    client.get(uri).send().await.unwrap().text().await.unwrap()
}

/**
 * @brief Structure that holds elements to scrap a website
 */
pub struct Worker<'a> {
    tr: Selector,
    td: Selector,
    table: Selector,
    _uri: &'a str,
    document: Html,
}

/**
 * @brief Structure that hold a score
 */
#[derive(Serialize, Deserialize)]
pub struct Score {
    lhs: u8,
    rhs: u8,
}

impl Score {
    pub fn new(lhs: u8, rhs: u8) -> Score {
        Score { lhs, rhs }
    }
}

/**
 * @brief Structure that holds a match
 */
#[derive(Serialize, Deserialize)]
pub struct Match {
    first_team: String,
    second_team: String,
    date: String,
    hour: String,
    place: String,
    match_score: Score,
    sets_score: Vec<Score>,
}

impl<'a> Worker<'a> {
    /**
     * @brief Create a new Worker
     * @param uri The uri to create a worker from
     * @return Worker
     */
    pub async fn new(uri: &str) -> Worker {
        Worker {
            table: Selector::parse("table").unwrap(),
            tr: Selector::parse("tr").unwrap(),
            td: Selector::parse("td").unwrap(),
            _uri: uri,
            document: Html::parse_document(&get_uri_html_content(uri).await[..]),
        }
    }

    /**
     * @brief Scrap the ranks
     */
    pub fn scrap_ranks(&self) {
        let ranking_table = self
            .document
            .select(&self.table)
            .nth(RANKING_TABLE)
            .unwrap();

        // println!("{}", table_content.inner_html());
        let mut ranking_table_lines = ranking_table.select(&self.tr);
        let _empty_line = ranking_table_lines.next();

        for ranking_table_line in ranking_table_lines {
            let mut team_line = ranking_table_line.select(&self.td);

            let team_rank: usize = team_line.next().unwrap().inner_html()[..1]
                .parse::<usize>()
                .unwrap();

            let team_name: String = team_line.next().unwrap().inner_html();

            println!("{team_rank} {team_name}");
        }
    }

    /**
     * @brief Scrap the matchs of a team
     * @param team_name The team to get the match from
     */
    pub fn scrap_matchs(&self, team_name: &str) -> Vec<Match> {
        /* Scrap the match table */
        let match_table = self.document.select(&self.table).nth(MATCH_TABLE).unwrap();

        /* Scrap the matchs lines */
        let match_lines = match_table.select(&self.tr);

        /* Create a vector of match to be returned */
        let mut matchs: Vec<Match> = Vec::new();

        /* Loop other the different matchs */
        for match_line in match_lines {
            /* Scrap the lines */
            let line_elts: Vec<scraper::ElementRef> = match_line.select(&self.td).collect();

            /* Little check to see if the line really describe a match */
            if line_elts.len() > 1 {
                /* Initialize a match */
                let mut m: Match = Match {
                    date: line_elts[1].inner_html(),
                    hour: line_elts[2].inner_html(),
                    first_team: line_elts[3].inner_html(),
                    second_team: line_elts[5].inner_html(),
                    place: String::new(),
                    match_score: Score::new(0, 0),
                    sets_score: vec![],
                };

                /* Try to parse the 6th and 7th element of the line.
                 * If the parsing succeed, it means that the match is already
                 * done. So we set the score of the match. Otherwise we set the
                 * place of the match.
                 */
                let parse_lhs = line_elts[6].inner_html().trim().parse::<u8>();
                let parse_rhs = line_elts[7].inner_html().trim().parse::<u8>();
                match parse_lhs {
                    Ok(lhs) => m.match_score = Score::new(lhs, parse_rhs.unwrap()),
                    Err(_e) => m.place = line_elts[7].inner_html(),
                }

                /* Check if we have to add this match to the match list */
                if (m.first_team == team_name || m.second_team == team_name)
                    && (m.first_team != "xxxxx" && m.second_team != "xxxxx")
                {
                    matchs.push(m);
                }
            }
        }

        matchs
    }
}
