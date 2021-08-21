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
use std::path::Path;
extern crate tini;
use tini::Ini;

/// Our file extentions
const EXTENTIONS:[&str; 3] = [".png", ".svg", ".xpm"];
/// the index file for themes
const INDEX_FILE:&str = "/index.theme";

/// A simple structure to hold directory and theme list
/// the reason we need this struct is because the theme name is **not** the same as the directory name :-P
#[derive(Debug, Clone)]
pub struct DirList {
    /// the string of the Path
    pub dir:String, //TODO change to path?
    /// The name of the theme
    pub theme:String,
}
impl DirList {
    /// easy peasy, make a new `Dirlist`
    pub fn new (dir:String, theme:String) -> DirList {
        DirList {
            dir:dir,
            theme:theme,
        }
    }
    /// In a nutshell, return `dir.push("/index.theme")`
    pub fn index(&self) -> String {
        let mut return_value = self.dir.to_owned();
        return_value.push_str(INDEX_FILE);
        return_value
    }
}
/// Get a `DirList` struct when we look for a theme name in the `Vec<DirList>` we send in, and return our Vector when we are done looking in it.
pub fn find_by_name(name:String, dir_list_vector:Vec<DirList>) -> Option<DirList> {
    if dir_list_vector.clone().is_empty() {
        println!("Empty `DirList` vector sent into `find_by_name()`");
        return None
    }
    for dir in dir_list_vector.clone() {
        let theme_name = dir.dir.to_owned();
        let position = match theme_name.find(name.as_str()) {
            Some(pos)=>pos,
            None=>theme_name.len(),
        };
        let lenny = theme_name.len() - name.len();
        if position == lenny {
            let dl:DirList = dir;
            return Some(dl)
        }
    }
    None
}

/// Make the list of `DirList` structs by reading the $XDG_DATA_DIRS/icons
pub fn generate_dir_list() -> Vec<DirList>{
    let mut return_value:Vec< DirList> = vec![];
    
    // make our directory of icons
    let mut directory_vec = icon_dirs_vector();
    for directory in directory_vec {
        //println!("DIRECTORY:{:?}",directory.to_owned());
        let path = Path::new(directory.as_str());
        if path.is_dir() {
            let dir_path = std::fs::read_dir(path);
            if dir_path.is_ok() {
                let dp = dir_path.ok()
                                 .unwrap();
                for entity in dp {
                    if entity.is_ok() {
                        let ent = entity.ok()
                                        .unwrap();
                        let return_path = ent.path();
                        let value = return_path.to_str()
                                               .unwrap()
                                               .to_string();
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
                                    dir:value.to_owned(),
                                    theme:theme_name.to_owned(),
                                }
                            );
                        }
                    }
                }
            }
        }
    }
    return_value
}


fn look_for_theme_directory(name:String, dir_list_vector:Vec<DirList> )->Option<String> {
    for dir_list in dir_list_vector.clone() {
        let nm = dir_list.theme.to_owned();
        let dir = dir_list.dir.to_owned();
        if nm == name {
            return Some(dir.to_owned())
        }
    }
    None
}

// should this be doc'd?
fn get_first_index_theme(place:String, dir_list_vector:Vec<DirList>) ->Option<String> {
    let mut index = None;
    let dirlist = find_by_name(place, dir_list_vector.clone());
    if dirlist.is_some() {
        index = Some(dirlist.unwrap()
                            .index());
    }
    //println!("index={:?}",index.clone());
    index
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
            if test_ini.is_ok() {
                let conf = test_ini.unwrap();
                let theme:Option<String> = conf.get("Icons","Theme");
                if theme.is_some() {
                    //println!("KDE: User theme: {:?}", theme.clone().unwrap().to_owned());
                    let theme_file:String = match get_first_index_theme(
                                                      theme.clone()
                                                           .unwrap()
                                                           .to_string(),
                                                      dir_list_vector
                                                           .clone()
                                                  ){
                        Some(theme_file) =>theme_file,
                        None =>String::from(""),
                     };
                     if !theme_file.is_empty() {
                        return Some(IconTheme::new(theme_file))
                     }
                }
            }
        }
        // GTK
        let mut gtkconf:String = config_home().unwrap();
        gtkconf.push_str("gtk-3.0");
        if Path::new(gtkconf.as_str()).is_file() {
            let test_ini = Ini::from_file(&kdeconf);
            if test_ini.is_ok() {
                let conf = test_ini.unwrap();
                let theme:Option<String> = conf.get("Settings","gtk-icon-theme-name");
                if theme.is_some() {
                    //println!("GTK3: User theme: {:?}", theme.clone().unwrap().to_owned());
                    let theme_file:String = match get_first_index_theme(
                                                    theme.unwrap()
                                                    .to_string(),
                                                    dir_list_vector.clone()
                                                  ) {
                        Some(theme_file) =>theme_file,
                        None =>String::from(""),
                     };
                     if !theme_file.is_empty() {
                        return Some(IconTheme::new(theme_file))
                     }
                }
            }
        }
        gtkconf = config_home().unwrap();
        gtkconf.push_str("gtk-4.0");
        if Path::new(gtkconf.as_str()).is_file() {
            let test_ini = Ini::from_file(&kdeconf);
            if test_ini.is_ok() {
                let conf = test_ini.unwrap();
                let theme:Option<String> = conf.get("Settings","gtk-icon-theme-name");
                if theme.is_some() {
                    //println!("GTK4: User theme: {:?}", theme.clone().unwrap().to_owned());
                    let theme_file:String = match get_first_index_theme(
                                                        theme.unwrap()
                                                        .to_string(),
                                                        dir_list_vector.clone()
                                                  ) {
                        Some(theme_file) =>theme_file,
                        None =>String::from(""),
                     };
                     if !theme_file.is_empty() {
                        return Some(IconTheme::new(theme_file))
                     }
                }
            }
        }
    }
    // Default to `hicolor` theme if we can't figure it out
    let theme_file:String = match get_first_index_theme("hicolor".to_string(), dir_list_vector.clone()) {
        Some(theme_file) =>theme_file,
        None =>String::from(""),
    };
    if !theme_file.is_empty() {
        return Some(IconTheme::new(theme_file))
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
pub fn find_icon(icon:String, size:i32, scale:i32) -> Option<String> {

    let dir_list_vector = generate_dir_list();
    let mut theme = user_theme(dir_list_vector.clone());
    if theme.is_none() {
        println!("No user theme");
        theme = Some(IconTheme::empty());
    }
    let theme:IconTheme = theme.unwrap();
    // try with the default theme
    let mut filename:Option<String> = find_icon_helper(icon.to_owned(), size, scale, theme.clone(), dir_list_vector.clone());
    if filename.is_some(){ return filename }

    // check hicolor a.k.a the "default" theme
    let theme_file:String = match get_first_index_theme("hicolor".to_string(), dir_list_vector.clone()){
        Some(theme_file) => theme_file,
        None => String::from(""),
    };
    let hicolor = IconTheme::new(theme_file);
    filename = find_icon_helper(icon.to_owned(), size, scale, hicolor.clone(), dir_list_vector.clone());
    if filename.is_some(){ return filename }

    // just find something already....
    return lookup_fallback_icon(icon.to_owned())
}

/// the "helper" function from the freedesktop example psuedo code
pub fn find_icon_helper(icon:String, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<String> {
    let mut filename:Option<String> = lookup_icon (icon.to_owned(), size, scale, theme.clone(), dir_list_vector.clone());
    if filename.is_some(){ return filename }

    let inherits = theme.inherits.clone();
    if  inherits.is_some() {
        let parents = inherits.unwrap()
                              .to_owned();
        for parent in parents {
        // make a theme from the 'parent'
            let theme_file:String = match get_first_index_theme(parent, dir_list_vector.clone()){
                Some(theme_file) => theme_file,
                None => String::from(""),
            };
            let parent_theme = IconTheme::new(theme_file.to_owned());
            // boo recursion :(
            filename = find_icon_helper (icon.to_owned(), size, scale, parent_theme.clone(), dir_list_vector.clone());
            if filename.is_some(){ return filename }
        }
    }
    return None
}

//With the following helper functions:
pub fn lookup_icon (iconname:String, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<String> {
    let list = theme.directories.to_owned();
    if list.is_none() { return None}

    let theme_name = theme.name
                          .to_owned()
                          .unwrap();
    let mut closest_filename:String = "".to_string();
    let theme_subdir_list:Vec<Directory> = list.unwrap()
                                               .to_owned();

    // first look check for size matching directories
    for subdir in theme_subdir_list.clone() {
        let subdir_name = subdir.name
                                .to_owned()
                                .unwrap();
        let directory:String = match look_for_theme_directory(theme_name.to_owned(),
                                 dir_list_vector.clone()) {
            Some(directory) => directory,
            None => String::from(""),
        };
        // empty string from above?
        if directory.is_empty() { continue }
            for extension in EXTENTIONS.iter() {
                let mut path = directory.to_owned();
                        path.push_str("/");
                        path.push_str(subdir_name.as_str());
                        path.push_str("/");
                        path.push_str(iconname.as_str());
                        path.push_str(extension);
                if directory_matches_size(subdir.to_owned(), size, scale) {
                    
                    if Path::new(path.as_str()).is_file() {
                        return Some(path.to_string())
                }
            }
        }
    }
    // ok second try lets look through all of them
    let mut minimal_size:i32 = std::i32::MAX;
    //TODO
    for subdir in theme_subdir_list.clone() {
        let subdir_name = subdir.name
                                .to_owned()
                                .unwrap();
        let directory_vec:Vec<String> = icon_dirs_vector();
        for directory in directory_vec {
            for extension in EXTENTIONS.iter() {
                let mut path = directory.to_owned();
                    path.push_str("/");
                    path.push_str(theme_name.as_str());
                    path.push_str("/");
                    path.push_str(subdir_name.as_str());
                    path.push_str("/");
                    path.push_str(iconname.as_str());
                    path.push_str(extension);
                if Path::new(path.as_str()).is_file() && directory_size_distance(subdir.clone(), size, scale) < minimal_size {
                    closest_filename = path.to_owned();
                    minimal_size = directory_size_distance(subdir.clone(), size, scale);
                }
            }
        }
    }
    return Some(closest_filename)
}

/// Look in the basic icon directories (like /us/share/pixmaps, /usr/share/icons) for anything that matches the icon name
/// this is the last function needing the directory vector so we can throw it away later
pub fn lookup_fallback_icon (iconname:String) ->Option<String> {
    let directory_vec:Vec<String> = icon_dirs_vector();
    for directory in directory_vec {
        for extension in EXTENTIONS.iter() {
            let mut path = directory.to_owned();
            path.push_str(iconname.as_str());
            path.push_str(extension);
            if Path::new(path.as_str()).is_file() {
                return Some(path)
            }
        }
    }
    return None
}

/// Check to see if the subdir size is in range
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
            if min_size <= iconsize {
               if iconsize <= max_size {return true}
            }
        },
        DirectoryType::Threshold => {
            if (size - threshold) <= iconsize {
                if iconsize <= (size + threshold) {return true}
            }
        }
     }
     return false
}

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
            return num.abs()
        },
        DirectoryType::Scalable => {
            if iconsize * iconscale < min_size * scale {
                return min_size * scale - iconsize * iconscale
            }
            if iconsize * iconscale > max_size * scale {
                return iconsize * iconscale - max_size * scale
            }
            return 0
        },
        DirectoryType::Threshold => {
            if iconsize * iconscale < (size - threshold) * scale {
                return min_size * scale - iconsize * iconscale
            }
            if iconsize*iconsize > (size + threshold) * scale {
                return iconsize * iconsize - max_size * scale
            }
            return 0
        },
    }
}

/// In some cases you don't always want to fall back to an icon in an inherited theme. For instance, sometimes you look for a set of icons, prefering any of them before using an icon from an inherited theme. To support such operations implementations can contain a function that finds the first of a list of icon names in the inheritance hierarchy. I.E. It looks like this:

pub fn find_best_icon(icon_list:Vec<String>, size:i32, scale:i32) -> Option<String> {

    let dir_list_vector = generate_dir_list();
    let mut theme:IconTheme = match user_theme(dir_list_vector.clone()){
        Some(theme) => theme,
        None => {
            IconTheme::empty();
            return None
        },
    };

    // Get the filename?
    let mut filename:Option<String> = find_best_icon_helper(
                                          icon_list.clone(),
                                          size,
                                          scale,
                                          theme.clone(),
                                          dir_list_vector.clone()
                                      );
    if filename.is_some(){ return filename }

    // check hicolor a.k.a the "default theme"
    let theme_dir = icon_dirs();
    let mut theme_file:String = match get_first_index_theme("hicolor".to_string(), dir_list_vector.clone()) {
                Some(theme_file) => theme_file,
                None => String::from(""),
            };
    let hicolor = IconTheme::new(theme_file);
    filename = find_best_icon_helper(icon_list.clone(), size, scale, hicolor.clone(), dir_list_vector.clone());

    if filename.is_some(){ return filename }

    for icon in icon_list {
        filename = lookup_fallback_icon(icon);
        if filename.is_some(){ return filename }
    }
    return None
}

/// This can be very useful, for example, when handling mimetype icons, where there are more and less "specific" versions of icons.
pub fn find_best_icon_helper(icon_list:Vec<String>, size:i32, scale:i32, theme:IconTheme, dir_list_vector:Vec<DirList>) -> Option<String> {
    let mut filename = None;
    let mut list = icon_list.clone().to_owned();
    let mut other = list.clone().to_owned();
    // look through a list of names to find any icon that is similar
    for icon in list {
        filename = lookup_icon(icon, size, scale, theme.clone(), dir_list_vector.clone());

        if filename.is_some(){ return filename }
    }

    // check the inherits
    let inherits = theme.inherits.clone();
    if  inherits.is_some() {
        let parents = inherits.unwrap().clone();
        for parent in parents {
            // make a theme from the 'parent'
            let theme_file:String = match get_first_index_theme(parent, dir_list_vector.clone()) {
                Some(theme_file) => theme_file,
                None => String::from(""),
            };
            let parent_theme = IconTheme::new(theme_file.to_owned());
            filename = find_best_icon_helper(other.clone(), size, scale, parent_theme.clone(), dir_list_vector.clone());

            if filename.is_some(){ return filename }
        }
    }
    filename
}
