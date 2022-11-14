pub mod backend;
use backend::Worker;

fn main() {
    let ffvb_uri: &str = "https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2022/2023&codent=LIIDF&poule=RMA";

    let worker = Worker::new(ffvb_uri);

    worker.scrap_ranks();
    worker.scrap_matchs("C S M CLAMART 2");
    worker.scrap_matchs("UGS DRAVEIL JUVISY ATHIS-MONS VOLLEY-BALL");
}
