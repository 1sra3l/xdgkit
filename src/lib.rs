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

pub mod utils;
pub mod categories;
pub mod desktop_entry;
pub mod basedir;
pub mod icon_theme;
pub mod desktop_menu;

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::basedir::*;
    //use crate::desktop_entry::*;
    //use crate::categories::*;
//basedir.rs
    #[test]
    fn basedir_test(){
        if cfg!(target_os = "linux") {
            let res = data_dirs();
            let result2 = res.unwrap().to_owned();
            let vec:Vec<String> = convert_to_vec(data_dirs());
            assert!(!result2.is_empty() && !vec.is_empty());
        }
    }
// desktop_entry.rs
    #[test]
    fn desktop_entry_test_good() {
        //TODO make desktop file to test
    }
    #[test]
    fn desktop_entry_test_bad() {
        //TODO make bad desktop file to test
    }
// utils.rs
    #[test]
    fn utils_converter() {
        let true_bool:String = "true".to_string();
        assert_eq!(Some(true), to_bool(Some(true_bool)));
        let one_int:String = "1".to_string();
        assert_eq!(Some(1), to_int(Some(one_int)));
        if cfg!(target_os = "linux") {
            let lang = get_language();
            assert_ne!(None, lang);
        }
    }
}
