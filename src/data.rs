use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Music Tagger", about = "A simple program to tag your music!")]
pub struct Input {
    #[structopt(short = "d", long = "dir", parse(from_os_str))]
    pub music_dir: PathBuf,
    #[structopt(short = "a", long = "artist")]
    pub artist: String,
    #[structopt(short = "A", long = "album")]
    pub album: String,
}

#[derive(Debug)]
pub enum TaggerError {
    InvalidInput,
    IOError,
}

impl Error for TaggerError {}

impl fmt::Display for TaggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaggerError::InvalidInput => write!(f, "Invalid directory!"),
            TaggerError::IOError => write!(f, "An IO-Error occured!"),
        }
    }
}

impl From<io::Error> for TaggerError {
    fn from(_error: io::Error) -> Self {
        TaggerError::IOError
    }
}

impl From<id3::Error> for TaggerError {
    fn from(_error: id3::Error) -> Self {
        TaggerError::IOError
    }
}
