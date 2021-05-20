/* 
desktop_entry.rs
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

extern crate tini;
use tini::Ini;
use std::path::Path;

use crate::utils::get_language;

/// This specification defines 3 types of desktop entries:
/// * Application (type 1),
/// * Link (type 2)
/// * Directory (type 3)
///
/// To allow the addition of new types in the future, implementations should ignore desktop entries with an "unknown" type.
#[derive(Debug)]
pub enum DesktopType {
	Application,
	Link,
	Directory,
	/// Ignore 'Unknown' type in your program, it really just means it is anything other than the above known types
	Unknown,
}

fn to_bool(value:Option<String>)->Option<bool> {
	if value.is_none() {
		return None
	}

	let mut val = value.unwrap();
	val.make_ascii_lowercase();
	if val == "true" {
		return Some(true)
	}
	Some(false)
}

/// This function converts a Option<String> to a DesktopType
pub fn convert_desktop_type(desktop_type_option:Option<String>)->DesktopType {
	if desktop_type_option.is_some() {
		let dt = desktop_type_option.unwrap();
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
pub fn display_desktop_type(dt:DesktopType)->Option<String> {
	match dt {
		DesktopType::Application => return Some(String::from("Application")),
		DesktopType::Link => return Some(String::from("Link")),
		DesktopType::Directory => return Some(String::from("Directory")),
		_=> return None,
	}
}

#[derive(Debug)]
/// # Recognized desktop entry keys
///
/// [XDG specs](https://specifications.freedesktop.org)
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
/// type is a reserved word in rust, so it is called 'desktop_type'
///
/// I copied and pasted the website here and made things rustified
///
/// No camelcase inside structs, all lowercase/underscore version of CamelCase
///
/// Comments are just because I didn't delete the rest of the text from the webpage
pub struct DesktopEntry {
//type is a reserved word in rust and other languages so we will use desktop_type
	/// This specification defines 3 types of desktop entries: Application (type 1), Link (type 2) and Directory (type 3). To allow the addition of new types in the future, implementations should ignore desktop entries with an unknown type. 	Stringing	YES
	pub desktop_type:DesktopType,
	/// Version of the Desktop Entry Specification that the desktop entry conforms with. Entries that confirm with this version of the specification should use 1.5. Note that the version field is not required to be present. 	Stringing	NO	1-3
	pub version:f32,
	/// Specific name of the application, for example "Mozilla". 	localeStringing	YES	1-3
	pub name:Option<String>,
	/// Generic name of the application, for example "Web Browser". 	localeStringing	NO	1-3
	pub generic_name:Option<String>,
	/// NoDisplay means "this application exists, but don't display it in the menus". This can be useful to e.g. associate this application with MIME types, so that it gets launched from a file manager (or other apps), without having a menu entry for it (there are tons of good reasons for this, including e.g. the netscape -remote, or kfmclient openURL kind of stuff). 	boolean	NO	1-3
	pub no_display:Option<bool>,
	/// Tooltip for the entry, for example "View sites on the Internet". The value should not be redundant with the values of Name and GenericName. 	localeStringing	NO	1-3
	pub comment:Option<String>,
	/// Icon to display in file manager, menus, etc. If the name is an absolute path, the given file will be used. If the name is not an absolute path, the algorithm described in the Icon Theme Specification will be used to locate the icon. 	iconStringing	NO	1-3
	pub icon:Option<String>,
	/// Hidden should have been called Deleted. It means the user deleted (at his level) something that was present (at an upper level, e.g. in the system dirs). It's Stringictly equivalent to the .desktop file not existing at all, as far as that user is concerned. This can also be used to "uninstall" existing files (e.g. due to a renaming) - by letting make install install a file with Hidden=true in it. 	boolean	NO	1-3
	pub hidden:Option<bool>,
	/// A list of Strings identifying the desktop environments that should display/not display a given desktop entry.
/// By default, a desktop file should be shown, unless an OnlyShowIn key is present, in which case, the default is for the file not to be shown.
/// If $XDG_CURRENT_DESKTOP is set then it contains a colon-separated list of Strings. In order, each Stringing is considered. If a matching entry is found in OnlyShowIn then the desktop file is shown. If an entry is found in NotShowIn then the desktop file is not shown. If none of the Strings match then the default action is taken (as above).
/// $XDG_CURRENT_DESKTOP should have been set by the login manager, according to the value of the DesktopNames found in the session file. The entry in the session file has multiple values separated in the usual way: with a semicolon.
/// The same desktop name may not appear in both OnlyShowIn and NotShowIn of a group. Stringing(s)	NO	1-3
	pub only_show_in:Option<Vec<String>>,
	pub not_show_in:Option<Vec<String>>,
	/// A boolean value specifying if D-Bus activation is supported for this application. If this key is missing, the default value is false. If the value is true then implementations should ignore the Exec key and send a D-Bus message to launch the application. See D-Bus Activation for more information on how this works. Applications should still include Exec= lines in their desktop files for compatibility with implementations that do not understand the DBusActivatable key. 	boolean	NO
	pub dbus_activatable:Option<bool>,
	/// Path to an executable file on disk used to determine if the program is actually installed. If the path is not an absolute path, the file is looked up in the $PATH environment variable. If the file is not present or if it is not executable, the entry may be ignored (not be used in menus, for example). 	Stringing	NO	1
	pub try_exec:Option<String>,
	/// Program to execute, possibly with arguments. See the Exec key for details on how this key works. The Exec key is required if DBusActivatable is not set to true. Even if DBusActivatable is true, Exec should be specified for compatibility with implementations that do not understand DBusActivatable. 	Stringing	NO	1
	pub exec:Option<String>,
	/// If entry is of type Application, the working directory to run the program in. 	Stringing	NO	1
	pub path:Option<String>,
	/// Whether the program runs in a terminal window. 	boolean	NO	1
	pub terminal:Option<bool>,
	/// Identifiers for application actions. This can be used to tell the application to make a specific action, different from the default behavior. The Application actions section describes how actions work. 	Stringing(s)	NO	1
	pub actions:Option<Vec<String>>,
	/// The MIME type(s) supported by this application. 	Stringing(s)	NO	1
	pub mime_type:Option<Vec<String>>,
	/// Categories in which the entry should be shown in a menu (for possible values see the Desktop Menu Specification). 	Stringing(s)	NO	1
	pub categories:Option<Vec<String>>,
	/// A list of interfaces that this application implements. By default, a desktop file implements no interfaces. See Interfaces for more information on how this works. 	Stringing(s)	NO
	pub implements:Option<Vec<String>>,
	/// A list of Strings which may be used in addition to other metadata to describe this entry. This can be useful e.g. to facilitate searching through entries. The values are not meant for display, and should not be redundant with the values of Name or GenericName. 	localeStringing(s)	NO	1
	pub keywords:Option<Vec<String>>,
	/// If true, it is KNOWN that the application will send a "remove" message when started with the DESKTOP_STARTUP_ID environment variable set. If false, it is KNOWN that the application does not work with startup notification at all (does not shown any window, breaks even when using StartupWMClass, etc.). If absent, a reasonable handling is up to implementations (assuming false, using StartupWMClass, etc.). (See the Startup Notification Protocol Specification for more details). 	boolean	NO	1
	pub startup_notify:Option<bool>,
	/// If specified, it is known that the application will map at least one window with the given Stringing as its WM class or WM name hint (see the Startup Notification Protocol Specification for more details). 	Stringing	NO	1
	pub startup_wm_class:Option<String>,
	/// If entry is Link type, the URL to access. 	Stringing	YES	2
	pub url:Option<String>,
	/// If true, the application prefers to be run on a more powerful discrete GPU if available, which we describe as “a GPU other than the default one” in this spec to avoid the need to define what a discrete GPU is and in which cases it might be considered more powerful than the default GPU. This key is only a hint and support might not be present depending on the implementation. 	boolean	NO	1
	pub prefers_non_default_gpu:Option<bool>,
}

///  Implementations
impl DesktopEntry {
	/// Creates a blank entry with a specified type
	pub fn empty(initial_type:DesktopType)->Self where Self:Sized {

		DesktopEntry {
			desktop_type:initial_type,
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
    pub fn new(file_name:String)->Self where Self:Sized {
		let test_ini = Ini::from_file(&file_name);
		if test_ini.is_err() {
			//TODO println!("ERROR!!!{}",file_name);
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
		DesktopEntry {
			desktop_type:convert_desktop_type(dt),
			version:ver.unwrap().parse::<f32>().ok().unwrap(),
			name:local_name,
			generic_name:local_gen,
			no_display:to_bool(nd),
			comment:com,
			icon:ic,
			hidden:to_bool(hid),
			only_show_in:only,
			not_show_in:not,
			dbus_activatable:to_bool(dbus),
			try_exec:tex,
			exec:ex,
			path:pth,
			terminal:to_bool(term),
			actions:act,
			mime_type:mime,
			categories:cat,
			implements:imp,
			keywords:local_key,
			startup_notify:to_bool(start),
			startup_wm_class:wm,
			url:ur,
			prefers_non_default_gpu:to_bool(gpu),
		}
    }
}
