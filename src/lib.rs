/*!
# XDGkit
[![Documentation](https://docs.rs/xdgkit/badge.svg)](https://docs.rs/xdgkit)
[![Crates.io](https://img.shields.io/crates/v/xdgkit.svg)](https://crates.io/crates/xdgkit)


The XDG library and command line tool kit!

Much has been copy-pasted from [freedesktop.org](http://freedesktop.org) and rustified as enums and structs with implementations in main for a binary tool to use the specs.

xdgkit follows SemVer

For Example:
 * 0.0.1 was the initial release
 * 0.1.0 saw the addition of `icon-theme` to the CLI subcommands
 * 0.2.0 saw the addition of `desktop-menu` to the CLI subcommands
 * 2.0.0 saw a breaking change: renaming libxdgkit to xdgkit


 ## basedir

This uses `std::env` and returns` Result<String, VarError>` as does `std::env`
This provides all the normal XDG variables, as well as locations for icons, menu/directory files, desktop files, and the autostart directories
The command line parser will automatically check for existing directories.
The functions that have `Vec` like properties (applications directory for example) can all be easily expanded
```
use xdgkit::basedir::*;

// simple use
let app_dirs:Vec<String> = convert_to_vec(applications());
```
This will return an empty vector with an empty string if nothing exists

** this implements `autostart-spec`, `basedir-spec`, and `trash-spec`** from the [XDG specifications](https://specifications.freedesktop.org/)

## desktop-entry

Reads in a desktop file and turns it into a struct which can be accessed for any of the desktop file features you will find in the freedesktop spec.

As a library this returns a struct of mostly `Option<whatever>`

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for. In other words, you will need something like:

## icon_finder

Based off of the psuedo code on freedesktop.org
```
use xdgkit::icon_finder;
use std::path::PathBuf;

let icon_name = "firefox";
// look for the 48px icon
let icon = match icon_finder::find_icon(icon_name.to_string(),48,1){
    Some(icon)=>icon,
    None=>PathBuf::new(),
};
// this will show the path to the icon in the current theme
println!("Firefox icon:{:?}", icon);
```


## icon_theme/icon-theme

Reads an `index.theme` ini-style file and turns it into a struct of `Option<whatever>` which can be accessed for any of the icon theme spec features you will find in the freedesktop spec, or the documentation of this library/program.

As a CLI utility it returns a String printed on a new line (or a blank line if the field is empty that you are looking for.

# WORKS IN PROGRESS

serde does not work for any xml library currently, due to "repeated, out of order elements", which is required for most xml files.  This will not be fixed in `serde-xml-rs`, so who knows if there will be a rust serde xml library that supports serde and xml and rust....
This effects:

 * recently_used
 * desktop_menu/desktop-menu
 * user_places


Until then, I may get around to manually reading it all...

*/

// xdgkit
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

pub mod basedir;
pub mod categories;
pub mod desktop_entry;
pub mod desktop_menu;
pub mod icon_finder;
pub mod icon_theme;
pub mod user_dirs;
pub mod utils;
//pub mod recently_used;

#[cfg(test)]
mod tests {
    use crate::basedir::*;
    use crate::utils::*;
    extern crate tini;
    use tini::Ini;
    //use crate::desktop_entry::*;
    //use crate::categories::*;
    //basedir.rs
    #[test]
    fn basedir_test() {
        if cfg!(target_os = "linux") {
            let res = data_dirs();
            let result2 = res.unwrap().to_owned();
            let vec: Vec<String> = convert_to_vec(data_dirs());
            assert!(!result2.is_empty() && !vec.is_empty());
        }
    }
    // desktop_entry.rs
    const TEST_DESKTOP_FILE: &str = r#"[Desktop Entry]
Name=Test
Name[en_US]=Test
Name[en_GB]=Test
Name[en]=Test
GenericName=Generic Test
Comment=A general test desktop file
Icon=text-editor
Exec=ls -lah
Terminal=true
Type=Application
Categories=Utility;Core;System"#;
    #[test]
    fn desktop_entry_test_good() {
        let conf = Ini::from_string(TEST_DESKTOP_FILE).unwrap();
        // manual test
        let res: Option<String> = conf.get("Desktop Entry", "Categories");
        let res = res.unwrap();
        let split = res.split(";");
        let result: Vec<&str> = split.collect();
        // ALSO FAILS :(
        assert_eq!(result, vec!["Utility", "Core", "System",]);
        //TODO make desktop file to test
    }
    #[test]
    fn desktop_entry_test_bad() {
        //TODO make bad desktop file to test
    }
    // utils.rs
    #[test]
    fn utils_converter() {
        let true_bool: String = "true".to_string();
        assert_eq!(Some(true), to_bool(Some(true_bool)));
        let one_int: String = "1".to_string();
        assert_eq!(Some(1), to_int(Some(one_int)));
        if cfg!(target_os = "linux") {
            let lang = get_language();
            assert_ne!(None, lang);
        }
    }
    #[test]
    fn recent_test() {
        /*use crate::recently_used::Xbel;
        let file = "tests/recently-used.xbel";
        let recent = match Xbel::read(file) {
            Some(recent) => {
                let bookmark = recent.items[0].clone();
                let href = bookmark.href;
                assert_eq!(href.as_str(), "file:///home/israel/programming/rs/vectorview/assets/blank.svg");
                recent
            },
            None => {
                assert_eq!(0,1);
                return
            },
        };*/
    }
    const TEST_MENU_XML:&str = "<!DOCTYPE Menu PUBLIC \"-//freedesktop//DTD Menu 1.0//EN\"           \"http://www.freedesktop.org/standards/menu-spec/menu-1.0.dtd\">
<Menu>
    <Name>Test</Name>
    <Menu>
        <Name>Testing</Name>
    </Menu>
</Menu>";
    //
    #[test]
    fn menu_test() {
        use crate::desktop_menu::DesktopMenu;
        let menu = DesktopMenu::read("/etc/xdg/menus/applications.menu");
        // Sub menu
        //assert_eq!(menu.submenus[0].name, String::from("Testing"));
    }
}
