mod models;
mod routines;
use env_logger::Builder;
use log::{debug, error, info};
use mongodb::{
    options::ClientOptions,
    sync::{Client, Database},
};
use rayon::prelude::*;
use routines::*;
use scraper::Html;

fn main() -> Result<(), reqwest::Error> {
    Builder::new()
        .filter_level(log::LevelFilter::Error)
        .format_timestamp_secs()
        .init();

    rayon::ThreadPoolBuilder::new()
        .num_threads(25)
        .build_global()
        .unwrap();

    let txt_bot = reqwest::blocking::Client::builder().build()?;
    let db = match gen_db() {
        Ok(db) => {
            info!("Got db");
            db
        }
        Err(e) => {
            error!("Db error:{e}");
            panic!()
        }
    };

    let page = get_dir_page(&txt_bot)?;
    debug!("entry point/dir_page.01");

    let page = Html::parse_document(&page);
    let pages = all_page_links(&page);
    debug!("got all page links");

    pages
        .par_iter()
        .map(|pag| -> Result<(), reqwest::Error> {
            info!("getting mangas in  {pag}");

            //TODO
            let page = txt_bot.get(pag).send()?.text()?;
            debug!("made get for new  pag");

            let page = Html::parse_document(&page);
            let link = get_manga_links(&page);
            debug!("got mangas links");
            parse_manga(link, &page, &db, &pag)?;
            info!("got all mangas in the page");

            Ok(())
        })
        .count();
    Ok(())
}

fn gen_db() -> Result<Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
    client_options.app_name = Some("rustaku".to_string());
    let client = Client::with_options(client_options)?;

    let db = client.database("mangas");
    Ok(db)
}
