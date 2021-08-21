// basedir.rs
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

//TODO learn how to namespace this!!!
use std::env;
use std::env::VarError;
use std::path::Path;

/// $HOME
#[allow(dead_code)]
pub fn home()->Result<String, VarError> {
    env::var("HOME")
}

/// the main 'getter' function
/// This basically checks env::var(whatever) and returns the right results
fn var_getter(env_var:&str, directory:String)->Result<String, VarError> {
    match env::var(env_var) {
        Ok(val)=> return Ok(val),
        Err(_e)=> {
            // make sure the direcory exists!
            if !Path::new(directory.as_str()).is_dir() {
                return Err(VarError::NotPresent)
            }
            return Ok(String::from(directory))
        },
    }
}

/// Redundant $HOME-to-string-path code lives here
#[allow(dead_code)]
pub fn prepare_home(directory:&str)->String {
    let conf_home:Result<String, VarError> = home();
    if conf_home.is_err() {
        return String::from("");
    }
    let mut path:String = conf_home.ok().unwrap();
    path.push_str(directory);
    path
}

/// Convert ';' list `Result<String, VarError>` into Vec<String>
#[allow(dead_code)]
pub fn convert_to_vec(list:Result<String, VarError>)-> Vec<String> {
    let mut result:Vec<String> = vec![];
    let input:String = list.unwrap_or("".to_string());
    if input.is_empty() {
        return result
    }
    let str_res:Vec<&str> = input.split(':').collect();
    for item in str_res {
      let s_item:String = item.to_owned();
      result.push(s_item);
    }
    return result
}

/// $XDG_DATA_HOME
#[allow(dead_code)]
pub fn data_home()->Result<String, VarError> {
//$HOME/.local/share
    let env_var = "XDG_DATA_HOME";
    let dir = prepare_home("/.local/share");
    var_getter(env_var, dir)
}

/// $XDG_CACHE_HOME
#[allow(dead_code)]
pub fn cache_home()->Result<String, VarError> {
    let env_var = "XDG_CACHE_HOME";
    let dir = prepare_home("/.cache");
    var_getter(env_var, dir)
}

/// $XDG_CONFIG_HOME
#[allow(dead_code)]
pub fn config_home()->Result<String, VarError> {
    //$HOME/.config
    let env_var = "XDG_CONFIG_HOME";
    let dir = prepare_home("/.config");
    var_getter(env_var, dir)
}

/// $XDG_DATA_DIRS
#[allow(dead_code)]
pub fn data_dirs()->Result<String, VarError> {
    let env_var = "XDG_DATA_DIRS";
    let dir = String::from("/usr/local/share/:/usr/share/");
    var_getter(env_var, dir)
}

/// $XDG_CONFIG_DIRS
#[allow(dead_code)]
pub fn config_dirs()->Result<String, VarError> {
    let env_var = "XDG_CONFIG_DIRS";
    let dir = String::from("/etc/xdg");
    var_getter(env_var, dir)
}

/// user TRASH DIRECTORY
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn loop_data_dirs(directory:String)->Result<String, VarError> {
    let mut result = String::from("");
    let result2 = data_dirs();
    let mut fail = false;
    if result2.is_ok() {
        let res_list = result2.unwrap().to_owned();
        for item in res_list.split(':') {
            let mut tmp_path:String = item.to_owned();
            tmp_path.push_str(directory.as_str());
            if Path::new(tmp_path.as_str()).is_dir() {
                tmp_path.push_str(":");
                result.push_str(tmp_path.as_str());
            }
        }
    }
    else { fail = true;}
    let home_dir = data_home();
    if home_dir.is_ok() {
        let mut d = home_dir.unwrap().to_owned();
        d.push_str(directory.as_str());
        if Path::new(d.as_str()).is_dir() {
            d.push_str(":");
            result.push_str(d.as_str());
            fail = false;
        }
    }
    else {fail = true;}
    if fail {return Err(VarError::NotPresent)}
    Ok(result)
}
pub fn data_dirs_vec(directory:String)->Vec<String> {
    let mut result:Vec<String> = vec![];
    let result2 = data_dirs();
    if result2.is_ok() {
        let res_list = result2.unwrap().to_owned();
        for item in res_list.split(':') {
            let mut tmp_path:String = item.to_owned();
            tmp_path.push_str(directory.as_str());
            if Path::new(tmp_path.as_str()).exists() {
                result.push(tmp_path.to_owned());
            }
        }
    }
    let home_dir = data_home();
    if home_dir.is_ok() {
        let mut d = home_dir.unwrap().to_owned();
        d.push_str(directory.as_str());
        if Path::new(d.as_str()).exists() {
            result.push(d);
        }
    }
    result.sort();
    result.dedup();
    result
}
/// The /menu directory
#[allow(dead_code)]
pub fn menu()->Result<String, VarError> {
    let result = config_dirs();
    let mut retval:String = "".to_string();
    if result.is_ok() {
        retval = result.unwrap().to_owned();
        retval.push_str("/menu");
    }
    if Path::new(retval.as_str()).is_dir() {
        return Ok(retval)
    }
    return Err(VarError::NotPresent)
}
/// the /menu/applications-merged directory
#[allow(dead_code)]
pub fn menu_merged()->Result<String, VarError> {
    let result = config_dirs();
    let mut retval:String = "".to_string();
    if result.is_ok() {
        retval = result.unwrap().to_owned();
        retval.push_str("/menu/applications-merged");
    }
    if Path::new(retval.as_str()).is_dir() {
        return Ok(retval)
    }
    return Err(VarError::NotPresent)
}

/// the /applications directories
#[allow(dead_code)]
pub fn applications()->Result<String, VarError> {
    loop_data_dirs("/applications".to_string())
}
/// DESKTOP DIRECTORIES, DIRECTORIES
#[allow(dead_code)]
pub fn desktop_directories()->Result<String, VarError> {
    loop_data_dirs("/desktop-directories".to_string())
}

/// 
#[allow(dead_code)]
pub fn autostart()->Result<String, VarError> {
    loop_data_dirs("/autostart".to_string())
}

/// ICON DIRECTORIES
#[allow(dead_code)]
pub fn icon_dirs()->Result<String, VarError> {
    let result = home();
    let mut retval:String = "".to_string();
    if result.is_ok() {
        retval = result.unwrap().to_owned();
        retval.push_str("./icons:");
    }
    let result2 = loop_data_dirs("/icons".to_string());
    if result2.is_ok() {
        let res_list = result2.unwrap().to_owned();
        retval.push_str(&res_list);
    }
    retval.push_str("/usr/share/pixmaps:");
    Ok(retval)
}
/// Vector of Icon directories
pub fn icon_dirs_vector()->Vec<String> {
    // make our directory of icons
    let mut directory_vec:Vec<String> = data_dirs_vec("/icons".to_string());
    let local_icons_dir = home();
    let mut local_icons:String = "".to_string();
    if local_icons_dir.is_ok() {
        local_icons = local_icons_dir.unwrap().to_owned();
        local_icons.push_str("./icons");
        directory_vec.push(local_icons.to_owned());
    }
    directory_vec.push("/usr/share/pixmaps".to_string());
    directory_vec.sort();
    directory_vec.dedup();
    directory_vec
}
