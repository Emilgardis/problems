//! Finds and returns Languages inside a folder.
//!

use super::Language;
use std::path::Path;
use std::fs;
use std::io;

pub fn find_languages<T: AsRef<Path>>(dir: T, depth: u8) -> Result<Vec<Language>, io::Error> {
    if dir.as_ref().is_dir() || depth != 0 {
        let mut result: Vec<Language> = vec![];
        for entry in try!(fs::read_dir(dir.as_ref())) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                let mut subresult = try!(find_languages(&path, depth-1));
                result.append(&mut subresult);
            } else {
                match path.extension() {
                    None => (),
                    Some(ext) => {
                        if ext == "lang" {
                            result.push(Language::open_lang(&path));
                        }
                    }
                }
            }
        }
        Ok(result)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a directory"))
    }
}
