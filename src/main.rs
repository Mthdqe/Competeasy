use scraper::{Html, Selector};
pub mod backend;

const RANKING_TABLE: usize = 2;
const MATCH_TABLE: usize = 3;

struct Worker<'a> {
    tr: Selector,
    td: Selector,
    table: Selector,
    uri: &'a str,
    document: Html,
}

impl<'a> Worker<'a> {
    fn new(uri: &str) -> Worker {
        Worker {
            table: Selector::parse("table").unwrap(),
            tr: Selector::parse("tr").unwrap(),
            td: Selector::parse("td").unwrap(),
            uri,
            document: Html::parse_document(&backend::get_uri_html_content(uri)[..]),
        }
    }

    fn scrap_ranks(&self) {
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
}

fn main() {
    let ffvb_uri: &str = "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2022/2023&codent=LIIDF&poule=RMA";
    /*
    let selectors: Selectors =
    let content: String = backend::get_uri_html_content(ffvb_uri);
    let html: Html = Html::parse_document(&content);
    */
    let worker = Worker::new(ffvb_uri);
    worker.scrap_ranks();
}
