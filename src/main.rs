mod models;
mod routines;
use std::time::Duration;

use models::Manga;
use reqwest::header::{HeaderMap, HeaderValue};

use routines::*;
use scraper::{Html, Selector};
// use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let headers = gen_img_headers();
    let img_bot = reqwest::Client::builder()
        .timeout(Duration::new(60, 0))
        .default_headers(headers)
        .build()?;
    let txt_bot = reqwest::Client::builder().build()?;

    let counter = 1;
    let page = get_dir_page(&txt_bot, counter).await?;
    let page = Html::parse_document(&page);
    let max_pages = num_of_pages(&page);

    while counter < max_pages {
        let page = get_dir_page(&txt_bot, counter).await?;
        let page = Html::parse_document(&page);
        for link in get_manga_links(&page).await {
            let manga = Manga::new(
                get_manga_cover(&page, &link),
                link.to_string(),
                link.replace("http://m.mangatown.com", ""),
                get_manga_ch(&txt_bot, &link).await?,
            );

            for ch in manga.chs {
                let page = match txt_bot.get(&ch).send().await {
                    Ok(text) => text.text().await?,
                    Err(e) => {
                        eprintln!("getting page for image parsing:{e}");
                        continue;
                    }
                };
                let page = Html::parse_document(&page);
                let selecta = Selector::parse(".ch-select > select:nth-child(5) > option").unwrap();
                //download each image
                let mut counter = 1;
                for pages in page.select(&selecta) {
                    let opt = pages.value().attr("value").unwrap();
                    if opt == ch {
                        let path =
                            format!("{}", ch.as_str().replace("http://m.mangatown.com/", ""),);
                        tokio::fs::create_dir_all(&path).await.unwrap();
                        dwnl_img(&img_bot, &page, &path, counter).await?;
                    } else {
                        counter += 1;
                        let page = match txt_bot.get(opt).send().await {
                            Ok(text) => text.text().await?,
                            Err(e) => {
                                eprintln!("getting page for image parsing:{e}");
                                continue;
                            }
                        };
                        let page = Html::parse_document(&page);
                        let path =
                            format!("{}", ch.as_str().replace("http://m.mangatown.com/", ""),);
                        match tokio::fs::create_dir_all(&path).await {
                            Ok(()) => (),
                            Err(e) => {
                                eprintln!("create dir err:{e}");
                                continue;
                            }
                        };
                        dwnl_img(&img_bot, &page, &path, counter).await?;
                    }
                }
            }
            println!("stole a manga");
        }
    }

    Ok(())
}

fn gen_img_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("image"));

    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9"),
    );

    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
    );

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("image"),
    );

    headers.insert(
        reqwest::header::REFERER,
        HeaderValue::from_static("https://m.mangatown.com/directory"),
    );

    headers
}

// fn gen_dirpage_header() -> HeaderMap {
//     let mut headers = HeaderMap::new();

//     headers.insert(
//         reqwest::header::USER_AGENT,
//          HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36").expect("invalid user _agent  "),
//             );

//     headers.insert(
//         reqwest::header::HOST,
//         HeaderValue::from_str(" m.mangatown.com").expect("invalid  header_value  [host]"),
//     );

//     headers.insert(
//         reqwest::header::CONTENT_TYPE,
//         HeaderValue::from_str("text/html; charset=UTF-8")
//             .expect("invalid  header value [content:type]"),
//     );

//     headers.insert(
//         reqwest::header::ACCEPT,
//         HeaderValue::from_str(
//             "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
//         )
//         .expect("invalid  header_value  [accept]"),
//     );

//     headers.insert(
//         reqwest::header::ACCEPT_ENCODING,
//         HeaderValue::from_str("Accept-Encoding: gzip, deflate, br")
//             .expect("invalid  header_value  [accept-encoding]"),
//     );
//     headers
// }
