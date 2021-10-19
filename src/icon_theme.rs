/*!
# Icon Theme

This rustifies the freedesktop specifications for icon themes
*/

// icon_theme.rs
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


extern crate tini;
use tini::Ini;
use std::path::PathBuf;

use crate::utils::to_bool;
use crate::utils::to_int;
//use std::fmt;

/// ## Type
///
/// (a.k.a DirectoryType/xdg_type)
/// 
/// The type of icon sizes for the icons in this directory.
/// Valid types are:
/// * Fixed
/// * Scalable
/// * Threshold
///
/// The type decides what other keys in the section are used. If not specified, the **default is Threshold**.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DirectoryType {
    Fixed,
    Scalable,
    Threshold,
}

/// ## Context
///
///  The Context allows the designer to group icons on a conceptual level. It doesn't act as a namespace in the file system, such that icons can have identical names, but allows implementations to categorize and sort by it, for example.
///
/// These are the available contexts:
/// * Actions. Icons representing actions which the user initiates, such as Save As.
/// * Devices. Icons representing real world devices, such as printers and mice. It's not for file system nodes such as character or block devices.
/// * FileSystems. Icons for objects which are represented as part of the file system. This is for example, the local network, “Home”, and “Desktop” folders.
/// * MimeTypes. Icons representing MIME types.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum IconContext {
    Actions,
    Devices,
    FileSystems,
    MimeTypes,
    /// There is no `default` for Context here, so I'll make an `Unknown`  similar to Desktop Entry's `type`
    Unknown,
}
/// # Overview
///
/// An icon theme is a set of icons that share a common look and feel. The user can then select the icon theme that they want to use, and all apps use icons from the theme. The initial user of icon themes is the icon field of the desktop file specification, but in the future it can have other uses (such as mimetype icons).
///
/// From a programmer perspective an icon theme is just a mapping. Given a set of directories to look for icons in and a theme name it maps from icon name and nominal icon size to an icon filename.
/// ## Definitions
///
/// #### Icon Theme
///
/// An icon theme is a named set of icons. It is used to map from an iconname and size to a file. Themes may inherit from other themes as a way to extend them.
///
/// #### Icon file
///
/// An icon file is an image that can be loaded and used as an icon. The supported image file formats are PNG, XPM and SVG. PNG is the recommended bitmap format, and SVG is for vectorized icons. XPM is supported due to backwards compability reasons, and it is not recommended that new themes use XPM files. Support for SVGs is optional. 
///
/// #### Base Directory
///
/// Icons and themes are searched for in a set of directories, called base directories. The themes are stored in subdirectories of the base directories.
///
/// #### Icon scale
///
/// On very high density (high dpi) screens the UI is often scaled to avoid the UI being so small it is hard to see. In order to support this icons can have a target scale, describing what scale factor they are designed for.
///
/// For instance, an icon with a directory size of 48 but scale 2x would be 96x96 pixels, but designed to have the same level of detail as a 48x48 icon at scale 1x. This can be used on high density displays where a 48x48 icon would be too small (or ugly upscaled) and a normal 96x96 icon would have a lot of detail that is hard to see.
/// ## Directory Layout
///
/// Icons and themes are looked for in a set of directories. By default, apps should look in `$HOME/.icons` (for backwards compatibility), in `$XDG_DATA_DIRS/icons` and in `/usr/share/pixmaps` (in that order). Applications may further add their own icon directories to this list, and users may extend or change the list (in application/desktop specific ways).In each of these directories themes are stored as subdirectories. A theme can be spread across several base directories by having subdirectories of the same name. This way users can extend and override system themes.
///
/// In order to have a place for third party applications to install their icons there should always exist a theme called ["hicolor"](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#ftn.idm44938525353648). The data for the hicolor theme is available for download at: <http://www.freedesktop.org/software/icon-theme/>. Implementations are required to look in the "hicolor" theme if an icon was not found in the current theme.
///
/// Each theme is stored as subdirectories of the base directories. The internal name of the theme is the name of the subdirectory, although the user-visible name as specified by the theme may be different. Hence, theme names are case sensitive, and are limited to ASCII characters. Theme names may also not contain comma or space.
///
/// In at least one of the theme directories there must be a file called index.theme that describes the theme. The first index.theme found while searching the base directories in order is used. This file describes the general attributes of the theme.
///
/// In the theme directory are also a set of subdirectories containing image files. Each directory contains icons designed for a certain nominal icon size and scale, as described by the index.theme file. The subdirectories are allowed to be several levels deep, e.g. the subdirectory "48x48/apps" in the theme "hicolor" would end up at $basedir/hicolor/48x48/apps.
///
/// The image files must be one of the types: PNG, XPM, or SVG, and the extension must be ".png", ".xpm", or ".svg" (lower case). The support for SVG files is optional. Implementations that do not support SVGs should just ignore any ".svg" files. In addition to this there may be an additional file with extra icon-data for each file. It should have the same basename as the image file, with the extension ".icon". e.g. if the icon file is called "mime_source_c.png" the corresponding file would be named "mime_source_c.icon".
/// #File Formats
/// Both the icon theme description file and the icon data files are ini-style text files, as described in the desktop file specification. They don't have any encoding field. Instead, they must **always be stored in UTF-8 encoding**.
/// 
/// The `index.theme` file must start with a section called `[Icon Theme]`, with contents according to the items below. All lists in the ini file, are to be comma-separated.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Directory {
    /// **[REQUIRED BY SPECS]** directory name
    pub name:Option<String>,
    /// Nominal (unscaled) size of the icons in this directory.
    pub size:Option<i32>,
    /// **[REQUIRED BY SPECS]** Target scale of of the icons in this directory. Defaults to the value `1` if not present. Any directory with a scale other than `1` should be listed in the `scaled_directories` list rather than `directories` for backwards compatibility.
    pub scale:Option<i32>,
    /// The context the icon is normally used in.  See: [Context](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#context)
    pub context:Option<IconContext>,
    /// The type of icon sizes for the icons in this directory. Valid types are `Fixed`, `Scalable` and `Threshold`. The type decides what other keys in the section are used. If not specified, the default is `Threshold`.
    pub xdg_type:Option<DirectoryType>,
    /// Specifies the maximum (unscaled) size that the icons in this directory can be scaled to. Defaults to the value of `size` if not present.
    pub max_size:Option<i32>,
    /// Specifies the minimum (unscaled) size that the icons in this directory can be scaled to. Defaults to the value of `size` if not present.
    pub min_size:Option<i32>,
    /// The icons in this directory can be used if the size differ at most this much from the desired (unscaled) size. Defaults to `2` if not present.
    pub threshold:Option<i32>,
}
impl Directory{
    /// Convert an `Option<String>` to a `DirectoryType`
    #[allow(dead_code)]
    pub fn convert_xdg_type(directory_type:Option<String>)->Option<DirectoryType> {
        if directory_type.is_some() {
            let dt = directory_type.unwrap();
            if dt == "Fixed" {
                return Some(DirectoryType::Fixed)
            }
            else if dt == "Scalable" {
                return Some(DirectoryType::Scalable)
            }
        }
        return Some(DirectoryType::Threshold)
    }

    /// This function will return a Some(String) from a DirectoryType **Threshold is returned by default**
    #[allow(dead_code)]
    pub fn string_xdg_type(dt:DirectoryType)->Option<String> {
        match dt {
            DirectoryType::Fixed => return Some(String::from("Fixed")),
            DirectoryType::Scalable => return Some(String::from("Scalable")),
            _ => return Some(String::from("Threshold")),
        }
    }
    /// Convert an `Option<String>` to an `Option<IconContext>`
    #[allow(dead_code)]
    pub fn context(icon_context:Option<String>)->Option<IconContext> {
        if icon_context.is_some() {
            let ic = icon_context.unwrap();
            if ic == "Actions" {
                return Some(IconContext::Actions)
            }
            else if ic == "Devices" {
                return Some(IconContext::Devices)
            }
            else if ic == "FileSystems" {
                return Some(IconContext::FileSystems)
            }
            else if ic == "MimeTypes" {
                return Some(IconContext::MimeTypes)
            }
        }
        // This was added to mimic Desktop Entry behavior
        return Some(IconContext::Unknown)
    }
}
/// Makes an `Option<Vec<Directory>>` from an `Option<Vec<String>>` of directories in the `Directories=` field
#[allow(dead_code)]
pub fn make_directories(dirs:Option<Vec<String>>, file_string:String)->Option<Vec<Directory>> {
    let mut result:Vec<Directory> = Vec::new();
    if dirs.is_none() { return None }
        let test_ini = Ini::from_string(file_string);
        if test_ini.is_err() {
            return None
        }
        let conf = test_ini.unwrap();
        let directories = dirs.unwrap();

        for dir in directories{
            if !dir.is_empty() {
                let section = dir.as_str();
                let mut nom:Option<String> = conf.get(section, "Name");
                if nom.is_none() {
                    nom = Some(String::from(section));
                }
                let sz:Option<String> = conf.get(section, "Size");
                let scl:Option<String> = conf.get(section, "Scale");
                let cntxt:Option<String> = conf.get(section, "Context");
                let x_type:Option<String> = conf.get(section, "Type");
                let max_sz:Option<String> = conf.get(section, "MaxSize");
                let min_sz:Option<String> = conf.get(section, "MinSize");
                let thresh:Option<String> = conf.get(section, "Threshold");
                result.push(
                    Directory {
                        name:nom,
                        size:to_int(sz),
                        scale:to_int(scl),
                        context:Directory::context(cntxt),
                        xdg_type:Directory::convert_xdg_type(x_type),
                        max_size:to_int(max_sz),
                        min_size:to_int(min_sz),
                        threshold:to_int(thresh),
                    }
                );
            }
        }
        Some(result)
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IconTheme {
    /// **[REQUIRED BY SPECS]** short name of the icon theme, used in e.g. lists when selecting themes.
    pub name:Option<String>,
    /// **[REQUIRED BY SPECS]**  longer string describing the theme
    pub comment:Option<String>,
    /// The name of the theme that this theme inherits from. If an icon name is not found in the current theme, it is searched for in the inherited theme (and recursively in all the inherited themes). If no theme is specified implementations are required to add the "hicolor" theme to the inheritance tree. An implementation may optionally add other default themes in between the last specified theme and the hicolor theme.
    pub inherits:Option<Vec<String>>,
    /// **[REQUIRED BY SPECS]** list of subdirectories for this theme. For every subdirectory there must be a section in the `index.theme` file describing that directory.
    pub directories:Option<Vec<Directory>>,
    /// Additional list of subdirectories for this theme, in addition to the ones in Directories. These directories should only be read by implementations supporting scaled directories and was added to keep compatibility with old implementations that don't support these.
    pub scaled_directories:Option<Vec<String>>,
    /// Whether to hide the theme in a theme selection user interface. This is used for things such as fallback-themes that are not supposed to be visible to the user.
    pub hidden:Option<bool>,
    /// The name of an icon that should be used as an example of how this theme looks.
    pub example:Option<String>,
}



///  Implementations
impl IconTheme {
    /// Creates a blank theme, **not according to specs**
    ///
    /// You **must** specify a `name`,`comment`, and `directories` to be inline with the specs
    #[allow(dead_code)]
    pub fn empty()->Self where Self:Sized {
        IconTheme {
            name:None,
            comment:None,
            inherits:None,
            directories:None,
            scaled_directories:None,
            hidden:None,
            example:None,
        }
    }
    pub fn from_pathbuff(file_name:PathBuf)->Self where Self:Sized {
        let filename:String = match file_name.as_path().to_str() {
            Some(t) => String::from(t),
            None => String::from(""),
        };
        Self::new(filename)
    }

    #[allow(dead_code)]
    /// Creates a new struct from a full desktop file path
    pub fn new(file_name:String)->Self where Self:Sized {
        if file_name.is_empty() { return Self::empty() }
        let test_ini = Ini::from_file(&file_name);
        if test_ini.is_err() {
            println!("___________________\nERROR: {:?}\nin file:{}\n___________________",test_ini,file_name);
            return Self::empty()
        }
        let conf = test_ini.unwrap();

        let section = "Icon Theme";
        //Populate our struct
        let nom:Option<String> = conf.get(section, "Name");
        let comm:Option<String> = conf.get(section, "Comment");
        let hid:Option<String> = conf.get(section, "Hidden");
        let ex:Option<String> = conf.get(section, "Example");
        let inh:Option<Vec<String>> = conf.get_vec_with_sep(section, "Inherits",",");
        let dirs:Option<Vec<String>> = conf.get_vec_with_sep(section, "Directories",",");
        let scaled:Option<Vec<String>> = conf.get_vec_with_sep(section, "ScaledDirectories",",");

        IconTheme {
            name:nom,
            comment:comm,
            inherits:inh,
            directories:make_directories(dirs, conf.to_string().to_owned()),
            scaled_directories:scaled,
            hidden:to_bool(hid),
            example:ex,
        }
    }
}
