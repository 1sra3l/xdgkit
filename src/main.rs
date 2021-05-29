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

mod desktop_entry;
mod basedir;
mod icon_theme;
mod desktop_menu;
mod utils;
mod categories;

extern crate clap;
use clap::{App, load_yaml};

/// Our main binary function simply uses clap to read the arguments and pull up the right structs/functions
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // basedir ARGS
    if let Some(matches) = matches.subcommand_matches("basedir") {
        if matches.is_present("Home") {
            println!("{}",basedir::home().unwrap_or("".to_string()));
        }
        if matches.is_present("Menus") {
            println!("{}",basedir::menu().unwrap_or("".to_string()));
        }
        if matches.is_present("MenusMerged") {
            println!("{}",basedir::menu_merged().unwrap_or("".to_string()));
        }
        if matches.is_present("Applications") {
            println!("{}",basedir::applications().unwrap_or("".to_string()));
        }
        if matches.is_present("DesktopDirectories") {
            println!("{}",basedir::desktop_directories().unwrap_or("".to_string()));
        }
        if matches.is_present("Autostart") {
            println!("{}",basedir::autostart().unwrap_or("".to_string()));
        }
        if matches.is_present("Icon") {
            println!("{}",basedir::icon_dirs().unwrap_or("".to_string()));
        }
        if matches.is_present("Trash") {
            println!("{}",basedir::trash().unwrap_or("".to_string()));
        }
        if matches.is_present("DataHome") {
            println!("{}",basedir::data_home().unwrap_or("".to_string()));
        }
        if matches.is_present("CacheHome") {
            println!("{}",basedir::cache_home().unwrap_or("".to_string()));
        }
        if matches.is_present("ConfigHome") {
            println!("{}",basedir::config_home().unwrap_or("".to_string()));
        }
        if matches.is_present("Data") {
            println!("{}",basedir::data_dirs().unwrap_or("".to_string()));
        }
        if matches.is_present("Config") {
            println!("{}",basedir::config_dirs().unwrap_or("".to_string()));
        }
}

// desktop-entry ARGS
    if let Some(matches) = matches.subcommand_matches("desktop-entry") {
    // Read the desktop file
        let filename = matches.value_of("FILE").unwrap();
        let desktop_file = desktop_entry::DesktopEntry::new(filename.to_string());
        //NOT OPTIONAL
        if matches.is_present("Type") {
            println!("{:?}",desktop_file.xdg_type.to_string());
        }
        //NOT OPTIONAL
        if matches.is_present("Version") {
            println!("{:?}",desktop_file.version);
        }
        //TODO make error messages for each struct that has 'required' things
        if matches.is_present("Name") {
            println!("{:?}",desktop_file.name.unwrap_or("".to_string()));
        }
        if matches.is_present("GenericName") {
            println!("{:?}",desktop_file.generic_name.unwrap_or("".to_string()));
        }
        if matches.is_present("NoDisplay") {
            println!("{:?}",desktop_file.no_display.unwrap_or_default());
        }
        if matches.is_present("Comment") {
            println!("{:?}",desktop_file.comment.unwrap_or("".to_string()));
        }
        if matches.is_present("Icon") {
            println!("{:?}",desktop_file.icon.unwrap_or("".to_string()));
        }
        if matches.is_present("Hidden") {
            println!("{:?}",desktop_file.hidden.unwrap_or_default());
        }
        if matches.is_present("OnlyShowIn") {
            if desktop_file.only_show_in.is_some() {
                for item in desktop_file.only_show_in.unwrap_or_default() {
                    print!("{:?};", item.to_string());
                }
            }
            print!("\n");
        }
        if matches.is_present("NotShowIn") {
            if desktop_file.not_show_in.is_some() {
                for item in desktop_file.not_show_in.unwrap_or_default() {
                    print!("{:?};", item.to_string());
                }
            }
            print!("\n");
        }
        if matches.is_present("DBusActivatable") {
            println!("{:?}",desktop_file.dbus_activatable.unwrap_or_default());
        }
        if matches.is_present("TryExec") {
            println!("{:?}",desktop_file.try_exec.unwrap_or("".to_string()));
        }
        if matches.is_present("Exec") {
            println!("{:?}",desktop_file.exec.unwrap_or("".to_string()));
        }
        if matches.is_present("Path") {
            println!("{:?}",desktop_file.path.unwrap_or("".to_string()));
        }
        if matches.is_present("Terminal") {
            println!("{:?}",desktop_file.terminal.unwrap_or_default());
        }
        if matches.is_present("Actions") {
            println!("{:?}",desktop_file.actions.unwrap_or_default());
        }
        if matches.is_present("MimeType") {
            println!("{:?}",desktop_file.mime_type.unwrap_or_default());
        }
        if matches.is_present("Categories") {
            println!("{:?}",desktop_file.categories.unwrap_or_default());
        }
        if matches.is_present("Implements") {
            println!("{:?}",desktop_file.implements.unwrap_or_default());
        }
        if matches.is_present("Keywords") {
            println!("{:?}",desktop_file.keywords.unwrap_or_default());
        }
        if matches.is_present("StartupNotify") {
            println!("{:?}",desktop_file.startup_notify.unwrap_or_default());
        }
        if matches.is_present("StartupWMClass") {
            println!("{:?}",desktop_file.startup_wm_class.unwrap_or("".to_string()));
        }
        if matches.is_present("URL") {
            println!("{:?}",desktop_file.url.unwrap_or("".to_string()));
        }
        if matches.is_present("PrefersNonDefaultGPU") {
            println!("{:?}",desktop_file.prefers_non_default_gpu.unwrap_or_default());
        }
    }

    // icon-theme ARGS
    if let Some(matches) = matches.subcommand_matches("icon-theme") {
    // Read the desktop file
        let filename = matches.value_of("FILE").unwrap();
        let icon_file = icon_theme::IconTheme::new(filename.to_string());
        if matches.is_present("Name") {
            println!("{:?}",icon_file.name.unwrap_or_default());
        }
        if matches.is_present("Comment") {
            println!("{:?}",icon_file.comment.unwrap_or_default());
        }
        if matches.is_present("Inherits") {
            println!("{:?}",icon_file.inherits.unwrap_or_default());
        }
        if matches.is_present("Directories") {
            println!("{:?}",icon_file.directories.unwrap_or_default());
        }
        if matches.is_present("ScaledDirectories") {
            println!("{:?}",icon_file.scaled_directories.unwrap_or_default());
        }
        if matches.is_present("Hidden") {
            println!("{:?}",icon_file.hidden.unwrap_or_default());
        }
        if matches.is_present("Example") {
            println!("{:?}",icon_file.example.unwrap_or_default());
        }
    }
}
