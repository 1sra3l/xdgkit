/*!
# Desktop Entry

This is the rustification of the desktop entry specifications!
*/

// desktop_entry.rs
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
use std::fmt;
// could just say use crate::utils::*
use crate::utils::get_language;
use crate::utils::to_bool;

use crate::categories::*;
use self::DesktopEnvironment::*;
use std::slice::Iter;
#[allow(dead_code)]
#[derive(Debug)]
/// Registered DesktopEnvironment Environments
pub enum DesktopEnvironment {
    /// GNOME Desktop
    GNOME,
    /// GNOME Classic Desktop
    GNOMEClassic,
    /// GNOME Flashback Desktop
    GNOMEFlashback,
    /// KDE Desktop
    KDE,
    /// LXDE Desktop
    LXDE,
    /// LXQt Desktop
    LXQt,
    /// MATÉ Desktop
    MATE,
    /// Razor-qt Desktop
    Razor,
    /// ROX Desktop
    ROX,
    /// Trinity Desktop
    TDE,
    /// Unity Shell
    Unity,
    /// XFCE Desktop
    XFCE,
    /// EDE Desktop
    EDE,
    /// Cinnamon Desktop
    Cinnamon,
    /// Pantheon Desktop
    Pantheon,
    /// Legacy menu systems
    Old,
    /// This is for random people making whatever they want... `Unknown` is similar to Desktop Entry's `type`
    Unknown,
}
impl DesktopEnvironment {
    #[allow(dead_code)]
    /// This is to allow people to iterate over the `enum` nicely
    pub fn iter() -> Iter<'static, DesktopEnvironment> {
        static ONLYSHOWIN:[DesktopEnvironment; 17] = [GNOME, GNOMEClassic, GNOMEFlashback, KDE, LXDE, LXQt, MATE, Razor, ROX, TDE, Unity, XFCE, EDE, Cinnamon, Pantheon, Old,  Unknown];
        ONLYSHOWIN.iter()
    }
    #[allow(dead_code)]
    /// Convert the DE to a `String`
    pub fn to_string(&self) ->String {
      match &self {
          GNOME => "GNOME".to_string(),
          GNOMEClassic => "GNOMEClassic".to_string(),
          GNOMEFlashback => "GNOMEFlashback".to_string(),
          KDE => "KDE".to_string(),
          LXDE => "LXDE".to_string(),
          LXQt => "LXQt".to_string(),
          MATE => "MATE".to_string(),
          Razor => "Razor".to_string(),
          ROX => "ROX".to_string(),
          TDE => "TDE".to_string(),
          Unity => "Unity".to_string(),
          XFCE => "XFCE".to_string(),
          EDE => "EDE".to_string(),
          Cinnamon => "Cinnamon".to_string(),
          Pantheon => "Pantheon".to_string(),
          Old => "Old".to_string(),
          Unknown => "Unknown".to_string(),
      }
    }
    #[allow(dead_code)]
    /// Take a String and return a `DesktopEnvironment`
    pub fn from_string(item:String) -> DesktopEnvironment {
        if item == "GNOME" {
            return DesktopEnvironment::GNOME
        } else if item == "GNOMEClassic" {
            return DesktopEnvironment::GNOMEClassic
        } else if item == "GNOMEFlashback" {
            return DesktopEnvironment::GNOMEFlashback
        } else if item == "KDE" {
            return DesktopEnvironment::KDE
        } else if item == "LXDE" {
            return DesktopEnvironment::LXDE
        } else if item == "LXQt" {
            return DesktopEnvironment::LXQt
        } else if item == "MATE" {
            return DesktopEnvironment::MATE
        } else if item == "Razor" {
            return DesktopEnvironment::Razor
        } else if item == "ROX" {
            return DesktopEnvironment::ROX
        } else if item == "TDE" {
            return DesktopEnvironment::TDE
        } else if item == "Unity" {
            return DesktopEnvironment::Unity
        } else if item == "XFCE" {
            return DesktopEnvironment::XFCE
        } else if item == "EDE" {
            return DesktopEnvironment::EDE
        } else if item == "Cinnamon" {
            return DesktopEnvironment::Cinnamon
        } else if item == "Pantheon" {
            return DesktopEnvironment::Pantheon
        } else if item == "Old" {
            return DesktopEnvironment::Old
        }
        DesktopEnvironment::Unknown
    }
}
impl fmt::Display for DesktopEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// This specification defines 3 types of desktop entries:
/// * Application (type 1),
/// * Link (type 2)
/// * Directory (type 3)
///
/// To allow the addition of new types in the future, implementations should ignore desktop entries with an "unknown" type.
#[derive(Debug)]
pub enum DesktopType {
    /// A program
    Application,
    /// A website
    Link,
    /// A file system directory
    Directory,
    /// Ignore 'Unknown' type in your program, it really just means it is anything other than the above known types
    Unknown,
}
impl DesktopType {
    /// Convert a `String` into a `DesktopType`
    pub fn from_string(dt:String)->DesktopType {
        if dt == "Application" {
            return DesktopType::Application
        } else if dt == "Link" {
            return DesktopType::Link
        } else if dt == "Directory" {
            return DesktopType::Directory
        }
        DesktopType::Unknown
    }
    /// Convert a `DesktopType` into a `String`
    pub fn to_string(dt:DesktopType)->String {
    match dt {
        DesktopType::Application => return String::from("Application"),
        DesktopType::Link => return String::from("Link"),
        DesktopType::Directory => return String::from("Directory"),
        _=> return String::from(""),
    }
}
}
impl fmt::Display for DesktopType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// This function converts a Option<String> to a DesktopType
#[allow(dead_code)]
pub fn convert_xdg_type(xdg_type_option:Option<String>)->DesktopType {
    if xdg_type_option.is_some() {
        let dt = xdg_type_option.unwrap();
        if dt == "Application" {
            return DesktopType::Application
        }
        else if dt == "Link" {
            return DesktopType::Link
        }
        else if dt == "Directory" {
            return DesktopType::Directory
        }
    }
    return DesktopType::Unknown
}

/// This function will return a Option<String> from a DesktopType
#[allow(dead_code)]
pub fn string_xdg_type(dt:DesktopType)->Option<String> {
    match dt {
        DesktopType::Application => return Some(String::from("Application")),
        DesktopType::Link => return Some(String::from("Link")),
        DesktopType::Directory => return Some(String::from("Directory")),
        _=> return None,
    }
}

/// # Desktop Entry Files
///
///
/// `Option<String>` will be used for all localeString/String
///
/// `Option<Vec<String>>` for all localeString(s)/String(s)
///
/// `Option<bool>` will be used for all boolean

///
/// `f32` for version number
///
/// `DesktopType` enumerates known "Type" fields and the "Unknown" reserved "Type"
///
/// `type` is a **reserved** word in rust, so it is called 'xdg_type'
///
/// I copied and pasted the website here and made things rustified
///
/// No camelcase inside structs, all lowercase/underscore version of CamelCase
///
/// The text following is from the XDG webpage:
/// 
/// # Recognized desktop entry keys
/// 
/// Keys are either OPTIONAL or REQUIRED. If a key is OPTIONAL it may or may not be present in the file. However, if it isn't, the implementation of the standard should not blow up, it must provide some sane defaults. 
/// 
/// Some keys only make sense in the context when another particular key is also present and set to a specific value. Those keys should not be used if the particular key is not present or not set to the specific value. For example, the Terminal key can only be used when the value of the Type key is Application.
/// 
/// If a REQUIRED key is only valid in the context of another key set to a specific value, then it has to be present only if the other key is set to the specific value. For example, the URL key has to be present when and only when when the value of the Type key is Link.
#[derive(Debug)]
pub struct DesktopEntry {
//type is a reserved word in rust and other languages so we will use xdg_type
    /// This specification defines 3 types of desktop entries: Application (type 1), Link (type 2) and Directory (type 3). To allow the addition of new types in the future, implementations should ignore desktop entries with an unknown type.
    pub xdg_type:DesktopType,
    /// Version of the Desktop Entry Specification that the desktop entry conforms with. Entries that confirm with this version of the specification should use 1.5. Note that the version field is not required to be present.
    pub version:f32,
    /// Specific name of the application, for example "Mozilla".
    pub name:Option<String>,
    /// Generic name of the application, for example "Web Browser".
    pub generic_name:Option<String>,
    /// NoDisplay means "this application exists, but don't display it in the menus". This can be useful to e.g. associate this application with MIME types, so that it gets launched from a file manager (or other apps), without having a menu entry for it (there are tons of good reasons for this, including e.g. the netscape -remote, or kfmclient openURL kind of stuff).
    pub no_display:Option<bool>,
    /// Tooltip for the entry, for example "View sites on the Internet". The value should not be redundant with the values of Name and GenericName.
    pub comment:Option<String>,
    /// Icon to display in file manager, menus, etc. If the name is an absolute path, the given file will be used. If the name is not an absolute path, the algorithm described in the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#icon_lookup) will be used to locate the icon.
    pub icon:Option<String>,
    /// Hidden should have been called Deleted. It means the user deleted (at his level) something that was present (at an upper level, e.g. in the system dirs). It's Stringictly equivalent to the .desktop file not existing at all, as far as that user is concerned. This can also be used to "uninstall" existing files (e.g. due to a renaming) - by letting make install install a file with Hidden=true in it.
    pub hidden:Option<bool>,
    /// A list of Strings identifying the desktop environments that should display/not display a given desktop entry.
/// By default, a desktop file should be shown, unless an OnlyShowIn key is present, in which case, the default is for the file not to be shown.
/// If $XDG_CURRENT_DESKTOP is set then it contains a colon-separated list of Strings. In order, each Stringing is considered. If a matching entry is found in OnlyShowIn then the desktop file is shown. If an entry is found in NotShowIn then the desktop file is not shown. If none of the Strings match then the default action is taken (as above).
/// $XDG_CURRENT_DESKTOP should have been set by the login manager, according to the value of the DesktopNames found in the session file. The entry in the session file has multiple values separated in the usual way: with a semicolon.
/// The same desktop name may not appear in both OnlyShowIn and NotShowIn of a group.
    pub only_show_in:Option<Vec<DesktopEnvironment>>,
    pub not_show_in:Option<Vec<DesktopEnvironment>>,
    /// A boolean value specifying if D-Bus activation is supported for this application. If this key is missing, the default value is false. If the value is true then implementations should ignore the Exec key and send a D-Bus message to launch the application. See [D-Bus Activation](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s08.html) for more information on how this works. Applications should still include Exec= lines in their desktop files for compatibility with implementations that do not understand the DBusActivatable key.
    pub dbus_activatable:Option<bool>,
    /// Path to an executable file on disk used to determine if the program is actually installed. If the path is not an absolute path, the file is looked up in the $PATH environment variable. If the file is not present or if it is not executable, the entry may be ignored (not be used in menus, for example).
    pub try_exec:Option<String>,
    /// Program to execute, possibly with arguments. See the [Exec](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s07.html) key for details on how this key works. The Exec key is required if DBusActivatable is not set to true. Even if DBusActivatable is true, Exec should be specified for compatibility with implementations that do not understand DBusActivatable.
    pub exec:Option<String>,
    /// If entry is of type Application, the working directory to run the program in.
    pub path:Option<String>,
    /// Whether the program runs in a terminal window.
    pub terminal:Option<bool>,
    /// Identifiers for application actions. This can be used to tell the application to make a specific action, different from the default behavior. The [Application actions section](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s11.html) describes how actions work.
    pub actions:Option<Vec<String>>,
    /// The MIME type(s) supported by this application.
    pub mime_type:Option<Vec<String>>,
    /// Categories in which the entry should be shown in a menu for possible values see the [Desktop Menu Specification](http://www.freedesktop.org/Standards/menu-spec).
    pub categories:Option<Vec<Categories>>,
    /// A list of interfaces that this application implements. By default, a desktop file implements no interfaces. See [Interface]([https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s09.html) for more information on how this works.
    pub implements:Option<Vec<String>>,
    /// A list of Strings which may be used in addition to other metadata to describe this entry. This can be useful e.g. to facilitate searching through entries. The values are not meant for display, and should not be redundant with the values of Name or GenericName.
    pub keywords:Option<Vec<String>>,
    /// If true, it is KNOWN that the application will send a "remove" message when started with the DESKTOP_STARTUP_ID environment variable set. If false, it is KNOWN that the application does not work with startup notification at all (does not shown any window, breaks even when using StartupWMClass, etc.). If absent, a reasonable handling is up to implementations (assuming false, using StartupWMClass, etc.). See the [Startup Notification Protocol Specification](http://www.freedesktop.org/Standards/startup-notification-spec) for more details.
    pub startup_notify:Option<bool>,
    /// If specified, it is known that the application will map at least one window with the given Stringing as its WM class or WM name hint see the [Startup Notification Protocol Specification](http://www.freedesktop.org/Standards/startup-notification-spec) for more details.
    pub startup_wm_class:Option<String>,
    /// If entry is Link type, the URL to access.
    pub url:Option<String>,
    /// If true, the application prefers to be run on a more powerful discrete GPU if available, which we describe as “a GPU other than the default one” in this spec to avoid the need to define what a discrete GPU is and in which cases it might be considered more powerful than the default GPU. This key is only a hint and support might not be present depending on the implementation.
    pub prefers_non_default_gpu:Option<bool>,
}

///  Implementations
impl DesktopEntry {
    /// Creates a blank entry with a specified type
    pub fn empty(initial_type:DesktopType)->Self where Self:Sized {
        // nothing to see here
        DesktopEntry {
            xdg_type:initial_type,
            version:1.0,
            name:None,
            generic_name:None,
            no_display:None,
            comment:None,
            icon:None,
            hidden:None,
            only_show_in:None,
            not_show_in:None,
            dbus_activatable:None,
            try_exec:None,
            exec:None,
            path:None,
            terminal:None,
            actions:None,
            mime_type:None,
            categories:None,
            implements:None,
            keywords:None,
            startup_notify:None,
            startup_wm_class:None,
            url:None,
            prefers_non_default_gpu:None,
        }
    }
    /// Creates a 'new' DesktopEntry from reading a file
    /// note: xdgkit does not check the file extention, you *may* want to
    /// This reads not only `.desktop` files but also `.directory`
    pub fn new(file_name:String)->Self where Self:Sized {
        let test_ini = Ini::from_file(&file_name);
        if test_ini.is_err() {
            println!("ERROR!!! {:?} in {}",test_ini,file_name);
            return Self::empty(DesktopType::Application)
        }
        let conf = test_ini.unwrap();

        let section = "Desktop Entry";
        let mut locale:bool = false;
        let lang_var: Option<String> = get_language();
        if lang_var.is_some() {
            locale = true;
        }
        let lang:String = lang_var.unwrap_or(String::from(""));
        //TODO
        let mut lang_two:String = lang.to_owned();
        let mut use_two_lang:bool = false;
        let pos = lang_two.chars().position(|c| c == '_');
        if pos.is_some() {
            use_two_lang = true;
            let posi = pos.unwrap();
            if posi < lang_two.len() {
                // throw away variable for excess of trim
                let _junk = lang_two.split_off(posi);
            }
        }

        //Populate our struct
        let dt:Option<String>                   = conf.get(section, "Type");
        let mut ver:Option<String>                  = conf.get(section, "Version");
        let nd:Option<String>                   = conf.get(section, "NoDisplay");
        let com:Option<String>                  = conf.get(section,"Comment");
        let ic:Option<String>                   = conf.get(section, "Icon");
        let hid:Option<String>                  = conf.get(section, "Hidden");
        let only:Option<Vec<String>>    = conf.get_vec_with_sep(section, "OnlyShowIn",";");
        let not:Option<Vec<String>>     = conf.get_vec_with_sep(section, "NotShowIn",";");
        let dbus:Option<String>                 = conf.get(section, "DBusActivatable");
        let tex:Option<String>                  = conf.get(section, "TryExec");
        let ex:Option<String>                   = conf.get(section, "Exec");
        let pth:Option<String>                  = conf.get(section, "Path");
        let term:Option<String>                 = conf.get(section, "Terminal");
        let act:Option<Vec<String>>     = conf.get_vec_with_sep(section, "Actions",";");
        let mime:Option<Vec<String>>    = conf.get_vec_with_sep(section, "MimeType",";");
        let cat:Option<Vec<String>>     = conf.get_vec_with_sep(section, "Categories",";");
        let imp:Option<Vec<String>>     = conf.get_vec_with_sep(section, "Implements",";");
        let start:Option<String>                = conf.get(section,"StartupNotify");
        let wm:Option<String>                   = conf.get(section,"StartupWMClass");
        let ur:Option<String>                   = conf.get(section,"URL");
        let gpu:Option<String>                  = conf.get(section,"PrefersNonDefaultGPU");
        //LOCALE
        let nom:Option<String>              = conf.get(section,"Name"); //why was is 'ref' again?
        let gen_nom:Option<String>              = conf.get(section, "GenericName");
        let keyw:Option<Vec<String>>    = conf.get_vec_with_sep(section,"Keywords",";");

// these are the 'return' variables for the struct
        let mut local_name:Option<String> = None;
        let mut local_gen:Option<String> = None;
        let mut local_key:Option<Vec<String>> = None;

        // Need to parse for locale specific strings
        if locale {
            
            // NAME
            let item:String = format!("{}{}{}{}","Name","[",lang,"]");
            let attempt_n: Option<String> = conf.get(section,item.as_str());
            if attempt_n.is_none() {
                if use_two_lang {
                    let itm2:String = format!("{}{}{}{}","Name","[",lang_two,"]");
                    let attmpt2: Option<String> = conf.get(section, itm2.as_str());
                    if attmpt2.is_some() {
                        local_name = attmpt2.to_owned();
                    }
                }
            }
            else {
                local_name = attempt_n;
            }
            if local_name.is_none() {
                local_name = nom;
            }

            // GENERIC NAME
            let item1:String = format!("{}{}{}{}","GenericName","[",lang,"]");
            let attempt_gn: Option<String> = conf.get(section,item1.as_str());
            if attempt_gn.is_none() {
                if use_two_lang {
                    let itm:String = format!("{}{}{}{}","GenericName","[",lang_two,"]");
                    let attmpt3: Option<String> = conf.get(section, itm.as_str());
                    if attmpt3.is_some() {
                        local_gen = attmpt3;
                    }
                }
            }
            else {
                local_gen = attempt_gn;
            }
            if local_gen.is_none() {
                local_gen = gen_nom;
            }

            // KEYWORDS
            let itm1:String = format!("{}{}{}{}","Keywords","[",lang,"]");
            let attempt_k: Option<Vec<String>> = conf.get_vec_with_sep(section,itm1.as_str(),";");
            if attempt_k.is_none() {
                if use_two_lang {
                    let it3m:String = format!("{}{}{}{}","Keywords","[",lang_two,"]");
                    let attm3pt: Option<Vec<String>> = conf.get_vec_with_sep(section, it3m.as_str(),";");
                    if attm3pt.is_some() {
                        local_key = attm3pt;
                    }
                }
            }
            else {
                local_key = attempt_k;
            }
            if local_key.is_none() {
                local_key = keyw;
            }
        }
        if ver.is_none() {
            ver = Some(String::from("1.0"));// 1.5/1.0 ???
        }
        // make Category enums
        let mut cats:Vec<Categories> =vec![];
        for item in cat.unwrap() {
          cats.push(Categories::from_string(item));
        }
        //make only show in enums
        let mut onlyshow:Vec<DesktopEnvironment> =vec![];
        if only.is_some() {
            for item in only.unwrap() {
                onlyshow.push(DesktopEnvironment::from_string(item));
            }
        }
         //make not show in enums
        let mut notshow:Vec<DesktopEnvironment> =vec![];
        if not.is_some() {
            for item in not.unwrap() {
                notshow.push(DesktopEnvironment::from_string(item));
            }
        }
        // check emptiness, before trying anying XD
        let mut categories:Option<Vec<Categories>> = None;
        if !cats.is_empty() {
            categories = Some(cats);
        }
        let mut only_show:Option<Vec<DesktopEnvironment>> = None;
        if !onlyshow.is_empty() {
            only_show = Some(onlyshow);
        }
        let mut not_show:Option<Vec<DesktopEnvironment>> = None;
        if !notshow.is_empty() {
            not_show = Some(notshow);
        }
        // blast off!
        DesktopEntry {
            xdg_type:convert_xdg_type(dt),
            version:ver.unwrap().parse::<f32>().ok().unwrap(),
            name:local_name,
            generic_name:local_gen,
            no_display:to_bool(nd),
            comment:com,
            icon:ic,
            hidden:to_bool(hid),
            only_show_in:only_show,
            not_show_in:not_show,
            dbus_activatable:to_bool(dbus),
            try_exec:tex,
            exec:ex,
            path:pth,
            terminal:to_bool(term),
            actions:act,
            mime_type:mime,
            categories:categories,
            implements:imp,
            keywords:local_key,
            startup_notify:to_bool(start),
            startup_wm_class:wm,
            url:ur,
            prefers_non_default_gpu:to_bool(gpu),
        }
    }
}
