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

use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod basedir;
mod categories;
mod desktop_entry;
mod desktop_menu;
mod icon_finder;
mod icon_theme;
mod menu;
mod user_dirs;
mod utils;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Basedir {
        #[arg(long)]
        home: bool,
        #[arg(long)]
        session_menu: bool,
        #[arg(long)]
        menus: bool,
        #[arg(long)]
        menus_merged: bool,
        #[arg(long)]
        applications: bool,
        #[arg(long)]
        desktop_directories: bool,
        #[arg(long)]
        autostart: bool,
        #[arg(long)]
        icon: bool,
        #[arg(long)]
        trash: bool,
        #[arg(long)]
        data_home: bool,
        #[arg(long)]
        cache_home: bool,
        #[arg(long)]
        config_home: bool,
        #[arg(long)]
        data: bool,
        #[arg(long)]
        config: bool,
    },
    Findicon {
        icon_name: String,
    },
    DesktopEntry {
        file: String,
        #[arg(long)]
        r#type: bool,
        #[arg(long)]
        version: bool,
        #[arg(long)]
        name: bool,
        #[arg(long)]
        generic_name: bool,
        #[arg(long)]
        no_display: bool,
        #[arg(long)]
        comment: bool,
        #[arg(long)]
        icon: bool,
        #[arg(long)]
        hidden: bool,
        #[arg(long)]
        only_show_in: bool,
        #[arg(long)]
        not_show_in: bool,
        #[arg(long)]
        dbus_activatable: bool,
        #[arg(long)]
        try_exec: bool,
        #[arg(long)]
        exec: bool,
        #[arg(long)]
        path: bool,
        #[arg(long)]
        terminal: bool,
        #[arg(long)]
        actions: bool,
        #[arg(long)]
        mime_type: bool,
        #[arg(long)]
        categories: bool,
        #[arg(long)]
        implements: bool,
        #[arg(long)]
        keywords: bool,
        #[arg(long)]
        startup_notify: bool,
        #[arg(long)]
        startup_wm_class: bool,
        #[arg(long)]
        url: bool,
        #[arg(long)]
        prefers_non_default_gpu: bool,
    },
    IconTheme {
        file: String,
        #[arg(long)]
        name: bool,
        #[arg(long)]
        comment: bool,
        #[arg(long)]
        inherits: bool,
        #[arg(long)]
        directories: bool,
        #[arg(long)]
        scaled_directories: bool,
        #[arg(long)]
        hidden: bool,
        #[arg(long)]
        example: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Basedir {
            home,
            session_menu,
            menus,
            menus_merged,
            applications,
            desktop_directories,
            autostart,
            icon,
            trash,
            data_home,
            cache_home,
            config_home,
            data,
            config,
        }) => {
            if home {
                println!("{}", basedir::home().unwrap_or_default());
            }
            if session_menu {
                println!("{}", basedir::session_menu_file().unwrap_or_default());
            }
            if menus {
                println!("{}", basedir::menu().unwrap_or_default());
            }
            if menus_merged {
                println!("{}", basedir::menu_merged().unwrap_or_default());
            }
            if applications {
                println!("{}", basedir::applications().unwrap_or_default());
            }
            if desktop_directories {
                println!("{}", basedir::desktop_directories().unwrap_or_default());
            }
            if autostart {
                println!("{}", basedir::autostart().unwrap_or_default());
            }
            if icon {
                println!("{}", basedir::icon_dirs().unwrap_or_default());
            }
            if trash {
                println!("{}", basedir::trash().unwrap_or_default());
            }
            if data_home {
                println!("{}", basedir::data_home().unwrap_or_default());
            }
            if cache_home {
                println!("{}", basedir::cache_home().unwrap_or_default());
            }
            if config_home {
                println!("{}", basedir::config_home().unwrap_or_default());
            }
            if data {
                println!("{}", basedir::data_dirs().unwrap_or_default());
            }
            if config {
                println!("{}", basedir::config_dirs().unwrap_or_default());
            }
        }
        Some(Commands::Findicon { icon_name }) => {
            let icon = icon_finder::find_icon(icon_name, 48, 1).unwrap_or_else(PathBuf::new);
            println!("{:?}", icon);
        }
        Some(Commands::DesktopEntry {
            file,
            r#type,
            version,
            name,
            generic_name,
            no_display,
            comment,
            icon,
            hidden,
            only_show_in,
            not_show_in,
            dbus_activatable,
            try_exec,
            exec,
            path,
            terminal,
            actions,
            mime_type,
            categories,
            implements,
            keywords,
            startup_notify,
            startup_wm_class,
            url,
            prefers_non_default_gpu,
        }) => {
            let desktop_file = desktop_entry::DesktopEntry::new(file);
            if r#type {
                println!("{:?}", desktop_file.xdg_type.to_string());
            }
            if version {
                println!("{:?}", desktop_file.version);
            }
            if name {
                println!("{:?}", desktop_file.name.unwrap_or_default());
            }
            if generic_name {
                println!("{:?}", desktop_file.generic_name.unwrap_or_default());
            }
            if no_display {
                println!("{:?}", desktop_file.no_display.unwrap_or_default());
            }
            if comment {
                println!("{:?}", desktop_file.comment.unwrap_or_default());
            }
            if icon {
                println!("{:?}", desktop_file.icon.unwrap_or_default());
            }
            if hidden {
                println!("{:?}", desktop_file.hidden.unwrap_or_default());
            }
            if only_show_in {
                for item in desktop_file.only_show_in.unwrap_or_default() {
                    print!("{:?};", item);
                }
                println!();
            }
            if not_show_in {
                for item in desktop_file.not_show_in.unwrap_or_default() {
                    print!("{:?};", item);
                }
                println!();
            }
            if dbus_activatable {
                println!("{:?}", desktop_file.dbus_activatable.unwrap_or_default());
            }
            if try_exec {
                println!("{:?}", desktop_file.try_exec.unwrap_or_default());
            }
            if exec {
                println!("{:?}", desktop_file.exec.unwrap_or_default());
            }
            if path {
                println!("{:?}", desktop_file.path.unwrap_or_default());
            }
            if terminal {
                println!("{:?}", desktop_file.terminal.unwrap_or_default());
            }
            if actions {
                println!("{:?}", desktop_file.actions.unwrap_or_default());
            }
            if mime_type {
                println!("{:?}", desktop_file.mime_type.unwrap_or_default());
            }
            if categories {
                println!("{:?}", desktop_file.categories.unwrap_or_default());
            }
            if implements {
                println!("{:?}", desktop_file.implements.unwrap_or_default());
            }
            if keywords {
                println!("{:?}", desktop_file.keywords.unwrap_or_default());
            }
            if startup_notify {
                println!("{:?}", desktop_file.startup_notify.unwrap_or_default());
            }
            if startup_wm_class {
                println!("{:?}", desktop_file.startup_wm_class.unwrap_or_default());
            }
            if url {
                println!("{:?}", desktop_file.url.unwrap_or_default());
            }
            if prefers_non_default_gpu {
                println!(
                    "{:?}",
                    desktop_file.prefers_non_default_gpu.unwrap_or_default()
                );
            }
        }
        Some(Commands::IconTheme {
            file,
            name,
            comment,
            inherits,
            directories,
            scaled_directories,
            hidden,
            example,
        }) => {
            let icon_file = icon_theme::IconTheme::new(file);
            if name {
                println!("{:?}", icon_file.name.unwrap_or_default());
            }
            if comment {
                println!("{:?}", icon_file.comment.unwrap_or_default());
            }
            if inherits {
                println!("{:?}", icon_file.inherits.unwrap_or_default());
            }
            if directories {
                println!("{:?}", icon_file.directories.unwrap_or_default());
            }
            if scaled_directories {
                println!("{:?}", icon_file.scaled_directories.unwrap_or_default());
            }
            if hidden {
                println!("{:?}", icon_file.hidden.unwrap_or_default());
            }
            if example {
                println!("{:?}", icon_file.example.unwrap_or_default());
            }
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
        }
    }
}
