//Basic data structures and their method implementations

extern crate id3;

use std::fmt;
use std::string::String;
use std::time::Duration;
use std::path::{Path,PathBuf};

macro_rules! safe_expand_tag {
    ($x:expr, $y:expr) => {
        match $x{
            None => $y,
            Some(value) => value,
        };
    }
}

pub struct BasicMetadata{
    //The assumption is that the program is using title+artist as a key in all lookups
    // Therefore it must be correct to begin with and must not be changed at all!
    // title: String,
    // artist: String,
    pub genre: String,
    pub album: String,
    pub track_number: u32,
    pub date: i32,
    pub composer: String,
}

impl BasicMetadata{
    pub fn new() -> BasicMetadata{
        return BasicMetadata{
            genre : String::new(),
            album : String::new(),
            track_number : 0,
            date : 0,
            composer : String::new(),
        }
    }

    pub fn has_some_data(&self) -> bool{
        return !self.genre.is_empty() || !self.album.is_empty() || !self.composer.is_empty() 
            || self.track_number > 0 || self.date > 1800;
    }
}

pub struct SongFile{
    pub metadata: id3::Tag,
    extension: String,
    file_path: PathBuf,
}

impl SongFile{
    pub fn make(file_path : &Path) -> SongFile{
        //Build metadata first
        let tag = id3::Tag::read_from_path(file_path).unwrap();
        let song = SongFile{
            metadata: tag, 
            extension: file_path.extension().unwrap().to_str().unwrap().to_string().to_lowercase(),
            file_path: PathBuf::from(file_path),
            };
        
        return song;
    }

    pub fn save(&mut self){
        self.metadata.write_to_path(self.file_path.as_path());
    }

    pub fn is_metadata_complete(&self) -> bool{
        //The important fields are: Title, Artist, Genre and Year
        let tag = &self.metadata;
        let year = safe_expand_tag!(tag.year(), 0);
        let genre = safe_expand_tag!(tag.genre(), "");
        let album = safe_expand_tag!(tag.album(), "");
        let artist = safe_expand_tag!(tag.artist(), "");
        let title = safe_expand_tag!(tag.title(), "");

        return !artist.is_empty()
            && !title.is_empty()
            && !genre.is_empty() && !(genre.contains("(") || genre.contains(")"))
            && year>1800; //Reasonably enough, I wouldn't catalogue pre-1800 music
    }

    pub fn get_filepath_str(&self) -> Option<&str>{
        return self.file_path.to_str();
    }

    pub fn set_basic_metadata(&mut self, ext_data : BasicMetadata){
        let mut metadata = &mut self.metadata;
        metadata.set_album(ext_data.album);
        let date_timestamp = id3::Timestamp{ year: Some(ext_data.date), 
            month: None, day: None, hour: None, minute: None, second: None };
        metadata.set_date_recorded(date_timestamp.clone());
        metadata.set_date_released(date_timestamp.clone());
        metadata.set_genre(ext_data.genre);
        metadata.set_track(ext_data.track_number);

        //Loose strings here
        let album_artist = String::from(metadata.artist().unwrap());
        metadata.set_album_artist(album_artist);
        //TODO: Add tag to identify its been through MOP
    }
}

impl fmt::Display for SongFile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag = &self.metadata;
        write!(f, "\nTitle: {}\nArtist: {}\nAlbum: {}\nGenre: {}\nYear: {}\nPath:{}", 
            safe_expand_tag!(tag.title(), "N/A"), 
            safe_expand_tag!(tag.artist(), "N/A"), 
            safe_expand_tag!(tag.album(), "N/A"),
            safe_expand_tag!(tag.genre(), "N/A"),
            safe_expand_tag!(tag.year(), 0), 
            self.file_path.display())
    }
}

