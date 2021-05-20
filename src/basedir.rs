/* 
basedir.rs
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
//TODO learn how to namespace this!!!
use std::env;
use std::env::VarError;

/// $HOME
pub fn home()->Result<String, VarError> {
	env::var("HOME")
}

/*
the main 'getter' function
This basically checks env::var(whatever) and returns the right results
*/
fn var_getter(env_var:&str, directory:String)->Result<String, VarError> {
	match env::var(env_var) {
		Ok(val)=> return Ok(val),
		Err(_e)=> {
			if directory.is_empty() {
				return Err(VarError::NotPresent)
			}
			return Ok(String::from(directory))
		},
	}
}

/// Redundant $HOME-to-string-path code lives here
pub fn prepare_home(directory:&str)->String {
	let conf_home:Result<String, VarError> = home();
	if conf_home.is_err() {
		return String::from("");
	}
	let mut path:String = conf_home.ok().unwrap();
	path.push_str(directory);
	path
}

/// $XDG_DATA_HOME
pub fn data_home()->Result<String, VarError> {
//$HOME/.local/share
	let env_var = "XDG_DATA_HOME";
	let dir = prepare_home("/.local/share");
	var_getter(env_var, dir)
}

/// $XDG_CACHE_HOME
pub fn cache_home()->Result<String, VarError> {
	let env_var = "XDG_CACHE_HOME";
	let dir = prepare_home("/.cache");
	var_getter(env_var, dir)
}

/// $XDG_CONFIG_HOME
pub fn config_home()->Result<String, VarError> {
	//$HOME/.config
	let env_var = "XDG_CONFIG_HOME";
	let dir = prepare_home("/.config");
	var_getter(env_var, dir)
}

/// $XDG_DATA_DIRS
pub fn data_dirs()->Result<String, VarError> {
	let env_var = "XDG_DATA_DIRS";
	let dir = String::from("/usr/local/share/:/usr/share/");
	var_getter(env_var, dir)
}

/// $XDG_CONFIG_DIRS
pub fn config_dirs()->Result<String, VarError> {
	let env_var = "XDG_CONFIG_DIRS";
	let dir = String::from("/etc/xdg");
	var_getter(env_var, dir)
}

/// user TRASH DIRECTORY
pub fn trash()->Result<String, VarError> {
	let dh = data_home();
	if dh.is_ok() {
		let mut result = dh.unwrap().to_owned();
		result.push_str("/Trash");
		return Ok(result)
	}
	Ok(String::from(""))
}
/// loop xdg data dirs
pub fn loop_data_dirs(directory:String)->Result<String, VarError> {
	let mut result = String::from("");
	let result2 = data_dirs();
	if result2.is_ok() {
		let res_list = result2.unwrap().to_owned();
		for item in res_list.split(':') {
			let mut tmp_path:String = item.to_owned();
			tmp_path.push_str(directory.as_str());
			tmp_path.push_str(":");
			result.push_str(tmp_path.as_str());
		}
	}
	Ok(result)
}
/// MENU DIRECTORIES
pub fn menu()->Result<String, VarError> {
	loop_data_dirs("/menu".to_string())
}
/// APPLICATIONS DIRECTORIES
pub fn applications()->Result<String, VarError> {
	loop_data_dirs("/applications".to_string())
}
/// DESKTOP DIRECTORIES, DIRECTORIES
pub fn desktop_directories()->Result<String, VarError> {
	loop_data_dirs("/desktop-directories".to_string())
}

/// AUTOSTART DIRECTORIES
pub fn autostart()->Result<String, VarError> {
	let ch = config_home();
	let mut result = String::from("");
	if ch.is_ok() {
		result = ch.unwrap().to_owned();
		result.push_str("/autostart");
	}
	let cd = config_dirs();
	if cd.is_ok() {
		if !result.is_empty() {
			result.push_str(":");
		}
		let res2 = cd.unwrap().to_owned();
		result.push_str(res2.as_str());
		result.push_str("/autostart");
	}
	Ok(result)
}

/// ICON DIRECTORIES
pub fn icon_dirs()->Result<String, VarError> {
	let result = home();
	let mut retval:String = "".to_string();
	if result.is_ok() {
		retval = result.unwrap().to_owned();
		retval.push_str("./icons:");
	}
	let result2 = data_dirs();
	if result2.is_ok() {
		let res_list = result2.unwrap().to_owned();
		for item in res_list.split(':') {
			let mut tmp_path:String = item.to_owned();
			tmp_path.push_str("/icons:");
			retval.push_str(tmp_path.as_str());
		}
	}
	retval.push_str("/usr/share/pixmaps:");
	Ok(retval)
}
