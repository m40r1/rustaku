use crate::models::{Chapter, Manga};
use log::error;
use scraper::{Html, Selector};

//just the chs{string}
pub fn get_ch_links(
    txt_bot: &reqwest::blocking::Client,
    link: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let page = txt_bot.get(link).send()?.text()?;
    let doc = Html::parse_document(&page);
    let selecta = Selector::parse(".detail-ch-list > li > a:nth-child(1)").unwrap();
    Ok(doc
        .select(&selecta)
        .map(|href| -> String { href.value().attr("href").unwrap().to_string() })
        .collect::<Vec<String>>())
}

fn get_ch_pages(page: &Html) -> (usize, Vec<String>) {
    let selecta = Selector::parse(".ch-select > select:nth-child(5) > option").unwrap();
    let pages: Vec<String> = page
        .select(&selecta)
        .map(|pages| pages.value().attr("value").unwrap().to_string())
        .collect();

    (pages.len(), pages)
}

pub fn get_manga_links(page: &Html) -> Vec<String> {
    let selecta = Selector::parse(".post-list > li > div:nth-child(1) > a:nth-child(2)").unwrap();
    page.select(&selecta)
        .map(|link| link.value().attr("href").unwrap().to_string())
        .collect()
}

// Gives a string page
// you parse it to Html
// all the other fns use it
pub fn get_dir_page(txt_bot: &reqwest::blocking::Client) -> Result<String, reqwest::Error> {
    let page = txt_bot
        .get(format!(
            "https://m.mangatown.com/directory/0-0-0-0-0-0/1.html",
        ))
        .send()?
        .text()?;
    Ok(page)
}

pub fn parse_manga(
    links: Vec<String>,
    page: &Html,
    txt_bot: &reqwest::blocking::Client,
    db: &mongodb::sync::Database,
    location: &String,
) -> Result<(), reqwest::Error> {
    links
        .iter()
        .map(|link| -> Result<(), reqwest::Error> {
            let manga = Manga::new(
                get_manga_cover(&page, &link),
                link.to_string(),
                get_ch_links(&txt_bot, link.as_str())?
                    .iter()
                    .map(|link| -> Chapter {
                        let page = txt_bot.get(link).send().unwrap().text().unwrap();
                        let page = Html::parse_document(&page);
                        Chapter::new(link.to_string(), get_ch_pages(&page))
                    })
                    .collect(),
                location.to_string(),
                get_manga_name(link.to_string()),
            );

            match db.collection::<Manga>("tests").insert_one(manga, None) {
                Ok(_i) => (),
                Err(e) => error!("failed  to  insert  manga  with  err:{e}"),
            };
            Ok(())
        })
        .count();
    Ok(())
}

fn get_manga_name(link: String) -> String {
    let link = link.replace("http://m.mangatown.com/manga/", "");
    let link = link.replace("_", " ");
    link
}
pub fn all_page_links(page: &Html) -> Vec<String> {
    let selecta = Selector::parse(".page-nav > select:nth-child(2) > option").unwrap();
    page.select(&selecta)
        .map(|pages| pages.value().attr("value").unwrap().to_string())
        .collect()
}

//some corner cases the name dont match
// the compare logic
// sinde they are corner cases
// i can look at into later
fn get_manga_cover(page: &Html, link: &str) -> String {
    let selecta =
        Selector::parse(".post-list > li > div:nth-child(1) > a:nth-child(1) > img:nth-child(1)")
            .unwrap();
    page.select(&selecta)
        .map(|src| {
            //TODO manga_name type
            // do a parse
            let link = link.replace("http://m.mangatown.com/manga/", "");
            let link = link.replace("_", " ");
            if link.contains(&src.value().attr("alt").unwrap().to_lowercase()) {
                let link = src.value().attr("src").unwrap().to_string();
                return link;
            } else if link.contains(&src.value().attr("onerror").unwrap().to_lowercase()) {
                let link = src.value().attr("onerror").unwrap().to_lowercase();
                // debug!("what i got onerror {link}");
                return link;
            } else {
                "".to_string()
            }
        })
        .collect()
}
