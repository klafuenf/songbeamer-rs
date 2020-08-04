use std::fs::File;

pub struct Song {
    properties: Properties,
    text: Text,
}

pub struct Properties {
    lang_count: i32,
    title: String,
    version: i32,
    editor: Option<String>,
    author: Option<String>,
    ccli: /* CCLI */ Option<i32>,
    copyright: /* (c) */ Option<String>,
    verse_order: Option<String>,

    add_copyright_info: Option<String>,
    background_image: Option<String>,
    bible: Option<String>,
    categories: Option<String>,
    chords: Option<String>,
    church_song_id: Option<i32>,
    comment: Option<String>,
    comments: Option<String>,
    font: Option<String>,
    font_size: Option<i32>,
    key: Option<String>,
    keywords: Option<String>,
    lang: Option<String>,
    nat_copyright: Option<String>,
    o_title: Option<String>,
    quick_find: Option<String>,
    rights: Option<String>,
    songbook: Option<String>,
    tempo: Option<i32>,
    text_align: Option<String>,
    title_align: Option<String>,
    translation: Option<String>,
}

pub struct Text {
    parts: Vec<Part>
}

pub struct Part {
    typ: Option<String>,
    text: Vec<String>,
}

pub fn parse(r: Read) -> Song {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
