

use serde;
use serde_yaml;
use std::path::Path;
use std::fs::OpenOptions;

use super::histogram::Histogram;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub language: String,
    pub histogram: Histogram<String>,
}

impl Language {
    // TODO: Implement n-grams, ie freq of "aa", "ab", .."zz", see
    // http://stackoverflow.com/a/15711310/4284367

    pub fn open_lang<T: AsRef<Path>>(file_path: T) -> Language {
        if let Ok(mut file) = OpenOptions::new()
             .read(true).open(file_path.as_ref()) {                   
            if let Ok(data) = serde_yaml::from_reader(&mut file) {
                data
            } else {
                panic!("Couldn't parse file")
            }
        } else {
            unreachable!()
        }
    }

    pub fn write_lang<T: AsRef<Path>>(&self, file_path: T) {
         if let Ok(mut file) = OpenOptions::new()
             .create(true)
             .write(true)
             .read(true)
             .open(file_path.as_ref()) {
                serde_yaml::to_writer(&mut file, &self);
             } else {
                panic!("File probably already exists!")
             }

    }
}
