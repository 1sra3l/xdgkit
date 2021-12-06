/*!
# Icon finder

This is the rustification of the example psuedo code for finding icons a.k.a "the algorithm described in the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#icon_lookup)"
*/

// icon_finder.rs
// Rusified in 2021 Copyright Israel Dahl. All rights reserved.
// 
//        /VVVV\
//      /V      V\
//    /V          V\
//   /      0 0     \
//   \|\|\</\/\>/|/|/
//        \_/\_/
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

use crate::icon_theme::*;
use crate::basedir::*;
use std::path::PathBuf;
use std::path::Path;
extern crate tini;
use tini::Ini;

/// Our icon file extentions
const EXTENTIONS:[&str; 3] = [".png", ".svg", ".xpm"];
/// the index file for themes
const INDEX_FILE:&str = "index.theme";

/// A simple structure to hold directory and theme list.
///
/// The reason this is needed is because the **theme** name is **not** the same as the **directory name** `:-P`
#[derive(Debug, Clone)]
pub struct DirList {
    /// the PathBuf
    pub dir:PathBuf,
    /// The name of the theme
    pub theme:String,
}
impl DirList {
    /// easy peasy, make a new `Dirlist`
    pub fn new (dir:String, theme:String) -> DirList {
        DirList {
            dir:PathBuf::from(dir.as_str()),
            theme:theme,
        }
    }
    /// In a nutshell, return `dir.push("/index.theme")`
    pub fn index(&self) -> PathBuf {
        let mut return_value = self.dir.to_owned();
        return_value.push(INDEX_FILE);
        return_value
    }
}
/// Get a `DirList` struct when we look for a theme name in the `Vec<DirList>` we send in, and return our Vector when we are done looking in it.
pub fn find_by_name(name:String, dir_list_vector:Vec<DirList>) -> Option<DirList> {
    if dir_list_vector.is_empty() {
        println!("Empty `DirList` vector sent into `find_by_name()`");
        return None
    }
    for dir in dir_list_vector {
        let theme_name = dir.dir.to_owned();
        for part in theme_name.iter() {
            let check:String = match part.to_str() {
                Some(c)=>String::from(c),
                None => continue,
            };
            if check == name {
                return Some(dir)
            }
        }
    }
    None
}

/// Make the list of `DirList` structs by reading the `$XDG_DATA_DIRS/icons`
pub fn generate_dir_list() -> Vec<DirList>{
    let mut return_value:Vec< DirList> = vec![];
    
    // make our directory of icons
    let directory_vec = icon_dirs_vector();
    for directory in directory_vec {
        //println!("DIRECTORY:{:?}",directory.to_owned());
        let path = Path::new(directory.as_str());
        if path.is_dir() {
            let dir_path = std::fs::read_dir(path);
            if let Ok(dp) = dir_path {
                for entity in dp.flatten() {
                    let return_path = entity.path();
                    let value = return_path.to_str()
                                               .unwrap();
                    let mut index = value.to_owned();
                    index.push_str(INDEX_FILE);
                        if Path::new(index.as_str()).is_file() {
                        let theme = IconTheme::new(index.to_owned());
                        let theme_name = match theme.name {
                            Some(name) => name.to_owned(),
                            None => continue,
                        };
                            //println!("{:?}",index.to_owned());
                        return_value.push(
                            DirList{
                                dir:PathBuf::from(value),
                                theme:theme_name.to_owned(),
                            }
                        );
                    }
                }
            }
        }
    }
    return_value
}


fn look_for_theme_directory(name:String, dir_list_vector:Vec<DirList> )->Option<PathBuf> {
    for dir_list in dir_list_vector {
        let nm = dir_list.theme.to_owned();
        let dir = dir_list.dir.to_owned();
        if nm == name {
            return Some(dir)
        }
    }
    None
}

// should this be doc'd?
fn get_first_index_theme(place:String, dir_list_vector:Vec<DirList>) ->Option<PathBuf> {
    let dirlist = find_by_name(place, dir_list_vector);
    if let Some(d) = dirlist {
        return Some(d.index())
    }
    None
}

/// This function looks in the ini files of KDE and GTK to find the icon theme!
pub fn user_theme(dir_list_vector:Vec<DirList>) -> Option<IconTheme> {
    let theme_dir = icon_dirs();
    if theme_dir.is_ok() {
        // let us look for the user icon theme now that we have directories to look in
        // KDE 
        let mut kdeconf:String = config_home().unwrap();
        kdeconf.push_str("/kdeglobals");
        if Path::new(kdeconf.as_str()).is_file() {
            let test_ini = Ini::from_file(&kdeconf);
            if let Ok(conf) = test_ini {
                let theme:Option<String> = conf.get("Icons","Theme");
                if let Some(themed) = theme {
                    let theme_file:PathBuf = match get_first_index_theme(
                                                      themed,
                                                      dir_list_vector
                                                           .clone()) {
                        Some(theme_file) =>theme_file,
                        None =>PathBuf::new(),
                     };
                     if !theme_file.exists() {
                        return Some(IconTheme::from_pathbuff(theme_file))
                     }
                }
            }
        }
        // GTK
        let mut gtkconf:String = config_home().unwrap();
        gtkconf.push_str("gtk-3.0");
        if Path::new(gtkconf.as_str()).is_file() {
            let test_ini = Ini::from_file(&kdeconf);
            if let Ok(conf) = test_ini {
                let theme:Option<String> = conf.get("Settings","gtk-icon-theme-name");
                if let Some(themed) = theme {
                    let theme_file:PathBuf = match get_first_index_theme(
                                                    themed,
                                                    dir_list_vector.clone()
                                                  ) {
                        Some(theme_file) =>theme_file,
                        None =>PathBuf::new(),
                     };
                     if !theme_file.exists() {
                        return Some(IconTheme::from_pathbuff(theme_file))
                     }
                }
            }
        }
        gtkconf = config_home().unwrap();
        gtkconf.push_str("gtk-4.0");
        if Path::new(gtkconf.as_str()).is_file() {
            let test_ini = Ini::from_file(&kdeconf);
            if let Ok(conf) = test_ini {
                let theme:Option<String> = conf.get("Settings","gtk-icon-theme-name");
                if let Some(themed) = theme {
                    let theme_file:PathBuf = match get_first_index_theme(
                                                        themed,
                                                        dir_list_vector.clone()
                                                  ) {
                        Some(theme_file) =>theme_file,
                        None =>PathBuf::new(),
                     };
                     if !theme_file.exists() {
                        return Some(IconTheme::from_pathbuff(theme_file))
                     }
                }
            }
        }
    }
    // Default to `hicolor` theme if we can't figure it out
    let theme_file:PathBuf = match get_first_index_theme("hicolor".to_string(), dir_list_vector) {
        Some(theme_file) =>theme_file,
        None =>PathBuf::new(),
    };
    if !theme_file.exists() {
        return Some(IconTheme::from_pathbuff(theme_file))
    }
    None
}

/// # Icon Lookup
/// 
/// The icon lookup mechanism has two global settings, the list of base directories and the internal name of the current theme. Given these we need to specify how to look up an icon file from the icon name, the nominal size and the scale.
/// 
/// The lookup is done first in the current theme, and then recursively in each of the current theme's parents, and finally in the default theme called "hicolor" (implementations may add more default themes before "hicolor", but "hicolor" must be last). As soon as there is an icon of any size that matches in a theme, the search is stopped. Even if there may be an icon with a size closer to the correct one in an inherited theme, we don't want to use it. Doing so may generate an inconsistant change in an icon when you change icon sizes (e.g. zoom in).
/// 
/// The lookup inside a theme is done in three phases. First all the directories are scanned for an exact match, e.g. one where the allowed size of the icon files match what was looked up. Then all the directories are scanned for any icon that matches the name. If that fails we finally fall back on unthemed icons. If we fail to find any icon at all it is up to the application to pick a good fallback, as the correct choice depends on the context.
/// 
/// The exact algorithm (in rust) is now here:
// this is our main function used by main.rs to find a single 'named' icon, regardless of type
pub fn find_icon(icon:String, size:i32, scale:i32) -> Option<PathBuf> {

    let dir_list_vector = generate_dir_list();
    let mut theme = user_theme(dir_list_vector.clone());
    if theme.is_none() {
        println!("No user theme");
        theme = Some(IconTheme::empty());
    }
    let theme:IconTheme = theme.unwrap();
    // try with the default theme
    let mut filename:Option<PathBuf> = find_icon_helper(icon.to_owned(), size, scale, theme, dir_list_vector.clone());
    if filename.is_some(){ return filename }

    // check hicolor a.k.a the "default" theme
    let theme_file:PathBuf = match get_first_index_theme("hicolor".to_string(), dir_list_vector.clone()){
        Some(theme_file) => theme_file,
        None => PathBuf::new(),
    };
    let i_theme_file:String = match theme_file.as_path().to_str() {
        Some(t) => String::from(t),
        None => String::from(""),
    };
    let hicolor = IconTheme::new(i_theme_file);
    filename = find_icon_helper(icon.to_owned(), size, scale, hicolor, dir_list_vector);
    if filename.is_some(){ return filename }

    // just find something already....
    lookup_fallback_icon(icon)
}

/// the "helper" function from the freedesktop example psuedo code
pub fn find_icon_helper(icon:String, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<PathBuf> {
    let mut filename:Option<PathBuf> = lookup_icon (icon.to_owned(), size, scale, theme.clone(), dir_list_vector.clone());
    if filename.is_some(){ return filename }

    if let Some(parents) = theme.inherits {
        for parent in parents {
        // make a theme from the 'parent'
            let theme_file:PathBuf = match get_first_index_theme(parent, dir_list_vector.clone()){
                Some(theme_file) => theme_file,
                None => PathBuf::new(),
            };
            let i_theme_file:String = match theme_file.as_path().to_str() {
                Some(t) => String::from(t),
                None => String::from(""),
            };
            let parent_theme = IconTheme::new(i_theme_file.to_owned());
            // boo recursion :(
            filename = find_icon_helper (icon.to_owned(), size, scale, parent_theme.clone(), dir_list_vector.clone());
            if filename.is_some(){ return filename }
        }
    }
    None
}

/// One of the "following helper functions"
pub fn lookup_icon (iconname:String, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<PathBuf> {
    let list = theme.directories.to_owned();
    list.as_ref()?;

    let theme_name = theme.name
                          .unwrap();
    let mut closest_filename:PathBuf = PathBuf::new();
    let theme_subdir_list:Vec<Directory> = list.unwrap();

    // first look check for size matching directories
    for subdir in theme_subdir_list.clone() {
        let subdir_name = subdir.name
                                .to_owned()
                                .unwrap();
        let directory:PathBuf = match look_for_theme_directory(theme_name.to_owned(),
                                 dir_list_vector.clone()) {
            Some(directory) => directory,
            None => PathBuf::new(),
        };
        // empty string from above?
        if !directory.exists() {
            continue 
        }
        for extension in EXTENTIONS.iter() {
            let mut path = directory.to_owned();
            path.push(subdir_name.as_str());
            let mut file_name:String = iconname.to_owned();
            file_name.push_str(extension);
            path.push(file_name.as_str());
            if directory_matches_size(subdir.to_owned(), size, scale)
               && path.as_path().is_file() {
                    return Some(path)
            }
        }
    }
    // ok second try lets look through all of them
    let mut minimal_size:i32 = std::i32::MAX;
    //TODO
    for subdir in theme_subdir_list {
        let subdir_name = subdir.name
                                .to_owned()
                                .unwrap();
        let directory_vec:Vec<PathBuf> = to_pathbuff(icon_dirs_vector());
        for directory in directory_vec {
            for extension in EXTENTIONS.iter() {
                let mut path = directory.to_owned();
                path.push(theme_name.as_str());
                path.push(subdir_name.as_str());
                let mut file_name:String = iconname.to_owned();
                file_name.push_str(extension);
                path.push(file_name.as_str());
                if path.as_path().is_file() && directory_size_distance(subdir.clone(), size, scale) < minimal_size {
                    closest_filename = path.to_owned();
                    minimal_size = directory_size_distance(subdir.clone(), size, scale);
                }
            }
        }
    }
    Some(closest_filename)
}

/// Look in the basic icon directories (like /us/share/pixmaps, /usr/share/icons) for anything that matches the icon name!
pub fn lookup_fallback_icon (iconname:String) ->Option<PathBuf> {
    let directory_vec:Vec<PathBuf> = to_pathbuff(icon_dirs_vector());
    for directory in directory_vec {
        for extension in EXTENTIONS.iter() {
            let mut path = directory.to_owned();
            let mut file_name:String = iconname.to_owned();
            file_name.push_str(extension);
            path.push(file_name.as_str());
            if path.as_path().is_file() {
                return Some(path)
            }
        }
    }
    None
}

/// Check to see if the sub directory size is in range
pub fn directory_matches_size(subdir:Directory, iconsize:i32, iconscale:i32) -> bool {
    let mut scale = 1;
    if subdir.scale.is_some() {
        scale = subdir.scale.unwrap();
    }
    // check scale sent in
    if scale != iconscale {return false}
    // get our variables
    let mut d_type = DirectoryType::Threshold;
    if subdir.xdg_type.is_some() {
        d_type = subdir.xdg_type.unwrap();
    }
    let sze = subdir.size;
    if sze.is_none() {return false}
    let size:i32  = sze.unwrap();
    let mut min_size:i32  = subdir.size.unwrap();
    if subdir.min_size.is_some() {
        min_size = subdir.min_size.unwrap();
    }
    let mut max_size:i32  = subdir.size.unwrap();
    if subdir.max_size.is_some() {
        max_size = subdir.max_size.unwrap();
    }
    let mut threshold:i32 = 2;
    if subdir.threshold.is_some() {
        threshold = subdir.threshold.unwrap();
    }
    
    match d_type {
        DirectoryType::Fixed => {
            return size == iconsize
        },
        DirectoryType::Scalable => {
            if min_size <= iconsize &&
               iconsize <= max_size {
                return true
            }
        },
        DirectoryType::Threshold => {
            if (size - threshold) <= iconsize &&
                iconsize <= (size + threshold) {
                return true
            }
        }
     }
     false
}
/// You guessed it more psuedo code that turned into rust
pub fn directory_size_distance(subdir:Directory, iconsize:i32, iconscale:i32) -> i32{
    // default scale is 1
    let mut scale = 1;
    if subdir.scale.is_some() {
        scale = subdir.scale.unwrap();
    }
    // default type is "Threshold"
    let mut d_type = DirectoryType::Threshold;
    if subdir.xdg_type.is_some() {
        d_type = subdir.xdg_type.unwrap();
    }
    // we need the size for the defaults later
    let size = subdir.size;
    if size.is_none() {return 0}
    let size:i32  = size.unwrap();
    // default to sie
    let mut min_size:i32  = subdir.size.unwrap();
    if subdir.min_size.is_some() {
        min_size = subdir.min_size.unwrap();
    }
    // efault to size
    let mut max_size:i32  = subdir.size.unwrap();
    if subdir.max_size.is_some() {
        max_size = subdir.max_size.unwrap();
    }
    // 2 is the default "threshold"
    let mut threshold:i32 = 2;
    if subdir.threshold.is_some() {
        threshold = subdir.threshold.unwrap();
    }
    
    // now we check our Directory "type"
    // all this math came directly from the page and is edited to be compatible with Rust
    match d_type {
        DirectoryType::Fixed => {
            let num:i32 = size * scale - iconsize * iconscale;
            num.abs()
        },
        DirectoryType::Scalable => {
            if iconsize * iconscale < min_size * scale {
                return min_size * scale - iconsize * iconscale
            }
            if iconsize * iconscale > max_size * scale {
                return iconsize * iconscale - max_size * scale
            }
            0
        },
        DirectoryType::Threshold => {
            if iconsize * iconscale < (size - threshold) * scale {
                return min_size * scale - iconsize * iconscale
            }
            if iconsize*iconsize > (size + threshold) * scale {
                return iconsize * iconsize - max_size * scale
            }
            0
        },
    }
}

/// In some cases you don't always want to fall back to an icon in an inherited theme. For instance, sometimes you look for a set of icons, prefering any of them before using an icon from an inherited theme. To support such operations implementations can contain a function that finds the first of a list of icon names in the inheritance hierarchy. This is that function!
pub fn find_best_icon(icon_list:Vec<String>, size:i32, scale:i32) -> Option<PathBuf> {

    let dir_list_vector = generate_dir_list();
    let theme:IconTheme = match user_theme(dir_list_vector.clone()){
        Some(theme) => theme,
        None => {
            IconTheme::empty();
            return None
        },
    };

    // Get the filename?
    let mut filename:Option<PathBuf> = find_best_icon_helper(
                                          icon_list.clone(),
                                          size,
                                          scale,
                                          theme,
                                          dir_list_vector.clone()
                                      );
    if filename.is_some(){ return filename }

    // check hicolor a.k.a the "default theme"

    let theme_file:PathBuf = match get_first_index_theme("hicolor".to_string(), dir_list_vector.clone()) {
                Some(theme_file) => theme_file,
                None => PathBuf::new(),
            };
    let i_theme_file:String = match theme_file.as_path().to_str() {
        Some(t) => String::from(t),
        None => String::from(""),
    };
    let hicolor = IconTheme::new(i_theme_file);
    filename = find_best_icon_helper(icon_list.clone(), size, scale, hicolor, dir_list_vector);

    if filename.is_some(){ return filename }

    for icon in icon_list {
        filename = lookup_fallback_icon(icon);
        if filename.is_some(){ return filename }
    }
    None
}

/// This can be very useful, for example, when handling mimetype icons, where there are more and less "specific" versions of icons.
pub fn find_best_icon_helper(icon_list:Vec<String>, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<PathBuf> {
    let mut filename = None;
    let list = icon_list;
    let other = list.clone();
    // look through a list of names to find any icon that is similar
    for icon in list {
        filename = lookup_icon(icon, size, scale, theme.clone(), dir_list_vector.clone());

        if filename.is_some(){ return filename }
    }

    // check the inherits
    let inherits = theme.inherits;
    if  let Some(parents) = inherits {
        for parent in parents {
            // make a theme from the 'parent'
            let theme_file:PathBuf = match get_first_index_theme(parent, dir_list_vector.clone()) {
                Some(theme_file) => theme_file,
                None => PathBuf::new(),
            };
            let i_theme_file:String = match theme_file.as_path().to_str() {
                Some(t) => String::from(t),
                None => String::from(""),
            };
            let parent_theme = IconTheme::new(i_theme_file);
            filename = find_best_icon_helper(other.clone(), size, scale, parent_theme.clone(), dir_list_vector.clone());

            if filename.is_some(){ return filename }
        }
    }
    filename
}
