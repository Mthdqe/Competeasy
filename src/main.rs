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

    fn scrap_matchs(&self, team_name: &str) {
        let match_table = self.document.select(&self.table).nth(MATCH_TABLE).unwrap();
        let match_lines = match_table.select(&self.tr);

        for match_line in match_lines {
            let line_elts: Vec<scraper::ElementRef> = match_line.select(&self.td).collect();

            if line_elts.len() > 1 {
                let date = line_elts[1].inner_html();
                let hour = line_elts[2].inner_html();
                let team_1 = line_elts[3].inner_html();
                let team_2 = line_elts[5].inner_html();
                let place = line_elts[7].inner_html();

                if (team_1 == team_name || team_2 == team_name)
                    && (team_1 != "xxxxx" && team_2 != "xxxxx")
                {
                    println!("[{team_1} VS {team_2}]");
                    println!("Date: {date}");
                    println!("Heure: {hour}");
                    println!("Lieu: {place}");
                    println!("\n");
                }
            }
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
    worker.scrap_matchs("C S M CLAMART 2");
    worker.scrap_matchs("UGS DRAVEIL JUVISY ATHIS-MONS VOLLEY-BALL");
}
