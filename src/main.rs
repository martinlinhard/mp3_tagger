use std::{fs, io, path::PathBuf};

use id3::{Tag, Version};
use rayon::prelude::*;
use structopt::StructOpt;

use crate::data::{Input, TaggerError};

mod data;

fn main() {
    let res = parse_input();
    match res {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}

fn parse_input() -> Result<(), TaggerError> {
    let opt = Input::from_args();

    if !opt.music_dir.is_dir() {
        return Err(TaggerError::InvalidInput);
    }

    //convert entries to pathbufs
    let files = fs::read_dir(&opt.music_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()?;

    //update data in parallel!
    files.par_iter().enumerate().for_each(|(index, file)| {
        //make sure it's actually a file :D
        if file.is_file() {
            write_artist(&opt.album, &opt.artist, &file, index as u32).unwrap();
        }
    });

    Ok(())
}

fn write_artist(album: &str, artist: &str, file: &PathBuf, index: u32) -> Result<(), TaggerError> {
    let mut tag = Tag::read_from_path(&file)?;
    tag.set_artist(artist);
    tag.set_album(album);
    tag.set_track(index + 1);
    tag.write_to_path(file, Version::Id3v24)?;
    Ok(())
}
