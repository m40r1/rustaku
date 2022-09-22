// u p  se reqwest::header::InvalidHeaderValue;


#[derive(Debug)]
pub struct Manga {
    pub cover_src: String,
    pub manga_path: String,
    pub fs_manga_path: String,
    pub chs: Vec<String>,
}



impl Manga {
    pub fn new(
		cover_src: String,
		manga_path: String,
		fs_manga_path: String,
		chs: Vec<String>,
    	) -> Self {
        Self {
			cover_src,
			manga_path,
			fs_manga_path,
			chs
        }
    }
}
