use scraper::{Html, Selector};
use std::ops::Add;

//just the chs{string}
pub async fn get_manga_ch(
    txt_bot: &reqwest::Client,
    link: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let page = txt_bot.get(link).send().await?.text().await?;
    let doc = Html::parse_document(&page);
    let selecta = Selector::parse(".detail-ch-list > li > a:nth-child(1)").unwrap();
    let mut chs: Vec<String> = vec![];
    for element in doc.select(&selecta) {
        let a = element.value().attr("href").unwrap().to_string();

        chs.push(a);
    }
    Ok(chs)
}

//This downloads it
pub async fn dwnl_img(
    img_bot: &reqwest::Client,
    page: &Html,
    path: &str,
    counter: i32,
) -> Result<(), reqwest::Error> {
    let selecta = Selector::parse("#image").unwrap();
    for element in page.select(&selecta) {
        let a = element.value().attr("src").unwrap();
        let a = format!("http:{a}");
        let img = match img_bot.get(&a).send().await {
            Ok(body) => body.bytes().await?,
            Err(e) => {
                eprintln!("get image for download:{e}");
                continue;
            }
        };

        match tokio::fs::write(format!("{path}/{counter}.jpg"), img).await {
            Ok(()) => {
                println!("wrote {counter}.jpg");
                ()
            }
            Err(e) => {
                eprintln!("write image error:{e}");
                continue;
            }
        };
    }

    Ok(())
}

pub async fn get_manga_links(page: &Html) -> Vec<String> {
    let selecta = Selector::parse(".post-list > li > div:nth-child(1) > a:nth-child(2)").unwrap();
    let mut mangas: Vec<String> = Vec::new();
    for element in page.select(&selecta) {
        let a = element.value().attr("href").unwrap();
        mangas.push(a.to_string());
    }
    mangas
}

pub async fn get_dir_page(
    txt_bot: &reqwest::Client,
    counter: u64,
) -> Result<String, reqwest::Error> {
    let page = txt_bot
        .get(format!(
            "https://m.mangatown.com/directory/0-0-0-0-0-0/{}.html",
            counter
        ))
        .send()
        .await?
        .text()
        .await?;
    Ok(page)
}

pub fn num_of_pages(page: &Html) -> u64 {
    let selecta = Selector::parse(".page-nav > select:nth-child(2) > option").unwrap();
    let mut counter = 0;
    for _element in page.select(&selecta) {
        counter = counter.add(1);
    }
    counter
}

//some corner cases the name dont match
// the compare logic
// sinde they are corner cases
// i can look at into later
pub fn get_manga_cover(page: &Html, link: &str) -> String {
    let selecta =
        Selector::parse(".post-list > li > div:nth-child(1) > a:nth-child(1) > img:nth-child(1)")
            .unwrap();
    let mut cover = String::new();
    for element in page.select(&selecta) {
        let name = element.value().attr("alt").unwrap().to_lowercase();
        let name = name.replace(" ", "_");
        let compare = link.replace("http://m.mangatown.com/manga/", "");
        if name.contains(&compare) {
            cover = element.value().attr("src").unwrap().to_string();
        }
    }
    cover
}
