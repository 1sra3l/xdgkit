/* 
xdgkit
Rusified in 2021 Copyright Israel Dahl. All rights reserved.

       /VVVV\
     /V      V\
   /V          V\
  /      0 0     \
  \|\|\</\/\>/|/|/
       \_/\_/

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License version 2 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

*/
mod desktop_entry;
mod basedir;
mod utils;
use crate::desktop_entry::DesktopEntry;
use crate::basedir::home;
//use crate::desktop_entry::DesktopType;
extern crate clap;
use clap::{App, load_yaml};
use crate::desktop_entry::display_desktop_type;
//#[cfg(feature = "yaml")]

fn main() {
	let yaml = load_yaml!("cli.yaml");
	let _home = basedir::home();
    let matches = App::from(yaml).get_matches();

// basedir ARGS
if let Some(matches) = matches.subcommand_matches("basedir") {
        if matches.is_present("Home") {
            println!("{}",basedir::home().unwrap_or("".to_string()));
        }
        if matches.is_present("Menus") {
            println!("{}",basedir::menu().unwrap_or("".to_string()));
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
		let desktop_file = DesktopEntry::new(filename.to_string());
		
        if matches.is_present("Type") {
            println!("{:?}",display_desktop_type(desktop_file.desktop_type));
        }
        if matches.is_present("Version") {
            println!("{:?}",desktop_file.version);
        }
        if matches.is_present("Name") {
            println!("{:?}",desktop_file.name);
        }
        if matches.is_present("GenericName") {
            println!("{:?}",desktop_file.generic_name);
        }
        if matches.is_present("NoDisplay") {
            println!("{:?}",desktop_file.no_display);
        }
        if matches.is_present("Comment") {
            println!("{:?}",desktop_file.comment);
        }
        if matches.is_present("Icon") {
            println!("{:?}",desktop_file.icon);
        }
        if matches.is_present("Hidden") {
            println!("{:?}",desktop_file.hidden);
        }
        if matches.is_present("OnlyShowIn") {
            println!("{:?}",desktop_file.only_show_in);
        }
        if matches.is_present("NotShowIn") {
            println!("{:?}",desktop_file.not_show_in);
        }
        if matches.is_present("DBusActivatable") {
            println!("{:?}",desktop_file.dbus_activatable);
        }
        if matches.is_present("TryExec") {
            println!("{:?}",desktop_file.try_exec);
        }
        if matches.is_present("Exec") {
            println!("{:?}",desktop_file.exec);
        }
        if matches.is_present("Path") {
            println!("{:?}",desktop_file.path);
        }
        if matches.is_present("Terminal") {
            println!("{:?}",desktop_file.terminal);
        }
        if matches.is_present("Actions") {
            println!("{:?}",desktop_file.actions);
        }
        if matches.is_present("MimeType") {
            println!("{:?}",desktop_file.mime_type);
        }
        if matches.is_present("Categories") {
            println!("{:?}",desktop_file.categories);
        }
        if matches.is_present("Implements") {
            println!("{:?}",desktop_file.implements);
        }
        if matches.is_present("Keywords") {
            println!("{:?}",desktop_file.keywords);
        }
        if matches.is_present("StartupNotify") {
            println!("{:?}",desktop_file.startup_notify);
        }
        if matches.is_present("StartupWMClass") {
            println!("{:?}",desktop_file.startup_wm_class);
        }
        if matches.is_present("URL") {
            println!("{:?}",desktop_file.url);
        }
        if matches.is_present("PrefersNonDefaultGPU") {
            println!("{:?}",desktop_file.prefers_non_default_gpu);
        }
    }
}
