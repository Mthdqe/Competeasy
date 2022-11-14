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
 * @brief Structure that holds a match
 */
#[derive(Serialize, Deserialize)]
pub struct Match {
    first_team: String,
    second_team: String,
    date: String,
    hour: String,
    place: String,
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
        let match_table = self.document.select(&self.table).nth(MATCH_TABLE).unwrap();
        let match_lines = match_table.select(&self.tr);
        let mut matchs: Vec<Match> = Vec::new();

        for match_line in match_lines {
            let line_elts: Vec<scraper::ElementRef> = match_line.select(&self.td).collect();

            if line_elts.len() > 1 {
                let date = line_elts[1].inner_html();
                let hour = line_elts[2].inner_html();
                let first_team = line_elts[3].inner_html();
                let second_team = line_elts[5].inner_html();
                let place = line_elts[7].inner_html();

                if (first_team == team_name || second_team == team_name)
                    && (first_team != "xxxxx" && second_team != "xxxxx")
                {
                    matchs.push(Match {
                        first_team,
                        second_team,
                        date,
                        hour,
                        place,
                    });
                }
            }
        }

        matchs
    }
}
