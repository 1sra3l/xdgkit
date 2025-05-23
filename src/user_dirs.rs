/*! # User directories
This reads the `~/config/user-dirs.dirs` file

You can get the location of the file with `filename()`

Or you can build a struct
```
use xdgkit::user_dirs::UserDirs;
let user_dirs = UserDirs::new();
// music directory
let music = user_dirs.music; // usually ~/Music
// documents directory
let documents = user_dirs.documents; // usually ~/Documents
// etc....
```

*/
use crate::basedir;
use std::env::VarError;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// get the `user-dirs.dirs` full path + filename
pub fn filename() -> Result<String, VarError> {
    let conf_dir = match basedir::config_home() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    Ok(format!("{}/user-dirs.dirs", conf_dir.as_str()))
}

/// The file is written by xdg-user-dirs-update
/// it contains the locations of each directory
pub struct UserDirs {
    /// xdg default is: $HOME/Desktop
    pub desktop: String,
    /// xdg default is: $HOME/Downloads
    pub download: String,
    /// xdg default is: $HOME/Templates
    pub template: String,
    /// xdg default is: $HOME/Public
    pub public_share: String,
    /// xdg default is: $HOME/Documents
    pub documents: String,
    /// xdg default is: $HOME/Music
    pub music: String,
    /// xdg default is: $HOME/Pictures
    pub pictures: String,
    /// xdg default is: $HOME/Videos
    pub videos: String,
}
impl UserDirs {
    /// make and empty one
    pub fn empty() -> Self {
        Self {
            desktop: String::new(),
            download: String::new(),
            template: String::new(),
            public_share: String::new(),
            documents: String::new(),
            music: String::new(),
            pictures: String::new(),
            videos: String::new(),
        }
    }
    /// Attempt to create and populate, or send an empty one
    pub fn new() -> Self {
        let mut desktop = String::new();
        let mut download = String::new();
        let mut template = String::new();
        let mut public_share = String::new();
        let mut documents = String::new();
        let mut music = String::new();
        let mut pictures = String::new();
        let mut videos = String::new();
        let file_name = match filename() {
            Ok(f) => f,
            Err(_) => return Self::empty(),
        };
        // try to get home directory
        let home = match basedir::home() {
            Ok(h) => h,
            Err(_) => "$HOME".to_string(),
        };
        // try to open the file
        let file = match File::open(file_name.as_str()) {
            Ok(f) => f,
            Err(_) => return Self::empty(),
        };
        let file_reader = BufReader::new(file);
        for (_line_number, line) in file_reader.lines().enumerate() {
            if line.is_err() {
                continue;
            }
            let mut line = line.unwrap();
            // check to see if the line is a comment
            if let Some(position) = line.find('#') {
                if position == 0 {
                    continue;
                }
            }
            line.retain(|c| c != '"'); //remove_matches('"');
            if let Some((var, dir)) = line.rsplit_once('=') {
                let dir = dir.replace("$HOME", home.as_str());
                if var == "XDG_DESKTOP_DIR" {
                    desktop = dir.to_string();
                } else if var == "XDG_DOWNLOAD_DIR" {
                    download = dir.to_string();
                } else if var == "XDG_TEMPLATES_DIR" {
                    template = dir.to_string();
                } else if var == "XDG_PUBLICSHARE_DIR" {
                    public_share = dir.to_string();
                } else if var == "XDG_DOCUMENTS_DIR" {
                    documents = dir.to_string();
                } else if var == "XDG_MUSIC_DIR" {
                    music = dir.to_string();
                } else if var == "XDG_PICTURES_DIR" {
                    pictures = dir.to_string();
                } else if var == "XDG_VIDEOS_DIR" {
                    videos = dir.to_string();
                }
            }
        }
        Self {
            desktop,
            download,
            template,
            public_share,
            documents,
            music,
            pictures,
            videos,
        }
    }
}
