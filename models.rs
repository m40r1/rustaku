use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Manga {
    pub cover_src: String,
    pub manga_path: String,
    pub chs: Vec<Chapter>,
    pub page_num: String,
    pub manga_name: String,
}

impl Manga {
    pub fn new(
        cover_src: String,
        manga_path: String,
        chs: Vec<Chapter>,
        page_num: String,
        manga_name: String,
    ) -> Self {
        Self {
            cover_src,
            manga_path,
            chs,
            page_num,
            manga_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Chapter {
    pub link: String,
    pub pages: (usize, Vec<String>),
}

impl Chapter {
    pub fn new(link: String, pages: (usize, Vec<String>)) -> Self {
        Self { link, pages }
    }
}
