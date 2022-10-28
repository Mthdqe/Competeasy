use scraper::{Html, Selector};
pub mod backend;

const RANKING_TABLE: usize = 2;

fn main() {
    let ffvb_uri: &str = "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2022/2023&codent=LIIDF&poule=RMA";
    let content: String = backend::get_uri_html_content(ffvb_uri);
    // println!("{}", content);
    let html: Html = Html::parse_document(&content);
    let table_selector: Selector = Selector::parse("table").unwrap();
    let tr_selector: Selector = Selector::parse("tr").unwrap();
    let td_selector: Selector = Selector::parse("td").unwrap();

    let ranking_table = html.select(&table_selector).nth(RANKING_TABLE).unwrap();

    // println!("{}", table_content.inner_html());
    let mut ranking_table_lines = ranking_table.select(&tr_selector);
    let _empty_line = ranking_table_lines.next();

    for ranking_table_line in ranking_table_lines {
        let mut team_line = ranking_table_line.select(&td_selector);

        let team_rank: usize = team_line.next().unwrap().inner_html()[..1]
            .parse::<usize>()
            .unwrap();

        let team_name: String = team_line.next().unwrap().inner_html();

        println!("{team_rank} {team_name}");
    }
}
